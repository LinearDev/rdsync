use std::fs::remove_dir_all;

use crate::db;
use crate::config;

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