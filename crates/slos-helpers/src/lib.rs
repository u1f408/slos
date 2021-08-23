#![no_std]
#![feature(alloc_prelude)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[allow(unused_imports)]
use alloc::prelude::v1 as alloc_prelude;

pub mod unsafecontainer;
pub use self::unsafecontainer::UnsafeContainer;
pub mod staticcollection;
pub use self::staticcollection::StaticCollection;
