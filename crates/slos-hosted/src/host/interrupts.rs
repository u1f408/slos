//! Virtual interrupt handlers

use std::thread::{self, Thread};
use std::time::Duration;

use crate::hal::interrupts::HostedInterrupt;
use crate::hal::SYSTEM;

/// Dispatch virtual interrupts, if enabled
///
/// For each interrupt in the queue:
///
/// - Halt the hosted kmain thread (kmain hooks will park the thread)
/// - Call the [`HostedInterrupt::dispatch`] method for the interrupt
/// - Unhalt the hosted kmain thread
/// - Manually unpark the hosted kmain thread
///
/// This function should be spawned in a new thread immediately after spawning
/// the thread running [`hosted_kmain`][crate::host::hosted_kmain].
pub fn dispatcher(kmain_thread: Thread) {
	while !SYSTEM.return_next_iter {
		thread::sleep(Duration::from_millis(20));
		if !SYSTEM.interrupts_enabled {
			continue;
		}

		while let Some(interrupt) = SYSTEM.get().pending_interrupts.pop() {
			trace!("dispatching {:?}", interrupt);

			// Halt the CPU, which will park the thread at the next opportunity
			SYSTEM.get().halted = true;

			// Dispatch the interrupt
			interrupt.dispatch();

			// Unhalt the CPU and manually unpark the thread
			SYSTEM.get().halted = false;
			kmain_thread.unpark();
		}
	}
}

/// Update the system clock and queue tick interrupts
///
/// While the hosted system is running (that is, `!SYSTEM.return_next_iter`):
///
/// - Update [`slos::clock::BOOT_CLOCK`] with the real time passed since
///    the last iteration,
/// - Queue a [`HostedInterrupt::ClockTick`] interrupt,
/// - Sleep the current thread for ~50ms
///
/// This function should be spawned in a new thread immediately after spawning
/// the thread running [`hosted_kmain`][crate::host::hosted_kmain].
pub fn clock_tick() {
	use std::time::{SystemTime, UNIX_EPOCH};

	// Get the current time since epoch as milliseconds, which we'll update
	// each iteration so we have the time since the last update
	let mut previous = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();

	while !SYSTEM.return_next_iter {
		// Get the current time since epoch as milliseconds
		let current = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_millis();

		// Update the BOOT_CLOCK with the actual amount of time passed
		slos::clock::BOOT_CLOCK
			.get()
			.increment_ms(((current - previous) & u32::MAX as u128) as u32);

		// Push a ClockTick interrupt
		SYSTEM
			.get()
			.pending_interrupts
			.push(HostedInterrupt::ClockTick);

		// Update the previous time variable with the current time, so the next
		// iteration can do the subtraction and get the time since last update
		previous = current;

		// And sleep for 50ms
		thread::sleep(Duration::from_millis(50));
	}
}
