use std::env;

use dotenv::dotenv;
use std::net::SocketAddr;
use structopt::StructOpt;
use tracing::Level;
use tracing_subscriber;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "csv_parser",
    about = "a csv to json parsing server",
    version = "0.1.0",
    author = "Ted Zilist",
    rename_all = "kebab-case"
)]
pub struct Config {
    #[structopt(
        short,
        long,
        env = "PORT",
        default_value = "3000",
        help = "The port the server will listen on"
    )]
    pub port: u16,

    #[structopt(
        short,
        long,
        env = "HOST",
        default_value = "127.0.0.1",
        help = "The host to bind the server to"
    )]
    pub host: String,

    #[structopt(
        short = "l",
        long = "log-level",
        env = "LOG_LEVEL",
        default_value = "info",
        help = "Log level, can be trace, debug, info, warn, or error"
    )]
    pub log_level: String,

    #[structopt(
        long,
        env = "PSQL_URL",
        help = "The PostgreSQL URL for the database to connect to"
    )]
    pub psql_url: String,

    #[structopt(skip)]
    pub addr: Option<SocketAddr>,
}
