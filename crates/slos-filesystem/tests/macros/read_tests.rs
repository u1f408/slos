#[macro_export(local_inner_macros)]
macro_rules! fs_read_tests {
	($t:ty, $gent:expr, $genb:expr) => {
		mod fs_read_tests {
			use super::*;

			#[test_env_log::test]
			fn readdir_root() {
				let (mut base, fs) = $crate::__construct!($t, $gent, $genb);
				base.mount(&[], Box::new(fs)).unwrap();
				let fs_node = base.node_at_path(&[]).unwrap();
				let fs_dir = fs_node.try_directory().unwrap();

				for (_index, node) in (0..).zip(fs_dir.readdir().unwrap().iter_mut()) {
					std::assert!(node.try_file().is_some() || node.try_directory().is_some());
					std::assert!(node.name() != "");
				}
			}

			#[test_env_log::test]
			fn file_read_each() {
				let (mut base, fs) = $crate::__construct!($t, $gent, $genb);
				base.mount(&[], Box::new(fs)).unwrap();
				let fs_node = base.node_at_path(&[]).unwrap();
				let fs_dir = fs_node.try_directory().unwrap();

				let mut has_file: bool = false;
				for (_index, node) in (0..).zip(fs_dir.readdir().unwrap().iter_mut()) {
					if let Some(file) = node.try_file() {
						has_file = true;
						let handle = file.open().unwrap();
						let _ = handle.raw_read(0, None).unwrap();
					}
				}

				std::assert!(has_file);
			}

			#[test_env_log::test]
			fn dir_readdir_each() {
				let (mut base, fs) = $crate::__construct!($t, $gent, $genb);
				base.mount(&[], Box::new(fs)).unwrap();
				let fs_node = base.node_at_path(&[]).unwrap();
				let fs_dir = fs_node.try_directory().unwrap();

				let mut has_dir: bool = false;
				for (_index, node) in (0..).zip(fs_dir.readdir().unwrap().iter_mut()) {
					if let Some(dir) = node.try_directory() {
						has_dir = true;
						std::assert!(dir.readdir().is_ok());
					}
				}

				std::assert!(has_dir);
			}
		}
	};

	($t:ty, $gent:expr) => {
		fs_read_tests!($t, $gent, slos_filesystem::FilesystemBase::new);
	};
}
