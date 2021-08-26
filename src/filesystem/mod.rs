use lazy_static::lazy_static;

use crate::alloc_prelude::*;
use crate::KernelError;

use slos_filesystem::FilesystemBase;
use slos_helpers::UnsafeContainer;

lazy_static! {
	pub static ref FSBASE: UnsafeContainer<FilesystemBase> =
		UnsafeContainer::new(FilesystemBase::new());
}

pub fn init() -> Result<(), KernelError> {
	log::info!("initializing filesystem");
	let fs = FSBASE.get();

	log::debug!("FSBASE is currently {:?}", fs);
	Ok(())
}
