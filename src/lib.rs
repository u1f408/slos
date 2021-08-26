#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_prelude)]
#![feature(trait_upcasting)]

extern crate alloc;
use alloc::prelude::v1 as alloc_prelude;

#[allow(unused_imports)]
use crate::alloc_prelude::*;

#[macro_use]
extern crate slos_log;

use lazy_static::lazy_static;
use slos_hal::SystemHardware;
use slos_helpers::{StaticCollection, UnsafeContainer};

mod errors;
pub use self::errors::*;
pub mod clock;
pub mod filesystem;

/// Callback type for [`kmain`] inner functions
pub type KmainPartial = fn() -> Result<(), KernelError>;

lazy_static! {
	/// Collection of [`KmainPartial`] callbacks, called before the [`kmain`] loop
	pub static ref KMAIN_INIT_PARTIALS: StaticCollection<Option<KmainPartial>> = {
		let mut partials = StaticCollection::new();

		// Basic initialization things
		partials.push(Some(clock::init as KmainPartial));
		partials.push(Some(filesystem::init as KmainPartial));

		// Init examples, if enabled by feature
		#[cfg(feature = "init_examples")]
		{
			log::warn!("KMAIN_INIT_PARTIALS: init_examples feature enabled");
			partials.push(Some(filesystem::init_examples_console_write as KmainPartial));
		}

		partials
	};

	/// Collection of [`KmainPartial`] callbacks, called each iteration of the [`kmain`] loop
	pub static ref KMAIN_LOOP_PARTIALS: StaticCollection<Option<KmainPartial>> = {
		#[allow(unused_mut)]
		let mut partials = StaticCollection::new();

		partials
	};
}

/// Maybe a [`SystemHardware`] implementation
#[doc(hidden)]
pub static mut SYSTEM: Option<UnsafeContainer<&'static mut dyn SystemHardware>> = None;

/// Returns the [`SystemHardware`] implementation for the running system
///
/// Panics if the system has not been initialized.
pub fn current_system() -> &'static mut dyn SystemHardware {
	unsafe {
		if SYSTEM.is_none() {
			panic!("slos::SYSTEM has not been initialized");
		}

		&mut **SYSTEM.as_ref().unwrap().get()
	}
}

/// Kernel main function
pub fn kmain(initial_system: &'static mut dyn SystemHardware) -> Result<(), KernelError> {
	unsafe {
		SYSTEM = Some(UnsafeContainer::new(initial_system));
	}

	log::info!(
		"Hello from {} v{}, on system {}, with features {:?}",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
		current_system().system_name(),
		env!("SLOS_FEATURES"),
	);

	if let Some((virt_type, _)) = current_system().virtualization() {
		log::info!("system is virtualized: {}", virt_type);
		clock::treat_as_unstable();
	}

	// Run init partials
	for partial in KMAIN_INIT_PARTIALS.as_slice().iter() {
		if let Some(partial) = partial {
			(partial)()?;
		}
	}

	while !current_system().has_requested_return() {
		current_system().hook_kmain_loop_head();

		// Run loop partials
		for partial in KMAIN_LOOP_PARTIALS.as_slice().iter() {
			if let Some(partial) = partial {
				(partial)()?;
			}

			current_system().hook_kmain_loop_inner_part();
		}

		// Enable interrupts and then halt
		current_system().current_cpu().interrupts_enable();
		current_system().current_cpu().halt();
	}

	error!("returning, was alive for {}s", clock::BOOT_CLOCK.get());
	Ok(())
}
