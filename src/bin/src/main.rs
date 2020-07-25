//! TODO

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rustdoc)]
#![warn(rust_2018_idioms)]

use config::Config;

#[actix_rt::main]
async fn main() {
    let _config = Config::init();

    println!("Hello");
}
