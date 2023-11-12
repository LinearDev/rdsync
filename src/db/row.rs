use crate::protos::row::Row;
use crate::config;
use crate::db::{self, table};

use std::fs::{OpenOptions, File};
use protobuf::Message;
use serde_json::Value;

fn detect_str_type(input: &str) -> &str {
    match serde_json::from_str::<Value>(input) {
        Ok(_) => "json",
        Err(_) => "string",
    }
}

#[test]
fn add_row_test() {
    assert_eq!(add_row("test_db", "sass", "test", "ar"), true);
}

pub fn add_row(db: &str, table: &str, key: &str, value: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        db::create_db(db);
    }

    if !table::is_table_exist(db, table) {
        table::create_table(db, table);
    }

    let file_path: &String = &format!("{}/{}/{}/{}", db_path, db, table, key);
    let mut file: File = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .read(true)
        .open(file_path)
        .expect(&format!("[ ERROR ]: Can't open key - {}", key));

    let mut proto: Row = Row::new();
    proto.set_value(value.to_string());
    proto.set_type(detect_str_type(value).to_string());

    let res = proto.write_to_writer(&mut file);

    match res {
        Ok(_) => {
            println!("[ INFO ]: Imported new key - {}", key);
            return true;
        },
        Err(error) => {
            println!("[ ERROR ] Row: Can't add new key - {}. Reason:  {}", key, error);
            return  false;
        },
    }
}

// pub fn is_row_exist() {

// }

// pub fn detect_row_type() {
    
// }