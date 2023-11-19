use crate::protos::row::Row;

use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

pub struct TimeCache {
    pub last_accessed: u128,
    pub data_length: usize
}

pub struct Cache {
    pub data: HashMap<String, Row>,
    pub time_data: HashMap<String, TimeCache>,
    pub max_data_size: usize,
    pub current_data_size: usize,
    latest_time: u128,
    newest_time: u128
}

impl Cache {
    pub fn new(size_limit_mb: usize) -> Self {
        Self {
            data: HashMap::new(),
            time_data: HashMap::new(),
            max_data_size: size_limit_mb * 1024 * 1024,
            current_data_size: 0,
            latest_time: 0,
            newest_time: 0
        }
    }

    pub fn get(&self, key: &str) -> Option<&Row> {
        self.data.get(key)
    }

    pub fn insert(&mut self, key: String, event: Row) {
        self.data.insert(key, event);
    }

    // fn insert_many(&mut self, keys: &[String]) {
    //     for key in keys {
    //         let event_data = db::get_data_on(key.clone());
    //         self.insert(key.clone(), event_data);
    //     }
    // }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn safe_time_insert(&mut self, key: &str, time_h: TimeCache) {
        if self.current_data_size + time_h.data_length <= self.max_data_size {
            if self.latest_time == 0 {self.latest_time = time_h.last_accessed}
            self.time_data.insert(key.to_string(), time_h);
        } else {
            expand_cache(self);
            // Now you can insert the new data
            self.time_data.insert(key.to_string(), time_h);
        }
    }

    pub fn update_last_accessed(&mut self, key: &str) {
        if let Some(k) = self.time_data.get_mut(key) {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
            
            if self.latest_time == 0 {self.latest_time = time}
            self.newest_time = time;

            *k = TimeCache {
                last_accessed: time,
                data_length: k.data_length,
            };
        }
    }

    pub fn delete(&mut self, key: &str) {
        self.current_data_size -= self.time_data.get(key).unwrap().data_length;
        self.data.remove(key);
        self.time_data.remove(key);
    }
}

fn expand_cache(cache: &mut Cache) {
    let mut lowest_time: u128 = std::u128::MAX;
    let mut lowest_data_length: usize = 0;
    let mut lowest_key: String = "".to_string();

    for (key, value) in cache.time_data.iter() {
        if value.last_accessed < lowest_time {
            lowest_data_length = value.data_length;
            lowest_time = value.last_accessed;
            lowest_key = key.to_string();
        }
    }

    if lowest_time != std::u128::MAX {
        cache.current_data_size -= lowest_data_length;
        cache.data.remove(&lowest_key);
        cache.time_data.remove(&lowest_key);
    }
}