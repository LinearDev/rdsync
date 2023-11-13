// use once_cell::sync::Lazy;

use crate::db::{self, row};
use crate::protos::row::Row;
use std::{collections::HashMap, sync::{Mutex, MutexGuard}, time::{SystemTime, UNIX_EPOCH}};
use lazy_static::lazy_static;

pub struct Cache {
    data: HashMap<String, Row>,
}

struct CacheKey {
    db: String,
    table: String,
    key: String
}

impl Cache {
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    fn get(&self, key: &str) -> Option<&Row> {
        self.data.get(key)
    }

    fn insert(&mut self, key: String, event: Row) {
        self.data.insert(key, event);
    }

    // fn insert_many(&mut self, keys: &[String]) {
    //     for key in keys {
    //         let event_data = db::get_data_on(key.clone());
    //         self.insert(key.clone(), event_data);
    //     }
    // }

    fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    fn remove_one(&mut self, key: &str) -> Option<Row> {
        self.data.remove(key)
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

// static mut CACHE: Cache = Cache::new();
lazy_static! {
    static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new());
}

/**
 * Add new row in cache table and in file db
 */
pub fn add(db: &str, table: &str, key: &str, value: &str) -> bool {
    let mut cache = CACHE.lock().unwrap();
    let dat = cache.get(&to_cache_string(db, table, key));

    if let Some(_r) = dat {
        return false;
    }
    
    let mut row = Row::new();
    row.set_value(value.to_string());
    let row_type = row::detect_str_type(value);
    row.set_type(row_type.to_string());

    cache.insert(to_cache_string(db, table, key).to_string(), row.clone());
    row::add_row(db, table, key, value);

    return true;
}

/**
 * Gets row from cache table
 */
pub fn get(db: &str, table: &str, key: &str) -> Result<Row, String> {
    let mut cache = CACHE.lock().unwrap();
    let data = cache.get(&to_cache_string(db, table, key));

    if let Some(r) = data {
        return Ok(r.clone())
    }

    let row = row::read_row(db, table, key);

    match row {
        Ok(r) => {
            cache.insert(to_cache_string(db, table, key), r.clone());
            return Ok(r)
        },
        Err(_) => return Err("[ ERROR ] Data not exist".to_string())
    }
}

/**
 * Deletes row from cache and from file db
 */
pub fn delete(db: &str, table: &str, key: &str) {
    CACHE.lock().unwrap().remove_one(&to_cache_string(db, table, key));
    row::delete_row(db, table, key);
}

pub fn keys() -> Vec<String> {
    return CACHE.lock().unwrap().keys();
}

pub fn clear() {
    CACHE.lock().unwrap().clear();
}

fn to_cache_string(db: &str, table: &str, key: &str) -> String {
    return format!("{}|rdb|{}|rdb|{}", db, table, key);
}

fn from_cache_string(cache_string: String) -> CacheKey {
    let mut key = CacheKey {
        db: "".to_string(),
        table: "".to_string(),
        key: "".to_string(),
    };

    let cs_split: Vec<&str> = cache_string.split("|rdb|").collect();

    key.db = cs_split[0].to_string();
    key.table = cs_split[1].to_string();
    key.key = cs_split[2].to_string();

    return key;
}

// pub fn insert_proto(event: Row) {
//     CACHE.lock().unwrap().insert(event.id().to_string(), event.clone());
//     let d_event: db::Row = db::Row{
//         id: event.id(),
//         name: event.name().to_string(),
//         stopped: event.stopped(),
//         every: event.every(),
//         script_type: event.script_type().to_string(),
//         last_schedule: event.last_schedule(),
//         command: event.command().to_string()
//     };
//     db::add(d_event);
// }

// pub fn edit(event: Row) {
//     let mut cache: MutexGuard<Cache> = CACHE.lock().unwrap();
//     if let Some(mut event_to_edit) = cache.remove_one(&event.id().to_string()) {
//         event_to_edit.set_last_schedule(event.last_schedule());

//         cache.insert(event_to_edit.id().to_string(), event_to_edit);
//     }
// }

// pub fn get_data_on(key: String) -> Row {
//     if let Some(data) = CACHE.lock().unwrap().get(&key.to_string()).cloned() {
//         return data;
//     } else {
//         let event_db: Row = db::get_data_on(key);
//         insert_proto(event_db.clone());
//         return event_db;
//     }
// }

// fn check_on_data() {
//     let keys = db::keys();
//     if !keys.is_empty() {
//         println!("INFO: Cache found keys in DB. Amount {}", keys.len());
//         let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
//         CACHE.lock().unwrap().insert_many(&keys);
//         let time_now: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
//         println!("INFO: Cache successfully imported keys from DB in {}ms", time_now - time);
//     }
// }