use slos_filesystem::FilesystemBase;

pub struct Context {
	pub fs: FilesystemBase,
}

impl Context {
	pub fn new() -> Self {
		Self {
			fs: FilesystemBase::new(),
		}
	}
}
