use std::{env, fs::File};

use once_cell::sync::OnceCell;
use serde::Deserialize;

const APP_CONFIG: &str = "APP_CONFIG";

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize)]
pub struct Config {
    /// Enable custom logging; acceptable directive must follow the following
    /// form https://docs.rs/env_logger/0.8.3/env_logger/index.html#enabling-logging.
    pub logging_directive: String,

    /// The IPv4 address to listen to.
    pub address: String,

    /// The tcp port to bind to.
    pub port: u16,

    /// The maximum size of an acceptable http request payload before responding
    /// with an error.
    pub max_payload_size_bytes: usize,

    /// The number of seconds for workers to shutdown gracefully.
    pub graceful_shutdown_timeout_sec: u64,

    /// The environmental variable name holding the Postgres connection uri.
    pub postgres_uri_env_var_name: String,

    /// The maximum number of active postgres connections to pool.
    pub postgres_max_pool_size: u32,

    /// The minimum number of active postgres connections to maintain in the pool.
    pub postgres_min_pool_size: u32,

    // The maximum lifetime of a connection.
    pub postgres_max_life_time_minutes: u64,

    /// The Postgres connection timeout in seconds.
    pub postgres_timeout_sec: u64,
}

fn init() -> Config {
    let path = env::var(APP_CONFIG)
        .unwrap_or_else(|_| panic!("Environmental variable {} could not be found.", APP_CONFIG));

    let file = File::open(&path)
        .unwrap_or_else(|_| panic!("Could not open configuration file located at: {}", path));

    ron::de::from_reader::<File, Config>(file)
        .unwrap_or_else(|error| panic!("Could not parse configuration file: {}", error))
}

/// Returns the app config.
pub fn app_config() -> &'static Config {
    CONFIG.get_or_init(init)
}
