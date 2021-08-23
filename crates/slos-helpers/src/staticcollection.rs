//! A fixed-size collection type

use array_init::array_init;
use core::default::Default;
use core::fmt::{self, Debug};

/// Maximum number of entries a [`StaticCollection`] can hold.
pub const STATIC_COLLECTION_SIZE: usize = 128;

/// A fixed-size collection.
pub struct StaticCollection<T: Default> {
	/// Objects in this collection.
	entries: [T; STATIC_COLLECTION_SIZE],

	/// Index of the next entry in the `entries` array.
	next_entry: usize,
}

impl<T: Default> StaticCollection<T> {
	/// Create a new empty `StaticCollection`.
	pub fn new() -> Self {
		Self {
			entries: array_init(|_| Default::default()),
			next_entry: 0,
		}
	}

	/// Add an entry to this collection.
	pub fn push(&mut self, entry: T) {
		assert!(self.next_entry < STATIC_COLLECTION_SIZE);
		self.entries[self.next_entry] = entry;
		self.next_entry += 1;
	}

	/// Return a slice of the entries within this collection.
	pub fn as_slice(&self) -> &[T] {
		&self.entries[0..self.next_entry]
	}
}

impl<T: Default> Default for StaticCollection<T> {
	fn default() -> StaticCollection<T> {
		Self::new()
	}
}

impl<T: Default> Debug for StaticCollection<T>
where
	T: Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("StaticCollection")
			.field(&self.as_slice())
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::prelude::v1::*;

	#[test]
	fn test_as_slice() {
		let mut container: StaticCollection<&'static str> = StaticCollection::new();
		container.push("test one");
		container.push("test two");

		assert_eq!(container.as_slice(), &["test one", "test two"]);
	}
}
