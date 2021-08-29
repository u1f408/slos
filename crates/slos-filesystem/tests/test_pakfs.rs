use slos_filesystem::{pakfs::PakFilesystem, FilesystemBase, FsReadDir};

fn _construct_pakfs() -> PakFilesystem<'static> {
	const EXAMPLE_PAK: &[u8] = include_bytes!("data/pak/test.pak");
	PakFilesystem::from_bytes("test", EXAMPLE_PAK).unwrap()
}

fn _construct_fsbase() -> FilesystemBase {
	FilesystemBase::new()
}

#[test]
fn test_mount_root() {
	let pakfs = _construct_pakfs();
	let mut base = _construct_fsbase();

	base.mount(&[], Box::new(pakfs)).unwrap();
	assert!(base.node_at_path(&[]).is_ok());
}

#[test]
fn test_mount_subpath() {
	let pakfs = _construct_pakfs();
	let mut base = _construct_fsbase();

	base.mount(&["test"], Box::new(pakfs)).unwrap();
	assert!(base.node_at_path(&["test"]).is_ok());
}

#[test]
fn test_root_readdir() {
	let mut pakfs = _construct_pakfs();
	for (index, node) in (0..).zip(pakfs.readdir().unwrap().iter()) {
		assert_eq!(node.inode(), index);
		assert_eq!(node.name(), &format!("{}.txt", index + 1));
	}
}

#[test]
fn test_file_read() {
	let mut pakfs = _construct_pakfs();
	let mut readdir = pakfs.readdir().unwrap();
	let node = readdir.iter_mut().next().unwrap();
	let file = node.try_file().unwrap();
	let handle = file.open().unwrap();

	let result = handle.raw_read(0, None);
	assert!(result.is_ok());

	let result = result.unwrap();
	assert_eq!(result, b"hello world!\n");
}
