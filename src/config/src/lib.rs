#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rustdoc)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod config;

pub use config::Config;
