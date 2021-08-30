use lazy_static::lazy_static;
use test_env_log::test;

use slos_filesystem::impls::OverlayFilesystem;
use slos_filesystem::impls::{PakFilesystem, SimpleMemoryFilesystem};
use slos_filesystem::{FilesystemBase, FsReadDir, FsRoot};
use slos_helpers::UnsafeContainer;

lazy_static! {
	static ref MEMORY_FS: UnsafeContainer<SimpleMemoryFilesystem> =
		UnsafeContainer::new(SimpleMemoryFilesystem::new());
	static ref BASE_FS: UnsafeContainer<PakFilesystem<'static>> = {
		const EXAMPLE_PAK: &[u8] = include_bytes!("data/pak/test.pak");
		let basefs = PakFilesystem::from_bytes("pakfs-test", EXAMPLE_PAK).unwrap();
		UnsafeContainer::new(basefs)
	};
}

fn _construct_overlayfs() -> OverlayFilesystem<'static, 'static, 'static> {
	MEMORY_FS.replace(SimpleMemoryFilesystem::new());

	OverlayFilesystem::new(
		"overlayfs-test",
		BASE_FS.get() as &mut dyn FsRoot,
		MEMORY_FS.get() as &mut dyn FsRoot,
	)
}

fn _construct_fsbase() -> FilesystemBase {
	FilesystemBase::new()
}

#[test]
fn test_root_readdir() {
	let mut overlayfs = _construct_overlayfs();
	for (index, node) in (0..).zip(overlayfs.readdir().unwrap().iter()) {
		assert_eq!(node.name(), &format!("{}.txt", index + 1));
	}
}

#[test]
fn test_mount_root() {
	let overlayfs = _construct_overlayfs();
	let mut base = _construct_fsbase();
	base.mount(&[], Box::new(overlayfs)).unwrap();

	let node = base.node_at_path(&[]).unwrap();
	assert_eq!(node.name(), "overlayfs-test");
}

#[test]
fn test_mount_subpath() {
	let overlayfs = _construct_overlayfs();
	let mut base = _construct_fsbase();
	base.mount(&["test"], Box::new(overlayfs)).unwrap();

	let node = base.node_at_path(&["test"]).unwrap();
	assert_eq!(node.name(), "overlayfs-test");
}

#[test]
fn test_mount_readdir() {
	let overlayfs = _construct_overlayfs();
	let mut base = _construct_fsbase();
	base.mount(&[], Box::new(overlayfs)).unwrap();

	let node = base.node_at_path(&[]).unwrap();
	let dir = node.try_directory().unwrap();
	for (index, node) in (0..).zip(dir.readdir().unwrap().iter()) {
		assert_eq!(node.name(), &format!("{}.txt", index + 1));
	}
}

#[test]
fn test_mount_file_read() {
	let overlayfs = _construct_overlayfs();
	let mut base = _construct_fsbase();
	base.mount(&[], Box::new(overlayfs)).unwrap();

	let node = base.node_at_path(&["3.txt"]).unwrap();
	let file = node.try_file().unwrap();
	let handle = file.open().unwrap();

	let result = handle.raw_read(0, None);
	assert!(result.is_ok());

	let result = result.unwrap();
	assert_eq!(result, b"hello world!\n");
}

#[test]
fn test_mount_file_touch_write() {
	let overlayfs = _construct_overlayfs();
	let mut base = _construct_fsbase();
	base.mount(&[], Box::new(overlayfs)).unwrap();

	let dirnode = base.node_at_path(&[]).unwrap();
	let dir = dirnode.try_directory().unwrap();
	let filenode = dir.touch("test-new-file").unwrap();
	let file = filenode.try_file().unwrap();
	let handle = file.open().unwrap();

	handle.raw_write(0, b"hello world!\n").unwrap();

	let readback = handle.raw_read(0, None).unwrap();
	assert_eq!(readback, b"hello world!\n");
}
