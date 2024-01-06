//! Database file operations module.
pub mod table;

/// Module for database row operations.
pub mod row;

/// Module for database operations.
pub mod db;

use std::{fs::{self, ReadDir}, io::Error, path::Path};

use crate::config;

/// Initializes a directory at the specified path if it does not exist.
///
/// # Arguments
///
/// * `path` - Path of the directory to be initialized.
///
/// # Returns
///
/// Returns true if the directory is successfully created or already exists; false otherwise.
fn init_dir(path: &str) -> bool {
    let dir: &Path = Path::new(&path);
    if !dir.exists() {
        let result: Result<(), Error> = fs::create_dir(path);
        match result {
            Ok(_) => {
                println!("[ INFO ] DB: dir created");
                return true;
            },
            Err(error) => {
                println!("[ ERROR ] DB: directory not created - {}", error);
                return false;
            },
        }
    }

    return true;
}

/// Test for checking if a directory exists.
#[test]
fn is_dir_exist_test() {
    assert_eq!(is_dir_exist("table"), false);
}

/// Checks if a directory exists at the specified path.
///
/// # Arguments
///
/// * `path` - Path of the directory to be checked.
///
/// # Returns
///
/// Returns true if the directory exists; false otherwise.
fn is_dir_exist(path: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    let format_path: String = format!("{}/{}", db_path, path);
    let dir: &Path = Path::new(&format_path);
    return dir.exists();
}

/// Initializes the database directory based on the configured path.
///
/// # Returns
///
/// Returns true if the initialization is successful; false otherwise.
pub fn init() -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    return init_dir(db_path);
}

/// Test for creating a new database.
#[test]
fn create_db_test() {
    // assert_eq!(create_db("test_db"), true);
    assert_eq!(create_db("new_test_db"), true);
}

/// Creates a new database with the specified name.
///
/// # Arguments
///
/// * `name` - Name of the database to be created.
///
/// # Returns
///
/// Returns true if the database is successfully created; false if it already exists.
pub fn create_db(name: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;
    if !Path::new(db_path).exists() {
        init();
    }

    if is_dir_exist(name) {
        return false;
    }

    return init_dir(&format!("{}/{}", db_path, name));
}

/// Retrieves a list of rows within a specified database.
///
/// # Arguments
///
/// * `db` - Database name.
/// * `name` - Table name.
///
/// # Returns
///
/// Returns a Result containing a vector of row names if successful, or an error message if unsuccessful.
pub fn get_db(db: &str, name: &str) -> Result<Vec<String>, String> {
    let db_path: &str = &config::CONFIG.db_path;

    let data: Result<ReadDir, Error> = fs::read_dir(format!("{}/{}/{}", db_path, db, name));

    match data {
        Ok(dir) => {
            let rows: Vec<String> = dir
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.file_name().to_str().map(String::from))
                    .map(|s| s[..s.len() - 3].to_string()) // Remove the last 3 characters
            })
            .collect();

            Ok(rows)
        },
        Err(err) => return Err(err.to_string())
    }
}

/// Test for checking if a database exists.
#[test]
fn is_db_exist_test() {
    assert_eq!(is_db_exist("zaz"), false);
}

/// Checks if a database exists with the specified name.
///
/// # Arguments
///
/// * `name` - Name of the database to be checked.
///
/// # Returns
///
/// Returns true if the database exists; false otherwise.
pub fn is_db_exist(name: &str) -> bool {
    return is_dir_exist(name);
}