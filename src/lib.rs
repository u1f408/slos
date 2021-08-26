#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_prelude)]
#![feature(trait_upcasting)]

extern crate alloc;
use crate::alloc_prelude::*;
pub use alloc::prelude::v1 as alloc_prelude;

use lazy_static::lazy_static;

use slos_hal::SystemHardware;
use slos_helpers::UnsafeContainer;

mod errors;
pub use self::errors::*;
pub mod clock;
pub mod filesystem;

pub type KmainPartial = fn() -> Result<(), KernelError>;

lazy_static! {
	pub static ref KMAIN_PARTIALS: UnsafeContainer<Vec<Box<KmainPartial>>> = {
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

	clock::init()?;
	filesystem::init()?;

	// TODO: remove this
	{
		log::warn!("Trying to write to the console via the filesystem, here goes nothing!");
		let fsbase = crate::filesystem::FSBASE.get();
		if let Ok(devnode) = fsbase.node_at_path(&["sys", "dev"]) {
			if let Some(devdir) = devnode.try_directory() {
				if let Ok(mut devices) = devdir.readdir() {
					if let Some(consolenode) = devices.iter_mut().filter(|x| x.name() == "console").next() {
						if let Some(consolefile) = consolenode.try_file() {
							if let Ok(consolehandle) = consolefile.open() {
								let _ = consolehandle.raw_write(0, b"hello via the filesystem!\n");
							}
						}
					}
				}
			}
		}
	}

	while !current_system().has_requested_return() {
		current_system().hook_kmain_loop_head();

		for partial in KMAIN_PARTIALS.get().iter() {
			(partial)()?;
			current_system().hook_kmain_loop_inner_part();
		}

		// Enable interrupts and then halt
		current_system().current_cpu().interrupts_enable();
		current_system().current_cpu().halt();
	}

	log::debug!(
		"kmain returning, was alive for {}s",
		clock::BOOT_CLOCK.get()
	);
	Ok(())
}
