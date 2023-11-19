use std::fs;
use toml::Value;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Config {
    pub db_path: String,
    pub port: u16
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
}

lazy_static! {
    pub static ref CONFIG: Config = read_config().unwrap_or_else(|e| {
        eprintln!("Error reading configuration: {:?}", e);
        Default::default()
    });
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            db_path: "".to_string(),
            port: 3000
        }
    }
}

pub fn read_config() -> Result<Config, ConfigError> {
    let mut conf = Config::default();

    let toml_str = fs::read_to_string("config.toml").map_err(ConfigError::IoError)?;

    let toml_value: Value = toml::from_str(&toml_str).map_err(ConfigError::TomlError)?;

    if let Some(patn) = toml_value["NAME"].as_str() {
        conf.db_path = patn.to_string();
    }

    if let Some(port) = toml_value["PORT"].as_integer() {
        conf.port = port.try_into().unwrap();
    }

    Ok(conf)
}
