use anyhow::{anyhow, Result};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use signal_hook::iterator::{exfiltrator::SignalOnly, SignalsInfo};
use std::path::PathBuf;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;

use crate::hal::SYSTEM;
use slos::kmain;

pub mod interrupts;

pub fn init(_kargs: String, _rootfs: Option<PathBuf>) -> Result<()> {
	// TODO: this
	Ok(())
}

pub fn hosted_kmain() -> Result<()> {
	kmain(SYSTEM.get()).or_else(|x| Err(anyhow!("kmain returned an error!? {:#?}", x)))?;
	Ok(())
}

pub fn run_kernel() -> Result<()> {
	let term_now = Arc::new(AtomicBool::new(false));
	for sig in TERM_SIGNALS {
		flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
		flag::register(*sig, Arc::clone(&term_now))?;
	}

	// Start hosted_main
	debug!("Starting hosted_kmain in a thread");
	let kmain_handle = thread::spawn(hosted_kmain);
	let kmain_thread = kmain_handle.thread().clone();
	SYSTEM.get().kmain_thread = Some(kmain_thread.clone());

	// Start other threads
	debug!("Starting utility threads");
	let handles = vec![
		thread::spawn(move || interrupts::dispatcher(kmain_thread.clone())),
		thread::spawn(interrupts::clock_tick),
	];

	// Wait for signal and set immediate return flag
	let mut signals = SignalsInfo::<SignalOnly>::new(TERM_SIGNALS)?;
	for _ in &mut signals {
		error!("Requesting kmain return, ^C again to terminate");

		// Tell the thread to return as soon as it's unparked …
		let mut system = SYSTEM.get();
		system.return_next_iter = true;

		// … and unpark it
		kmain_handle.thread().unpark();

		break;
	}

	kmain_handle
		.join()
		.or_else(|x| Err(anyhow!("hosted_main error {:#?}", x)))??;

	for handle in handles {
		handle.join().expect("failed to join thread");
	}

	Ok(())
}
