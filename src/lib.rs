#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_prelude)]
#![feature(trait_upcasting)]

extern crate alloc;
use crate::alloc_prelude::*;
pub use alloc::prelude::v1 as alloc_prelude;

#[macro_use]
extern crate slos_log;

use lazy_static::lazy_static;
use slos_hal::SystemHardware;
use slos_helpers::UnsafeContainer;

mod errors;
pub use self::errors::*;
pub mod clock;
pub mod filesystem;

pub type KmainPartial = fn() -> Result<(), KernelError>;

lazy_static! {
	pub static ref KMAIN_INIT_PARTIALS: UnsafeContainer<Vec<KmainPartial>> = {
		let mut partials = Vec::new();

		// Basic initialization things
		partials.push(clock::init as KmainPartial);
		partials.push(filesystem::init as KmainPartial);

		UnsafeContainer::new(partials)
	};

	pub static ref KMAIN_LOOP_PARTIALS: UnsafeContainer<Vec<KmainPartial>> = {
		#[allow(unused_mut)]
		let mut partials = Vec::new();

		UnsafeContainer::new(partials)
	};
}

pub static mut SYSTEM: Option<UnsafeContainer<&'static mut dyn SystemHardware>> = None;
pub fn current_system() -> &'static mut dyn SystemHardware {
	unsafe {
		if SYSTEM.is_none() {
			panic!("slos::SYSTEM has not been initialized");
		}

		&mut **SYSTEM.as_ref().unwrap().get()
	}
}

pub fn kmain(initial_system: &'static mut dyn SystemHardware) -> Result<(), KernelError> {
	unsafe {
		SYSTEM = Some(UnsafeContainer::new(initial_system));
	}

	log::info!(
		"Hello from {} (system: {})",
		concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION")),
		current_system().system_name(),
	);

	if let Some((virt_type, _)) = current_system().virtualization() {
		log::info!("system is virtualized: {}", virt_type);
		clock::treat_as_unstable();
	}

	// Run init partials
	for partial in KMAIN_INIT_PARTIALS.get().iter() {
		(partial)()?;
	}

	while !current_system().has_requested_return() {
		current_system().hook_kmain_loop_head();

		// Run loop partials
		for partial in KMAIN_LOOP_PARTIALS.get().iter() {
			(partial)()?;
			current_system().hook_kmain_loop_inner_part();
		}

		// Enable interrupts and then halt
		current_system().current_cpu().interrupts_enable();
		current_system().current_cpu().halt();
	}

	error!(
		"returning, was alive for {}s",
		clock::BOOT_CLOCK.get()
	);
	Ok(())
}
