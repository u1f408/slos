//! Null console implementation

use crate::alloc_prelude::*;

use crate::SystemConsole;
use lazy_static::lazy_static;
use slos_filesystem::{FsError, FsFileHandle};
use slos_helpers::UnsafeContainer;

lazy_static! {
	/// Global instance of the [`NullConsole`]
	pub static ref NULL_CONSOLE: UnsafeContainer<NullConsole> = UnsafeContainer::new(NullConsole);
}

/// A [`SystemConsole`] implementation that ignores read & write operations
pub struct NullConsole;

impl FsFileHandle for NullConsole {
	fn raw_read(&mut self, _offset: usize, _length: Option<usize>) -> Result<Vec<u8>, FsError> {
		Ok(Vec::new())
	}

	fn raw_write(&mut self, _offset: usize, _data: &[u8]) -> Result<(), FsError> {
		Ok(())
	}
}

impl SystemConsole for NullConsole {}
