#[macro_export(local_inner_macros)]
macro_rules! fs_mount_tests {
	($t:ty, $gent:expr, $genb:expr) => {
		mod fs_mount_tests {
			use super::*;
			use slos_filesystem::FsNode;

			#[test_env_log::test]
			fn mount_root() {
				let (mut base, fs) = $crate::__construct!($t, $gent, $genb);

				let fs_name = String::from((&fs as &dyn FsNode).name());
				base.mount(&[], Box::new(fs)).unwrap();

				let node = base.node_at_path(&[]).unwrap();
				std::assert_eq!(node.name(), fs_name);
			}

			#[test_env_log::test]
			fn mount_subdir() {
				let (mut base, fs) = $crate::__construct!($t, $gent, $genb);

				let fs_name = String::from((&fs as &dyn FsNode).name());
				base.mount(&["test"], Box::new(fs)).unwrap();

				let node = base.node_at_path(&["test"]).unwrap();
				std::assert_eq!(node.name(), fs_name);
			}
		}
	};

	($t:ty, $gent:expr) => {
		fs_mount_tests!($t, $gent, slos_filesystem::FilesystemBase::new);
	};
}
