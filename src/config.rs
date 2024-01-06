//! Database startup configuration module

use std::fs;
use toml::Value;
use lazy_static::lazy_static;

// Struct to hold configuration values
#[derive(Debug)]
pub struct Config {
    /// Database path
    pub db_path: String,

    /// Server IP address
    pub ip: String,

    /// Server port
    pub port: u16,

    /// Cache size (in MB)
    pub cache_size: u16,

    /// Thread pool size
    pub workers_count: u16
}

/// Enum to handle configuration-related errors
#[derive(Debug)]
pub enum ConfigError {
    /// IO error
    IoError(std::io::Error),

    /// TOML parsing error
    TomlError(toml::de::Error),
}

/// Lazy static instance of the configuration
lazy_static! {
    /// Global application configuration
    pub static ref CONFIG: Config = read_config().unwrap_or_else(|e| {
        eprintln!("Error reading configuration: {:?}", e);
        Default::default()
    });
}

/// Default implementation for Config
impl Default for Config {
    fn default() -> Self {
        Config { 
            db_path: "db".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 3000,
            cache_size: 10,
            workers_count: 4
        }
    }
}

/// Function to read the configuration from a TOML file
pub fn read_config() -> Result<Config, ConfigError> {
    let mut conf: Config = Config::default();

    let toml_str: String = fs::read_to_string("config.toml").map_err(ConfigError::IoError)?;

    let toml_value: Value = toml::from_str(&toml_str).map_err(ConfigError::TomlError)?;

    if let Some(patn) = toml_value["NAME"].as_str() {
        conf.db_path = patn.to_string();
    }

    if let Some(ip) = toml_value["IP"].as_str() {
        conf.ip = ip.try_into().unwrap();
    }

    if let Some(port) = toml_value["PORT"].as_integer() {
        conf.port = port.try_into().unwrap();
    }

    if let Some(cache_size) = toml_value["CACHE_SIZE"].as_integer() {
        conf.cache_size = cache_size.try_into().unwrap();
    }

    if let Some(workers_count) = toml_value["WORKERS_COUNT"].as_integer() {
        conf.workers_count = workers_count.try_into().unwrap();
    }

    Ok(conf)
}
