use std::{io::Error, fs::{self, remove_dir_all, ReadDir}};

use crate::{db, config, http::row_methods::Bunch};

pub fn get_table(db: &str, name: &str) -> Result<Vec<String>, String> {
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

pub fn get_table_with_keys(db: &str, name: &str) -> Result<Vec<Bunch>, String> {
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

            let mut row_data: Vec<Bunch> = Vec::with_capacity(rows.len());

            for r in rows.iter() {
                match db::row::read_row(db, name, r) {
                    Ok(data) => {
                        let one: Bunch = Bunch {
                            key: r.to_string(),
                            value: data.value().into(),
                            _type: data.type_().to_string(),
                        };
                        row_data.push(one)
                    },
                    Err(_) => {}
                }
            }

            Ok(row_data)
        },
        Err(_) => return Err("{\"code\": 400, \"message\": \"no such table\"}\njson".to_string())
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

    let status: Result<(), Error> = remove_dir_all(&format!("{}/{}/{}", db_path, db, name));
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