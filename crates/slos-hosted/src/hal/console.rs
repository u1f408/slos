use lazy_static::lazy_static;
use slos_filesystem::{FsError, FsFileHandle};
use slos_hal::SystemConsole;
use slos_helpers::UnsafeContainer;
use std::io::{self, Read, Write};

lazy_static! {
	pub static ref CONSOLE: UnsafeContainer<Console> = UnsafeContainer::new(Console);
}

pub struct Console;

impl FsFileHandle for Console {
	fn raw_read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError> {
		if offset != 0 {
			return Err(FsError::InvalidArgument);
		}

		let mut buffer = Vec::new();
		if let Some(len) = length {
			buffer.reserve(len);
		}

		io::stdin().read(&mut buffer[..])?;
		Ok(buffer)
	}

	fn raw_write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError> {
		if offset != 0 {
			return Err(FsError::InvalidArgument);
		}

		let mut stdout = io::stdout();
		stdout.write_all(data)?;
		stdout.flush()?;

		Ok(())
	}
}

impl SystemConsole for Console {}
