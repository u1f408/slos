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

pub trait SystemConsole: FsFileHandle {
}

pub trait SystemHardware: Send {
	fn console(&mut self) -> &'static mut dyn SystemConsole;

	/// Has the HAL has requested an immediate kmain return?
	fn has_requested_return(&self) -> bool;
}
