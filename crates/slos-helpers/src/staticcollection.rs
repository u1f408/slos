//! A fixed-size collection type

use array_init::array_init;
use core::default::Default;
use core::fmt::{self, Debug};
use core::ops::Deref;

const STATIC_COLLECTION_SIZE: usize = 128;

/// A fixed-size collection.
pub struct StaticCollection<T: Default> {
	/// Objects in this collection.
	entries: [T; STATIC_COLLECTION_SIZE],

	/// Index of the next entry in the `entries` array.
	next_entry: usize,
}

impl<T: Default> StaticCollection<T> {
	/// Maximum number of entries the collection can hold.
	pub const MAX_SIZE: usize = STATIC_COLLECTION_SIZE;

	/// Create a new empty `StaticCollection`.
	pub fn new() -> Self {
		Self {
			entries: array_init(|_| Default::default()),
			next_entry: 0,
		}
	}

	/// Returns the number of elements in the collection.
	pub fn len(&self) -> usize {
		self.next_entry
	}

	/// Appends an element to the tail end of the collection.
	pub fn push(&mut self, entry: T) {
		assert!(self.next_entry < Self::MAX_SIZE);
		self.entries[self.next_entry] = entry;
		self.next_entry += 1;
	}

	/// Return a slice of the entries within this collection.
	pub fn as_slice(&self) -> &[T] {
		&self.entries[0..self.next_entry]
	}

	/// Return a mutable slice of the entries within this collection.
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.entries[0..self.next_entry]
	}
}

impl<T: Default> Deref for StaticCollection<T> {
	type Target = [T];
	fn deref<'a>(&'a self) -> &'a [T] {
		self.as_slice()
	}
}

impl<T: Default> Default for StaticCollection<T> {
	fn default() -> StaticCollection<T> {
		Self::new()
	}
}

impl<T: Default> FromIterator<T> for StaticCollection<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut c = Self::new();
		for i in iter {
			c.push(i);
		}

		c
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
	#[allow(unused_imports)]
	use alloc::prelude::v1::*;

	#[test]
	fn test_as_slice() {
		let mut container: StaticCollection<&'static str> = StaticCollection::new();
		container.push("test one");
		container.push("test two");

		assert_eq!(container.as_slice(), &["test one", "test two"]);
	}

	#[test]
	fn test_deref() {
		let mut container: StaticCollection<&'static str> = StaticCollection::new();
		container.push("test one");
		container.push("test two");

		assert_eq!(container.deref(), &["test one", "test two"]);
	}

	#[test]
	fn test_from_iterator() {
		let mut container: Vec<&'static str> = Vec::new();
		container.push("test one");
		container.push("test two");

		let c = container
			.iter()
			.map(|x| *x)
			.collect::<StaticCollection<&'static str>>();
		assert_eq!(c.as_slice(), &["test one", "test two"]);
	}
}
