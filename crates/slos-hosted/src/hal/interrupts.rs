//! Virtual interrupts

use slos::clock;

/// A virtual interrupt
#[derive(Debug)]
pub enum HostedInterrupt {
	/// Clock tick interrupt
	ClockTick,
}

impl HostedInterrupt {
	/// Perform handling for `self` as an interrupt
	pub fn dispatch(&self) {
		match self {
			Self::ClockTick => {
				clock::on_tick();
			}
		}
	}
}
