use std::fs;
use std::fs::remove_dir_all;

use crate::db;
use crate::config;

pub fn get_table(db: &str, name: &str) -> Result<Vec<String>, String> {
    let db_path: &str = &config::CONFIG.db_path;

    let data = fs::read_dir(format!("{}/{}/{}", db_path, db, name));

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

/**
 * Creates new table
 */
pub fn create_table(db: &str, name: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        db::create_db(db);
    }

    return db::init_dir(&format!("{}/{}/{}", db_path, db, name));
}

/**
 * Deletes one table
 */
pub fn delete_table(db: &str, name: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        db::create_db(db);
    }

    let status = remove_dir_all(&format!("{}/{}/{}", db_path, db, name));
    match status {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

/**
 * Checks if table exist
 */
pub fn is_table_exist(db: &str, name: &str) -> bool {
    return db::is_dir_exist(&format!("{}/{}", db, name));
}