use std::env;

use dotenv::dotenv;
use std::net::SocketAddr;
use structopt::StructOpt;
use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "csv_parser",
    about = "a csv to json parsing server",
    version = "0.1.0",
    author = "Ted Zilist",
    rename_all = "kebab-case"
)]
/// The global config for the application
/// All defaults, env variable names, and info should be stored here
pub struct Config {
    #[structopt(
        short,
        long,
        env = "PORT",
        default_value = "3000",
        help = "The port the server will listen on"
    )]
    /// The port the server listens on, defaults to "3000"
    pub port: u16,

    #[structopt(
        short,
        long,
        env = "HOST",
        default_value = "127.0.0.1",
        help = "The host to bind the server to"
    )]
    /// The host address to use for the server, defaults to "127.0.0.1"
    pub host: String,

    #[structopt(
        short = "l",
        long = "log-level",
        env = "LOG_LEVEL",
        default_value = "info",
        help = "Log level, can be trace, debug, info, warn, error, or off"
    )]
    /// The log level to use. Anything below the log level will be ignored
    /// defaults to "info"
    pub log_level: String,

    // #[structopt(
    // long,
    // env = "PSQL_URL",
    // help = "The PostgreSQL URI for the database to connect to"
    // )]
    // /// the PostgreSQL URI to use to connect to the database
    // pub psql_url: String,
    #[structopt(skip)]
    /// the host and port put together as {host}:{port}
    pub addr: Option<SocketAddr>,
}

impl Config {
    /// Initialize the config, setup tracing/log sinks
    pub fn init() -> Self {
        dotenv().ok();

        let mut config = Config::from_args();

        debug!("Parsed Configs: {:?}", config);

        // intiialize the tracing instance
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|e| {
            error!("Failed to intitialize tracing service. Panicking...");
            panic!(e);
        });

        Self::set_log_level();

        config.addr = Some(
            format!("{}:{}", &config.host, &config.port)
                .parse()
                .unwrap_or_else(|e| {
                    eprintln!(
                        "Invalid socket address given, {}:{}. Panicking...",
                        &config.host, &config.port
                    );
                    panic!(e)
                }),
        );

        config
    }

    /// set the log level of the server, can be "trace", "debug", "info", "warn", "error", or "off"
    /// The default value is set in the Config struct above
    fn set_log_level() {
        // ok to unwrap here and panic if the set OsString is not valid
        let set_env_log_level = env::var_os("LOG_LEVEL")
            .unwrap_or_else(|| "".into())
            .into_string()
            .unwrap();

        let log_level_filter = match set_env_log_level.as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            "off" => log::LevelFilter::Off,
            _ => {
                println!("Could not match on log type, defaulting to \"INFO\"");
                log::LevelFilter::Info
            }
        };

        // ok to unwrap here and panic as something has gone wrong intializing our log sink
        LogTracer::init_with_filter(log_level_filter).unwrap();
    }
}
