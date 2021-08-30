#[macro_use]
mod macros;

use slos_filesystem::impls::PakFilesystem;

fn _construct_pakfs() -> PakFilesystem<'static> {
	const EXAMPLE_PAK: &[u8] = include_bytes!("data/pak/test.pak");
	PakFilesystem::from_bytes("pakfs-test", EXAMPLE_PAK).unwrap()
}

fs_mount_tests!(PakFilesystem, _construct_pakfs);
fs_read_tests!(PakFilesystem, _construct_pakfs);
