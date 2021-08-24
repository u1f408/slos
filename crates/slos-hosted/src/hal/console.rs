use lazy_static::lazy_static;
use slos_filesystem::{FsError, FsFileHandle};
use slos_hal::{SystemConsoleInput, SystemConsoleOutput};
use slos_helpers::UnsafeContainer;
use std::io::{self, Read, Write};

lazy_static! {
	pub static ref CONSOLE_STDIN: UnsafeContainer<ConsoleStdin> =
		UnsafeContainer::new(ConsoleStdin);
	pub static ref CONSOLE_STDOUT: UnsafeContainer<ConsoleStdout> =
		UnsafeContainer::new(ConsoleStdout);
}

pub struct ConsoleStdin;
impl SystemConsoleInput for ConsoleStdin {}
impl FsFileHandle for ConsoleStdin {
	fn read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError> {
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

	fn write(&mut self, _offset: usize, _data: &[u8]) -> Result<(), FsError> {
		Err(FsError::InvalidArgument)
	}
}

pub struct ConsoleStdout;
impl SystemConsoleOutput for ConsoleStdout {}
impl FsFileHandle for ConsoleStdout {
	fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError> {
		if offset != 0 {
			return Err(FsError::InvalidArgument);
		}

		io::stdout().write_all(data)?;
		Ok(())
	}

	fn read(&mut self, _offset: usize, _length: Option<usize>) -> Result<Vec<u8>, FsError> {
		Err(FsError::InvalidArgument)
	}
}
