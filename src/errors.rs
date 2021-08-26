use displaydoc::Display as DisplayDoc;

#[derive(DisplayDoc, Debug)]
pub enum KernelError {
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
