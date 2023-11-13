pub mod table;
pub mod row;

use crate::config;
use std::{fs, path::Path};

/**
 * Initialize directory on patch
 */
fn init_dir(path: &str) -> bool {
    let dir = Path::new(&path);
    if !dir.exists() {
        let result = fs::create_dir(path);
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

/**
 * Checks if directory exist
 */
#[test]
fn is_dir_exist_test() {
    assert_eq!(is_dir_exist("table"), false);
}

fn is_dir_exist(path: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    let format_path = format!("{}/{}", db_path, path);
    let dir = Path::new(&format_path);
    return dir.exists();
}

/*
 * inits db folder
 */
pub fn init() -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    return init_dir(db_path);
}

#[test]
fn create_db_test() {
    // assert_eq!(create_db("test_db"), true);
    assert_eq!(create_db("new_test_db"), true);
}

/**
 * Creates DB
 */
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

#[test]
fn is_db_exist_test() {
    assert_eq!(is_db_exist("zaz"), false);
}
/**
 * Checks if DB exist
 */
pub fn is_db_exist(name: &str) -> bool {
    return is_dir_exist(name);
}