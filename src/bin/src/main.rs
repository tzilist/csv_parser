//! TODO

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rustdoc)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use std::sync::Arc;

use config::Config;
use server::app::{self, State};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // initialize the config
    let config = Config::init();

    // create our state
    let state = Arc::new(State { config });

    info!("Starting server...");
    let status = app::start(state).await;

    // see ya later, alligator
    info!("Bye :)");

    status
}
