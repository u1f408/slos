use displaydoc::Display as DisplayDoc;

#[derive(DisplayDoc, Debug)]
pub enum FsError {
	/// Permission denied
	PermissionDenied,

	/// A file with a duplicate name already exists
	FileExists,

	/// File already has an open handle
	OpenHandleExists,

	/// End of file reached (possibly unexpectedly)
	EndOfFile,

	/// File not found
	FileNotFound,

	/// Invalid argument
	InvalidArgument,

	/// Filesystem root error (possibly set to None)
	FilesystemRootError,

	#[cfg(feature = "std")]
	/// IO error: {0}
	StdIoError(std::io::Error),

	/// Unknown error
	Unknown,
}

#[cfg(feature = "std")]
impl From<std::io::Error> for FsError {
	fn from(e: std::io::Error) -> FsError {
		FsError::StdIoError(e)
	}
}

#[cfg(feature = "std")]
impl std::error::Error for FsError {}

#[derive(DisplayDoc, Debug)]
pub enum MountError {
	/// Permission denied
	PermissionDenied,

	/// Unknown error
	Unknown,
}

#[cfg(feature = "std")]
impl std::error::Error for MountError {}
