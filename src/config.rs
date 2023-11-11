use std::fs;
use toml::Value;
use lazy_static::lazy_static;

pub struct Config {
    db_path: String
}

lazy_static! {
    pub static ref CONFIG: Config = read_config().0;
}

pub fn read_config() -> (Config, bool) {
    let mut conf: Config = Config { db_path: "".to_string() };
     // Read the contents of the TOML file into a string
     let toml_str = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return (conf, false);
        }
    };

    // Parse the TOML string into a `toml::Value`
    let toml_value: Value = match toml::from_str(&toml_str) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error parsing TOML: {}", e);
            return (conf, false);
        }
    };

    // Access specific values in the TOML structure
    if let Some(patn) = toml_value["NAME"].as_str() {
        conf.db_path = patn.to_string();
    }

    return (conf, true);
}