#![no_std]
#![feature(alloc_prelude)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[allow(unused_imports)]
use self::alloc_prelude::*;
use alloc::prelude::v1 as alloc_prelude;

use slos_filesystem::FsFileHandle;

pub mod null_console;

pub trait SystemConsoleInput: FsFileHandle {}
pub trait SystemConsoleOutput: FsFileHandle {}
pub trait SystemConsole {
	fn console_input(&self) -> &'static mut dyn SystemConsoleInput;
	fn console_output(&self) -> &'static mut dyn SystemConsoleOutput;
}

pub trait SystemHardware: SystemConsole {
	/// Has the HAL has requested an immediate kmain return?
	fn has_requested_return(&self) -> bool;
}
