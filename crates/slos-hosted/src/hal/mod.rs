use lazy_static::lazy_static;
use std::thread::{self, Thread};
use std::time::Duration;

use slos_hal::{SystemConsole, SystemCpu, SystemHardware, SystemKmainHooks};
use slos_helpers::UnsafeContainer;

pub mod console;
pub mod interrupts;

lazy_static! {
	pub static ref SYSTEM: UnsafeContainer<HostedSystem> = UnsafeContainer::new(Default::default());
}

#[derive(Debug)]
pub struct HostedSystem {
	/// Whether to make kmain return in its next iteration
	pub return_next_iter: bool,

	/// Number of pending interrupts
	pub pending_interrupts: Vec<interrupts::HostedInterrupt>,

	/// Whether interrupts are enabled
	pub interrupts_enabled: bool,

	/// Whether the hosted machine is halted until the next interrupt
	pub halted: bool,

	/// kmain std::thread::Thread
	pub kmain_thread: Option<Thread>,
}

impl HostedSystem {
	fn park_if_halted(&mut self) {
		while self.halted {
			if let Some(kmain_thread) = &self.kmain_thread {
				if thread::current().id() == kmain_thread.id() {
					thread::park()
				}
			}

			if self.return_next_iter {
				log::debug!("HostedSystem return_next_iter set, unparking and unhalting");
				self.halted = false;
				if let Some(kmain_thread) = &self.kmain_thread {
					kmain_thread.unpark();
				}

				break;
			}

			thread::sleep(Duration::from_millis(50));
		}
	}
}

impl Default for HostedSystem {
	fn default() -> Self {
		Self {
			return_next_iter: false,
			pending_interrupts: Vec::new(),
			interrupts_enabled: false,
			halted: false,
			kmain_thread: None,
		}
	}
}

impl SystemCpu for HostedSystem {
	fn interrupts_disable(&mut self) {
		self.interrupts_enabled = false;
	}

	fn interrupts_enable(&mut self) {
		self.interrupts_enabled = true;
	}

	fn interrupts_are_enabled(&self) -> bool {
		self.interrupts_enabled
	}

	fn halt(&mut self) {
		self.halted = true;
		self.park_if_halted();
	}
}

impl SystemKmainHooks for HostedSystem {
	fn hook_kmain_loop_head(&mut self) {
		self.park_if_halted();
	}

	fn hook_kmain_loop_inner_part(&mut self) {
		self.park_if_halted();
	}
}

impl SystemHardware for HostedSystem {
	fn system_name(&self) -> &'static str {
		env!("CARGO_PKG_NAME")
	}

	fn console(&mut self) -> &'static mut dyn SystemConsole {
		console::CONSOLE.get()
	}

	fn has_requested_return(&self) -> bool {
		self.return_next_iter
	}

	fn current_cpu(&mut self) -> &'static mut dyn SystemCpu {
		SYSTEM.get()
	}

	fn virtualization(&self) -> Option<(&'static str, ())> {
		Some((env!("CARGO_PKG_NAME"), ()))
	}
}
