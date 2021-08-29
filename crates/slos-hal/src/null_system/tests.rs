use super::{NullConsole, NullSystem};
use std::prelude::v1::*;

#[test]
fn console_normal_read_succeeds() {
	use slos_filesystem::FsFileHandle;

	let mut console = NullConsole;
	assert!(console.raw_read(0, None).is_ok());
}

#[test]
fn console_nonsensical_read_succeeds() {
	use slos_filesystem::FsFileHandle;

	let mut console = NullConsole;
	assert!(console.raw_read(usize::MAX, Some(usize::MAX)).is_ok());
}

#[test]
fn console_normal_write_succeeds() {
	use slos_filesystem::FsFileHandle;

	let mut console = NullConsole;
	assert!(console.raw_write(0, b"hello world!").is_ok());
}

#[test]
fn console_nonsensical_write_succeeds() {
	use slos_filesystem::FsFileHandle;

	let mut console = NullConsole;
	assert!(console
		.raw_write(usize::MAX, b"[insert muffled screaming]")
		.is_ok());
}

#[test]
fn system_requests_return_after_iter() {
	use crate::SystemKmainHooks;

	let mut system: NullSystem = Default::default();
	assert_eq!(system.has_requested_return, false);

	// Call the trait methods that kmain would call each iteration
	system.hook_kmain_loop_head();
	system.hook_kmain_loop_inner_part();

	// Check we've got the flag
	assert_eq!(system.has_requested_return, true);
}

#[test]
fn system_virtualization_enabled() {
	use crate::SystemHardware;

	let mut system: NullSystem = Default::default();

	// Set the virtualization flag
	system.is_virtualized = true;
	assert!(system.virtualization().is_some());
}

#[test]
fn system_virtualization_disabled() {
	use crate::SystemHardware;

	let mut system: NullSystem = Default::default();

	// Clear the virtualization flag
	system.is_virtualized = false;
	assert!(system.virtualization().is_none());
}
