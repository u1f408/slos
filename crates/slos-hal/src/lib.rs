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

pub trait SystemConsole: FsFileHandle {}

pub trait SystemCpu {
	fn interrupts_disable(&mut self);
	fn interrupts_enable(&mut self);
	fn interrupts_are_enabled(&self) -> bool;
	fn halt(&mut self);
}

pub trait SystemKmainHooks {
	fn hook_kmain_loop_head(&mut self) {}
	fn hook_kmain_loop_inner_part(&mut self) {}
}

pub trait SystemHardware: Send + SystemKmainHooks {
	fn system_name(&self) -> &'static str;

	fn console(&mut self) -> &'static mut dyn SystemConsole;

	/// Has the HAL has requested an immediate kmain return?
	fn has_requested_return(&self) -> bool;

	/// Current CPU
	fn current_cpu(&mut self) -> &'static mut dyn SystemCpu;

	/// Virtualization
	fn virtualization(&self) -> Option<(&'static str, ())>;
}
