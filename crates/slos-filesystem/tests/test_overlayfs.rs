#[macro_use]
mod macros;

use slos_filesystem::impls::OverlayFilesystem;

fn _construct_overlayfs() -> OverlayFilesystem<'static, 'static, 'static> {
	use slos_filesystem::impls::{PakFilesystem, SimpleMemoryFilesystem};
	use slos_filesystem::FsRoot;
	use slos_helpers::UnsafeContainer;

	const EXAMPLE_PAK: &[u8] = include_bytes!("data/pak/test.pak");

	lazy_static::lazy_static! {
		static ref MEMORY_FS: UnsafeContainer<SimpleMemoryFilesystem> =
			UnsafeContainer::new(SimpleMemoryFilesystem::new());
		static ref BASE_FS: UnsafeContainer<PakFilesystem<'static>> =
			UnsafeContainer::new(PakFilesystem::from_bytes("pakfs-test", EXAMPLE_PAK).unwrap());
	}

	MEMORY_FS.replace(SimpleMemoryFilesystem::new());
	BASE_FS.replace(PakFilesystem::from_bytes("pakfs-test", EXAMPLE_PAK).unwrap());

	OverlayFilesystem::new(
		"overlayfs-test",
		BASE_FS.get() as &mut dyn FsRoot,
		MEMORY_FS.get() as &mut dyn FsRoot,
	)
}

fs_mount_tests!(OverlayFilesystem, _construct_overlayfs);
fs_read_tests!(OverlayFilesystem, _construct_overlayfs);
fs_write_tests!(OverlayFilesystem, _construct_overlayfs);
