use crate::alloc_prelude::*;

use crate::{SystemConsole, SystemConsoleInput, SystemConsoleOutput};
use lazy_static::lazy_static;
use slos_filesystem::{FsError, FsFileHandle};
use slos_helpers::UnsafeContainer;

lazy_static! {
	pub static ref IGNORE_DEVICE: UnsafeContainer<IgnoreDevice> =
		UnsafeContainer::new(IgnoreDevice);
	pub static ref NULL_CONSOLE: UnsafeContainer<NullConsole> = UnsafeContainer::new(NullConsole);
}

pub struct IgnoreDevice;

impl SystemConsoleInput for IgnoreDevice {}
impl SystemConsoleOutput for IgnoreDevice {}

impl FsFileHandle for IgnoreDevice {
	fn read(&mut self, _offset: usize, _length: Option<usize>) -> Result<Vec<u8>, FsError> {
		Ok(Vec::new())
	}

	fn write(&mut self, _offset: usize, _data: &[u8]) -> Result<(), FsError> {
		Ok(())
	}
}

pub struct NullConsole;

impl SystemConsole for NullConsole {
	fn console_input(&self) -> &'static mut dyn SystemConsoleInput {
		IGNORE_DEVICE.get()
	}

	fn console_output(&self) -> &'static mut dyn SystemConsoleOutput {
		IGNORE_DEVICE.get()
	}
}
