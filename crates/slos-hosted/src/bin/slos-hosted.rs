use anyhow::{anyhow, Result};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use signal_hook::iterator::{exfiltrator::SignalOnly, SignalsInfo};
use simple_logger::SimpleLogger;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;

use slos::kmain;
use slos_hosted::hal::SYSTEM;

fn hosted_main() -> Result<()> {
	// Init SYSTEM
	log::debug!("SYSTEM is currently {:#?}", SYSTEM.get());

	// Hand over to kmain
	log::info!("Init complete, handing over to kmain");
	kmain(SYSTEM.get()).or_else(|x| Err(anyhow!("kmain returned an error!? {:#?}", x)))?;

	Ok(())
}

fn main() -> Result<()> {
	SimpleLogger::new().init()?;

	let term_now = Arc::new(AtomicBool::new(false));
	for sig in TERM_SIGNALS {
		flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
		flag::register(*sig, Arc::clone(&term_now))?;
	}

	// Start hosted_main
	log::debug!("Starting hosted_main in a thread");
	let main_handle = thread::spawn(hosted_main);

	// Wait for signal and set immediate return flag
	let mut signals = SignalsInfo::<SignalOnly>::new(TERM_SIGNALS)?;
	for _ in &mut signals {
		log::warn!("Requesting kmain return, ^C again to terminate");
		let mut system = SYSTEM.get();
		system.return_next_iter = true;

		break;
	}

	main_handle
		.join()
		.or_else(|x| Err(anyhow!("hosted_main error {:#?}", x)))??;

	Ok(())
}
