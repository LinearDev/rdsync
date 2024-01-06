use std::{fs::remove_dir_all, io::Error};

use crate::config;
use crate::db;

/// Deletes a database and its associated directory from the file system.
///
/// # Arguments
///
/// * `name` - Name of the database to be deleted.
///
/// # Returns
///
/// Returns a Result indicating the status of the database deletion operation.
/// - If the deletion is successful, it returns an empty string.
/// - If the database does not exist, it returns an error message indicating that the database doesn't exist.
/// - If an error occurs during the deletion, it returns an empty string.
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