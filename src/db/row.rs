use std::{fs::{self, remove_file, OpenOptions, File}, io::{self, Read}};
use protobuf::{Message, Error };
use serde_json::Value;

use crate::{db::{self, table}, protos::row::Row, config};

pub fn detect_str_type(input: &str) -> &str {
    match serde_json::from_str::<Value>(input) {
        Ok(_) => "json",
        Err(_) => "string",
    }
}
/**
 * Add new row in table
 */
pub fn add_row(db: &str, table: &str, key: &str, row: &mut Row) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        db::create_db(db);
    }

    if !table::is_table_exist(db, table) {
        table::create_table(db, table);
    }

    let file_path: &String = &format!("{}/{}/{}/{}.el", db_path, db, table, key);
    let mut file: File = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .read(true)
        .open(&file_path)
        .expect(&format!("[ ERROR ]: Can't open key - {}", key));

    match row.write_to_writer(&mut file) {
        Ok(_) => {
            println!("[ INFO ]: Imported new key - {}", key);
            true
        }
        Err(error) => {
            println!("[ ERROR ] Row: Can't add new key - {}. Reason:  {}", key, error);
            false
        }
    }
}

/**
 * Return data at the row
 */
pub fn read_row(db: &str, table: &str, key: &str) -> Result<Row, String> {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        return Err("[ ERROR ] Row: DB is not exist".to_string())
    }

    if !table::is_table_exist(db, table) {
        return Err("[ ERROR ] Row: Table is not exist".to_string())
    }

    let file_path: &String = &format!("{}/{}/{}/{}.el", db_path, db, table, key);
    let file_res: Result<File, std::io::Error> = OpenOptions::new()
        .read(true)
        .open(file_path);

    let mut file: fs::File;
    match file_res {
        Ok(f) => file = f,
        Err(_) => return Err("0".to_string())
    }

    let mut proto: Row = Row::new();

    let mut cont: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len().try_into().unwrap());
    file.read_to_end(&mut cont).expect("[ ERROR ] Row: Can not read key");

    let res: Result<(), Error> = proto.merge_from_bytes(&cont);

    match res {
        Ok(_) => return Ok(proto),
        Err(_) => return Err("[ ERROR ] Row: Can not merge from bytes".to_string())
    };
}

/**
 * Delete one row
 */
pub fn delete_row(db: &str, table: &str, key: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    let exist: Result<(), String> = is_row_exist(db, table, key);

    match exist {
        Ok(_) => {}
        Err(_) => return false
    }

    let res: Result<(), io::Error> = remove_file(&format!("{}/{}/{}/{}.el", db_path, db, table, key));
    match res {
        Ok(_) => return true,
        Err(_) => return false
    }
}

/**
 * Check if row is exist
 */
pub fn is_row_exist(db: &str, table: &str, key: &str) -> Result<(), String> {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        return Err("[ ERROR ] Row: DB is not exist".to_string())
    }

    if !table::is_table_exist(db, table) {
        return Err("[ ERROR ] Row: Table is not exist".to_string())
    }

    match fs::metadata(&format!("{}/{}/{}/{}.el", db_path, db, table, key)) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("[ ERROR ] Row: is not exist".to_string())
    }
}