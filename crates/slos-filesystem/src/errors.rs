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

    /// Filesystem root error (possibly set to None)
    FilesystemRootError,

    /// Unknown error
    Unknown,
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
