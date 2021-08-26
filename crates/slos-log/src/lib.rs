#![cfg_attr(not(feature = "std"), no_std)]

extern crate log;
#[doc(hidden)]
pub use log as logcrate;

#[macro_use]
mod macros;
