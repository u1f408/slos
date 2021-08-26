use lazy_static::lazy_static;

use crate::alloc_prelude::*;
use crate::KernelError;

use slos_filesystem::FilesystemBase;
use slos_helpers::UnsafeContainer;

pub mod devices;

lazy_static! {
	pub static ref FSBASE: UnsafeContainer<FilesystemBase> =
		UnsafeContainer::new(FilesystemBase::new());
}

pub fn init() -> Result<(), KernelError> {
	info!("initializing filesystem");
	let fs = FSBASE.get();

	fs.mount(&["sys", "dev"], Box::new(devices::system_devices()))?;

	trace!("FSBASE is currently {:?}", fs);
	Ok(())
}

#[cfg(feature = "init_examples")]
pub fn init_examples_console_write() -> Result<(), KernelError> {
	debug!("attempting write to system console via filesystem");
	if let Ok(devnode) = FSBASE.get().node_at_path(&["sys", "dev"]) {
		if let Some(devdir) = devnode.try_directory() {
			if let Ok(mut devices) = devdir.readdir() {
				if let Some(consolenode) = devices.iter_mut().filter(|x| x.name() == "console").next() {
					if let Some(consolefile) = consolenode.try_file() {
						if let Ok(consolehandle) = consolefile.open() {
							let _ = consolehandle.raw_write(0, b"hello via the filesystem!\n");
						}
					}
				}
			}
		}
	}

	Ok(())
}
