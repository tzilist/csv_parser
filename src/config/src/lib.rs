//! Library is used to create and initialize all configs
//! This app follows the 12 factor app structure and looks for a `.env` file first
//! This is superceded by environment variables, followed by passed arguments to the binary
//! # Example
//! ```ignore
//! use config::Config;
//!
//! let config = Config::init();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rustdoc)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod config;

pub use crate::config::Config;
