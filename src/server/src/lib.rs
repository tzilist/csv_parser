//! This is server initializer, the router, and handlers
//! Error conversion should happen in this crate

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rustdoc)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

pub mod app;
mod errors;
mod parser;
