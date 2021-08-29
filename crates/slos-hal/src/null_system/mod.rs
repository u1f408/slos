//! Placeholder no-op HAL implementation

use core::default::Default;
use lazy_static::lazy_static;
use slos_helpers::UnsafeContainer;

use crate::{SystemConsole, SystemCpu, SystemHardware, SystemKmainHooks};

mod console;
pub use self::console::{NullConsole, NULL_CONSOLE};

#[cfg(test)]
mod tests;

lazy_static! {
	/// Global instance of the [`NullSystem`]
	pub static ref SYSTEM: UnsafeContainer<NullSystem> = UnsafeContainer::new(Default::default());
}

/// A [`SystemHardware`] implementation where (almost) everything has no-op
/// implementations
///
/// # Behaviour
///
/// The following general rules apply:
///
/// Methods on HAL traits that return `&'static mut T` references to other HAL
/// trait objects return references to valid things, but those things are also
/// all no-op implementations (for example, [`NullConsole`]).
///
/// All methods that do not return a value are implemented as empty functions.
///
/// All methods that return a value will return either (a) a "sensible" value,
/// in the case where the value is critical to the kernel, (b) the same value
/// that [`Default::default`] would return, where `Default` is implemented for
/// the type, or (c) a manually-constructed default value for the type that
/// will make any operations on that type into no-ops.
///
/// ## Exceptions to the rules
///
/// - [`SystemHardware::system_name`] returns "slos-hal-nullsystem".
///
/// - [`SystemHardware::has_requested_return`] will return the value of the
///   `has_requested_return` field of the struct - this field is set to
///   `false` in the constructor, and set to `true` on every call to the
///   [`SystemKmainHooks::hook_kmain_loop_head`] method. This is done so that
///   `kmain` will iterate exactly **once** before returning.
///
/// - If the `is_virtualized` field of this struct is set to `true`, the
///   [`SystemHardware::virtualization`] method will return a value indicating
///   that the system is virtualized (the "virtualization type" field of the
///   value is set to [`SystemHardware::system_name`]). If the field is set to
///   `false`, [`None`][Option::None] is returned.
///
/// # Default values
///
/// This struct implements the [`Default`] trait, these are the values that
/// are set in that constructor:
///
/// ```
/// # use slos_hal::null_system::NullSystem;
/// # let sys: NullSystem = {
/// NullSystem {
///     has_requested_return: false,
///     is_virtualized: true,
/// }
/// # };
/// #
/// # // let's check that the documentation is actually correct, huh
/// # assert_eq!(sys, core::default::Default::default());
/// ```
///
/// # Safety
///
/// This implementation will never panic directly.
#[derive(Debug, PartialEq)]
pub struct NullSystem {
	/// Whether to request a `kmain` return
	pub has_requested_return: bool,

	/// Whether to indicate the system is virtualized
	pub is_virtualized: bool,
}

impl Default for NullSystem {
	fn default() -> Self {
		Self {
			has_requested_return: false,
			is_virtualized: true,
		}
	}
}

impl SystemCpu for NullSystem {
	fn interrupts_disable(&mut self) {}
	fn interrupts_enable(&mut self) {}
	fn halt(&mut self) {}

	fn interrupts_are_enabled(&self) -> bool {
		false
	}
}

impl SystemKmainHooks for NullSystem {
	fn hook_kmain_loop_head(&mut self) {
		self.has_requested_return = true;
	}
}

impl SystemHardware for NullSystem {
	fn system_name(&self) -> &'static str {
		concat!(env!("CARGO_PKG_NAME"), "-nullsystem")
	}

	fn console(&mut self) -> &'static mut dyn SystemConsole {
		NULL_CONSOLE.get()
	}

	fn has_requested_return(&self) -> bool {
		self.has_requested_return
	}

	fn current_cpu(&mut self) -> &'static mut dyn SystemCpu {
		SYSTEM.get()
	}

	fn virtualization(&self) -> Option<(&'static str, ())> {
		match self.is_virtualized {
			true => Some((self.system_name(), ())),
			false => None,
		}
	}
}
