//! Clonable comparable time keeper

use core::cmp::{Ordering, PartialEq, PartialOrd};
use core::fmt::{self, Debug, Display};

/// A time keeper.
///
/// This structure is used by the kernel to keep record of the current time
/// since boot, as well as for timer operations such as delays.
///
/// The global kernel timer can be cloned and then incremented, allowing
/// delay operations by then comparing the cloned and incremented timer to
/// the global timer.
#[derive(Copy, Clone, Eq, Ord)]
pub struct Timer {
	pub seconds: u64,
	pub microseconds: u32,
}

impl Timer {
	/// Create a new timer with it's values set to zero.
	pub const fn new() -> Timer {
		Timer {
			seconds: 0,
			microseconds: 0,
		}
	}

	/// Increment the timer by the given number of microseconds.
	///
	/// This operation wraps the internal `microseconds` value, keeping the
	/// internal `seconds` value accurate.
	pub fn increment(&mut self, microseconds: u32) {
		self.microseconds += microseconds;
		while self.microseconds >= 1_000_000 {
			self.microseconds -= 1_000_000;
			self.seconds += 1;
		}
	}

	/// Increment the timer by the given number of milliseconds.
	///
	/// This operation wraps the internal `microseconds` value, keeping the
	/// internal `seconds` value accurate.
	pub fn increment_ms(&mut self, milliseconds: u32) {
		self.increment(milliseconds * 1000);
	}

	/// Returns this timer's total time in milliseconds.
	pub fn in_milliseconds(&self) -> u64 {
		(self.seconds * 1000) + ((self.microseconds / 1000) as u64)
	}
}

impl Display for Timer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{:06}", self.seconds, self.microseconds)
	}
}

impl Debug for Timer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Timer({}.{:06})", self.seconds, self.microseconds)
	}
}

impl PartialOrd for Timer {
	fn partial_cmp(&self, other: &Timer) -> Option<Ordering> {
		if self.seconds == other.seconds {
			self.microseconds.partial_cmp(&other.microseconds)
		} else {
			self.seconds.partial_cmp(&other.seconds)
		}
	}
}

impl PartialEq for Timer {
	fn eq(&self, other: &Timer) -> bool {
		self.seconds == other.seconds && self.microseconds == other.microseconds
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::format;

	#[test]
	fn timer_increments_one_microsecond_correctly() {
		let mut timer = Timer::new();
		timer.increment(1);
		assert_eq!(timer.microseconds, 1);
	}

	#[test]
	fn timer_increments_one_millisecond_correctly() {
		let mut timer = Timer::new();
		timer.increment_ms(1);
		assert_eq!(timer.microseconds, 1000);
	}

	#[test]
	fn timer_increments_one_second_correctly() {
		let mut timer = Timer::new();
		timer.increment(1000000);
		assert_eq!(timer.seconds, 1);
		assert_eq!(timer.microseconds, 0);
	}

	#[test]
	fn timer_formats_one_millisecond_correctly() {
		let mut timer = Timer::new();
		timer.increment(1000);
		assert_eq!(&format!("{}", timer), "0.001000");
	}

	#[test]
	fn timer_formats_one_second_and_one_millisecond_correctly() {
		let mut timer = Timer::new();
		timer.increment(1001000);
		assert_eq!(&format!("{}", timer), "1.001000");
	}

	#[test]
	fn timer_partialeq_works() {
		let mut timer_one = Timer::new();
		timer_one.increment_ms(1000);
		let mut timer_two = Timer::new();
		timer_two.increment_ms(1000);

		assert_eq!(timer_one, timer_two);
	}

	#[test]
	fn timer_partialcmp_seconds_equal_microseconds_differ_works() {
		let mut timer_one = Timer::new();
		timer_one.increment_ms(1000);
		let mut timer_two = Timer::new();
		timer_two.increment_ms(1100);

		assert!(timer_one < timer_two);
	}

	#[test]
	fn timer_partialcmp_seconds_differ_works() {
		let mut timer_one = Timer::new();
		timer_one.increment_ms(1000);
		let mut timer_two = Timer::new();
		timer_two.increment_ms(2000);

		assert!(timer_one < timer_two);
	}
}
