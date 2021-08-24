use lazy_static::lazy_static;
use slos_hal::{SystemConsole, SystemConsoleInput, SystemConsoleOutput, SystemHardware};
use slos_helpers::UnsafeContainer;

pub mod console;

lazy_static! {
	pub static ref SYSTEM: UnsafeContainer<HostedSystem> = UnsafeContainer::new(Default::default());
}

#[derive(Debug)]
pub struct HostedSystem {
	/// Whether to make kmain return in its next iteration
	pub return_next_iter: bool,
}

impl Default for HostedSystem {
	fn default() -> Self {
		Self {
			return_next_iter: false,
		}
	}
}

impl SystemConsole for HostedSystem {
	fn console_input(&self) -> &'static mut dyn SystemConsoleInput {
		console::CONSOLE_STDIN.get()
	}

	fn console_output(&self) -> &'static mut dyn SystemConsoleOutput {
		console::CONSOLE_STDOUT.get()
	}
}

impl SystemHardware for HostedSystem {
	fn has_requested_return(&self) -> bool {
		self.return_next_iter
	}
}
