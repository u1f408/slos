#![no_std]
#![feature(alloc_prelude)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[allow(unused_imports)]
use self::alloc_prelude::*;
use alloc::prelude::v1 as alloc_prelude;

use core::fmt::Debug;
use slos_filesystem::FsFileHandle;

pub mod null_system;

/// System console
pub trait SystemConsole: Debug + FsFileHandle {}

/// System CPU handling
pub trait SystemCpu {
	/// Disable interrupts on the current CPU
	fn interrupts_disable(&mut self);

	/// Enable interrupts on the current CPU
	fn interrupts_enable(&mut self);

	/// Return whether interrupts are enabled on the current CPU
	fn interrupts_are_enabled(&self) -> bool;

	/// Halt the current CPU
	fn halt(&mut self);
}

/// Optional `kmain` hook methods
pub trait SystemKmainHooks {
	/// `kmain` loop head hook
	///
	/// This is called at the beginning of every iteration of the `kmain`
	/// main loop.
	fn hook_kmain_loop_head(&mut self) {}

	/// `kmain` inner partial loop hook
	///
	/// This is called after **each** function in the `KMAIN_LOOP_PARTIALS`
	/// collection.
	fn hook_kmain_loop_inner_part(&mut self) {}
}

/// Base system hardware trait
pub trait SystemHardware: Send + Debug + SystemKmainHooks {
	/// Name of the crate implementing this system
	///
	/// This method should be implemented as the following:
	///
	/// ```no_build
	/// fn system_name(&self) -> &'static str {
	///	    env!("CARGO_PKG_NAME")
	/// }
	/// ```
	fn system_name(&self) -> &'static str;

	/// Get a reference to the default [`SystemConsole`] instance
	fn console(&mut self) -> &'static mut dyn SystemConsole;

	/// Has the HAL has requested an immediate kmain return?
	fn has_requested_return(&self) -> bool;

	/// Current CPU
	fn current_cpu(&mut self) -> &'static mut dyn SystemCpu;

	/// Virtualization
	fn virtualization(&self) -> Option<(&'static str, ())>;
}
