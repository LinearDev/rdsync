pub mod cache_table;

use crate::db::{row, table};
use crate::protos::row::Row;
use crate::cache::cache_table::Cache;

use std::{sync::Mutex, time::{SystemTime, UNIX_EPOCH}};
use lazy_static::lazy_static;

use self::cache_table::TimeCache;

struct CacheKey {
    db: String,
    table: String,
    key: String
}

// static mut CACHE: Cache = Cache::new();
lazy_static! {
    pub  static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new(10));
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

    let cache_key = to_cache_string(db, table, key).to_string();
    cache.insert(cache_key.clone(), row.clone());

    //update time when updated
    cache.safe_time_insert(&cache_key, TimeCache {
        last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
        data_length: value.len()
    });
    
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
            let cache_key = to_cache_string(db, table, key);
            cache.insert(cache_key.clone(), r.clone());
            cache.update_last_accessed(&cache_key);

            return Ok(r)
        },
        Err(_) => return Err("[ ERROR ] Data not exist".to_string())
    }
}

/**
 * Deletes row from cache and from file db
 */
pub fn delete(db: &str, table: &str, key: &str) -> Result<String, String> {
    let mut cache = CACHE.lock().unwrap();
    let cache_key = to_cache_string(db, table, key);
    
    cache.data.remove(&cache_key);
    cache.time_data.remove(&cache_key);

    let status = row::delete_row(db, table, key);
    if status {
        return Ok(format!("Row with key {} was deleted", key.to_string()));
    } else {
        return Err(format!("Cant delete row with key {}", key.to_string()));
    }
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

pub fn delete_table(db: &str, name: &str) -> bool {
    let status = table::delete_table(db, name);
    if !status {
        return false;
    }

    let mut cache = CACHE.lock().unwrap();
    let keys_to_delete: Vec<String> = cache
        .time_data
        .keys()
        .filter(|key| {
            let data = from_cache_string(key.to_string());
            data.db == db && data.table == name
        })
        .cloned()
        .collect();

    for key in keys_to_delete {
        cache.delete(&key);
    }

    return true;
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