#[macro_use]
extern crate slos_log;

use anyhow::{anyhow, Result};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use signal_hook::iterator::{exfiltrator::SignalOnly, SignalsInfo};
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, Thread};
use std::time::Duration;
use log::LevelFilter;

use slos::kmain;
use slos_hosted::hal::interrupts::HostedInterrupt;
use slos_hosted::hal::SYSTEM;

fn hosted_interrupts(kmain_thread: Thread) {
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

fn hosted_clock_tick() {
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

fn hosted_main() -> Result<()> {
	// Init SYSTEM
	trace!("SYSTEM is currently {:?}", SYSTEM.get());

	// Hand over to kmain
	info!("Init complete, handing over to kmain");
	kmain(SYSTEM.get()).or_else(|x| Err(anyhow!("kmain returned an error!? {:#?}", x)))?;

	Ok(())
}

fn main() -> Result<()> {
	env_logger::Builder::new()
		.format_timestamp_nanos()
		.filter_level(LevelFilter::Info)
		.filter_module(module_path!(), LevelFilter::Debug)
		.parse_default_env()
		.try_init()?;

	let term_now = Arc::new(AtomicBool::new(false));
	for sig in TERM_SIGNALS {
		flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
		flag::register(*sig, Arc::clone(&term_now))?;
	}

	// Start hosted_main
	debug!("Starting hosted_main in a thread");
	let main_handle = thread::spawn(hosted_main);
	let main_thread = main_handle.thread().clone();
	SYSTEM.get().kmain_thread = Some(main_thread.clone());

	// Start other threads
	debug!("Starting utility threads");
	let handles = vec![
		thread::spawn(move || hosted_interrupts(main_thread.clone())),
		thread::spawn(hosted_clock_tick),
	];

	// Wait for signal and set immediate return flag
	let mut signals = SignalsInfo::<SignalOnly>::new(TERM_SIGNALS)?;
	for _ in &mut signals {
		error!("Requesting kmain return, ^C again to terminate");

		// Tell the thread to return as soon as it's unparked …
		let mut system = SYSTEM.get();
		system.return_next_iter = true;

		// … and unpark it
		main_handle.thread().unpark();

		break;
	}

	main_handle
		.join()
		.or_else(|x| Err(anyhow!("hosted_main error {:#?}", x)))??;

	for handle in handles {
		handle.join().expect("failed to join thread");
	}

	Ok(())
}
