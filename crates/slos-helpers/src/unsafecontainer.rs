//! Static container type with interior mutability

use core::cell::UnsafeCell;
use core::marker::Sync;
use core::ops::Deref;
use core::fmt::{self, Debug};

/// Container usable as a static that allows getting a mutable reference
/// to it's interior value.
pub struct UnsafeContainer<T: ?Sized>(UnsafeCell<T>);

unsafe impl<T: ?Sized + Send> Sync for UnsafeContainer<T> {}
unsafe impl<T: ?Sized + Send> Send for UnsafeContainer<T> {}

impl<T> UnsafeContainer<T> {
    /// Create a new container.
    pub const fn new(t: T) -> UnsafeContainer<T> {
        UnsafeContainer(UnsafeCell::new(t))
    }

    /// Consume this container and return the interior value.
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    /// Get a mutable reference to the interior value.
    pub fn get<'a>(&self) -> &'a mut T {
        unsafe { &mut *self.0.get() }
    }
}

impl<T: ?Sized> Deref for UnsafeContainer<T> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.0.get() }
    }
}

impl<T> Default for UnsafeContainer<T>
where
    T: Default
{
    fn default() -> UnsafeContainer<T> {
        Self::new(Default::default())
    }
}

impl<T> Debug for UnsafeContainer<T>
where
    T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnsafeContainer")
            .field(self.get())
            .finish()
    }
}