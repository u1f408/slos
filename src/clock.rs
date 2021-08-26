use lazy_static::lazy_static;

use crate::KernelError;

use slos_helpers::Timer;
use slos_helpers::UnsafeContainer;

/// Warn if the clock increases more than this time (in milliseconds) in one tick
pub const CLOCK_TICK_WARN_MS: u32 = 100;

/// If CLOCK_TICK_WARN_COUNT >= this value, set CLOCK_UNSTABLE
pub const CLOCK_TICK_WARN_THRESHOLD: usize = 2;

/// Current number of warnings
pub static mut CLOCK_TICK_WARN_COUNT: usize = 0;

/// Whether to treat the system clock as unstable
pub static mut CLOCK_UNSTABLE: bool = false;

lazy_static! {
	static ref PREVIOUS_CLOCK_TICK: UnsafeContainer<Timer> = UnsafeContainer::new(Timer::new());
	pub static ref BOOT_CLOCK: UnsafeContainer<Timer> = UnsafeContainer::new(Timer::new());
}

pub fn init() -> Result<(), KernelError> {
	info!("initializing clock");
	let _boot_clock = BOOT_CLOCK.get();

	// TODO: clock initialization

	Ok(())
}

pub fn treat_as_unstable() {
	debug!("clock is being treated as unstable");
	unsafe {
		CLOCK_UNSTABLE = true;
	}
}

pub fn on_tick() {
	// Warn if we've increased more than CLOCK_TICK_WARN
	let mut warn_clock = PREVIOUS_CLOCK_TICK.get().clone();
	warn_clock.increment_ms(CLOCK_TICK_WARN_MS);
	if *BOOT_CLOCK.get() > warn_clock {
		warn!(
			"BOOT_CLOCK increased {}ms in one tick, something's going on here",
			BOOT_CLOCK.in_milliseconds() - PREVIOUS_CLOCK_TICK.in_milliseconds()
		);

		unsafe {
			CLOCK_TICK_WARN_COUNT += 1;
			if !CLOCK_UNSTABLE && CLOCK_TICK_WARN_COUNT >= CLOCK_TICK_WARN_THRESHOLD {
				log::warn!("Clock source appears to be unstable, marking as such");
				treat_as_unstable();
			}
		}
	}

	// Copy BOOT_CLOCK into PREVIOUS_CLOCK_TICK
	PREVIOUS_CLOCK_TICK.replace(BOOT_CLOCK.get().clone());
}
