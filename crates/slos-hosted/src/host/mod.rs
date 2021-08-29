//! Hosted slOS kernel
//!
//! This module contains the functions necessary to start the slOS kernel as an
//! application on a host *NIX system.

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

/// Initialize the environment for the hosted kernel
pub fn init(kargs: String, rootfs: Option<PathBuf>) -> Result<()> {
	// TODO: this

	let _ = kargs;
	let _ = rootfs;

	Ok(())
}

/// Run [`slos::kmain`] in our hosted environment
///
/// This function should be spawned in a new thread, the handle of that thread
/// stored (so we can `join` it at the end of execution), and the thread object
/// passed to the helper threads that should be spawned after the thread running
/// this function.
pub fn hosted_kmain() -> Result<()> {
	kmain().or_else(|x| Err(anyhow!("kmain returned an error!? {:#?}", x)))?;
	Ok(())
}

/// Start the hosted kernel
///
/// Does the following things, in this order:
///
/// - Set up signal handling so that a `^C` will attempt to gracefully stop the
///   running kernel threads
/// - Start [`hosted_kmain`] in a new thread
/// - Start helper threads:
///    - [`interrupts::dispatcher`]
///    - [`interrupts::clock_tick`]
/// - Waits for a termination signal
/// - Joins on the [`hosted_kmain`] thread
/// - Joins on the helper threads
pub fn run_kernel() -> Result<()> {
	let term_now = Arc::new(AtomicBool::new(false));
	for sig in TERM_SIGNALS {
		flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
		flag::register(*sig, Arc::clone(&term_now))?;
	}

	debug!("Initializing kernel");
	slos::init(SYSTEM.get()).or_else(|x| Err(anyhow!("slos::init error {:#?}", x)))?;

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
