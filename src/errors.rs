use displaydoc::Display as DisplayDoc;

/// Fatal kernel errors
#[derive(DisplayDoc, Debug)]
pub enum KernelError {
	/// `kmain` called before `init`
	KernelUninitialized,

	/// Bubbled filesystem mount error
	MountError(slos_filesystem::MountError),

	/// Unknown error
	Unknown,
}

impl From<slos_filesystem::MountError> for KernelError {
	fn from(e: slos_filesystem::MountError) -> KernelError {
		KernelError::MountError(e)
	}
}
