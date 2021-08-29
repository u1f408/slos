use slos_filesystem::impls::OverlayFilesystem;
use slos_filesystem::{FilesystemBase, FsReadDir, FsRoot};
use slos_helpers::UnsafeContainer;

fn _construct_overlayfs() -> OverlayFilesystem<'static, 'static, 'static> {
	use lazy_static::lazy_static;
	use slos_filesystem::impls::{PakFilesystem, SimpleMemoryFilesystem};

	lazy_static! {
		static ref BASE_FS: UnsafeContainer<PakFilesystem<'static>> = {
			const EXAMPLE_PAK: &[u8] = include_bytes!("data/pak/test.pak");
			let basefs = PakFilesystem::from_bytes("pakfs-test", EXAMPLE_PAK).unwrap();
			UnsafeContainer::new(basefs)
		};
		static ref OVERLAY_FS: UnsafeContainer<SimpleMemoryFilesystem> =
			UnsafeContainer::new(SimpleMemoryFilesystem::new());
	}

	OverlayFilesystem::new(
		"overlayfs-test",
		BASE_FS.get() as &mut dyn FsRoot,
		OVERLAY_FS.get() as &mut dyn FsRoot,
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
