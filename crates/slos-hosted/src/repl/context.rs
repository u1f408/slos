use slos_filesystem::FilesystemBase;

/// Default command context type
pub struct Context {
	/// Filesystem
	pub fs: FilesystemBase,
}

impl Context {
	pub fn new() -> Self {
		Self {
			fs: FilesystemBase::new(),
		}
	}
}
