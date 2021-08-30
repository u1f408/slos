#[macro_export]
#[doc(hidden)]
macro_rules! __construct {
	($t:ty, $gent:expr, $genb:expr) => {{
		let base: slos_filesystem::FilesystemBase = $genb();
		let fs: $t = $gent();

		(base, fs)
	}};
}

#[macro_use]
mod mount_tests;
#[macro_use]
mod read_tests;
#[macro_use]
mod write_tests;
