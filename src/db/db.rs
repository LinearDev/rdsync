use std::{fs::remove_dir_all, io::Error};

use crate::config;
use crate::db;

pub fn delete(name: &str) -> Result<String, String> {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(name) {
        return Err("{\"code\": 400, \"message\": \"DataBase not exist\"}".to_string());
    }

    let status: Result<(), Error> = remove_dir_all(&format!("{}/{}", db_path, name));
    match status {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err("".to_string()),
    }
}