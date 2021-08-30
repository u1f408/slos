#[macro_export(local_inner_macros)]
macro_rules! fs_write_tests {
	($t:ty, $gent:expr, $genb:expr) => {
		mod fs_write_tests {
			use super::*;

			#[test_env_log::test]
			fn file_touch_and_write() {
				let (mut base, fs) = $crate::__construct!($t, $gent, $genb);
				base.mount(&[], Box::new(fs)).unwrap();
				let fs_node = base.node_at_path(&[]).unwrap();
				let fs_dir = fs_node.try_directory().unwrap();

				let filenode = fs_dir.touch("test-new-file").unwrap();
				let file = filenode.try_file().unwrap();
				let handle = file.open().unwrap();

				handle.raw_write(0, b"hello world!\n").unwrap();

				let readback = handle.raw_read(0, None).unwrap();
				std::assert_eq!(readback, b"hello world!\n");
			}
		}
	};

	($t:ty, $gent:expr) => {
		fs_write_tests!($t, $gent, slos_filesystem::FilesystemBase::new);
	};
}
