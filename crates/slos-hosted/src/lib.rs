#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate slos_log;

pub mod hal;
#[cfg(feature = "repl")]
pub mod repl;
