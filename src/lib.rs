#![no_std]

use slos_hal::SystemHardware;

mod errors;
pub use self::errors::*;

pub fn kmain(system: &'static mut dyn SystemHardware) -> Result<(), KernelError> {
	log::debug!("kmain entry");

	system
		.console_output()
		.write(0, b"Hello, world!\n")
		.or(Err(KernelError::Unknown))?;

	while !system.has_requested_return() {
		// Do something
	}

	log::debug!("kmain returning!");
	Ok(())
}
