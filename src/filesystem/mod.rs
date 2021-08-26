use lazy_static::lazy_static;

use crate::alloc_prelude::*;
use crate::KernelError;

use slos_filesystem::path as fspath;
use slos_filesystem::{FilesystemBase, FsError, FsFileHandle};
use slos_helpers::UnsafeContainer;

pub mod devices;

lazy_static! {
	pub static ref FSBASE: UnsafeContainer<FilesystemBase> =
		UnsafeContainer::new(FilesystemBase::new());
}

pub fn init() -> Result<(), KernelError> {
	info!("initializing filesystem");
	let fs = FSBASE.get();

	fs.mount(&["sys", "dev"], Box::new(devices::system_devices()))?;

	trace!("FSBASE is currently {:?}", fs);
	Ok(())
}

/// Open a file somewhere on the filesystem, returning a handle
///
/// The `path` is normalized and segmented before being used to call the
/// `FilesystemBase::node_at_path` method to find the file in question.
/// Errors from that are bubbled, if that function returns a non-file node
/// then we return an `Err(FsError::FileNotFound)`, otherwise we try to get
/// a handle on that file by returning the `Result` from `FsFile::open`.
pub fn fopen<'a>(path: &str) -> Result<&'a mut (dyn FsFileHandle), FsError> {
	let path = fspath::split(path);
	let path = path.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

	match FSBASE.get().node_at_path(&path)?.try_file() {
		Some(file) => file.open(),
		None => Err(FsError::FileNotFound),
	}
}

#[cfg(feature = "init_examples")]
pub fn init_examples_console_write() -> Result<(), KernelError> {
	debug!("attempting write to system console via filesystem");
	if let Ok(fh) = fopen("/sys/dev/console") {
		if let Err(e) = fh.raw_write(0, b"hello via the filesystem!\n") {
			error!("failed to write to console: {:?}", e);
		}
	}

	Ok(())
}
