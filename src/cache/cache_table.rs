use crate::protos::row::Row;

use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

/// Represents time-related metadata for entries in the cache.
pub struct TimeCache {
    /// Timestamp of the last access to the cache entry in microseconds since the Unix epoch.
    pub last_accessed: u128,

    /// Length of the data associated with the cache entry.
    pub data_length: usize
}

/// Represents a cache storing rows and associated time metadata.
pub struct Cache {
    /// HashMap storing rows with cache keys as the identifier.
    pub data: HashMap<String, Row>,

    /// HashMap storing time-related metadata for cache entries.
    pub time_data: HashMap<String, TimeCache>,

    /// Maximum size limit for the total data size in the cache (in bytes).
    pub max_data_size: usize,

    /// Current size of the cached data.
    pub current_data_size: usize,

    /// Timestamp of the latest access in microseconds since the Unix epoch.
    latest_time: u128,

    /// Timestamp of the newest access in microseconds since the Unix epoch.
    newest_time: u128
}

impl Cache {
    /// Creates a new cache with a specified size limit in megabytes.
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

    /// Retrieves a row from the cache based on the provided cache key.
    pub fn get(&self, key: &str) -> Option<&Row> {
        self.data.get(key)
    }

    /// Inserts a new entry into the cache with the provided cache key and row.
    pub fn insert(&mut self, key: String, event: Row) {
        self.data.insert(key, event);
    }

    /// Retrieves a vector containing all cache keys.
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Clears all entries from the cache.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Safely inserts time-related metadata into the cache, expanding if necessary.
    pub fn safe_time_insert(&mut self, key: &str, time_h: TimeCache) {
        if self.current_data_size + time_h.data_length <= self.max_data_size {
            if self.latest_time == 0 {self.latest_time = time_h.last_accessed}
            self.time_data.insert(key.to_string(), time_h);
        } else {
            expand_cache(self);
            self.time_data.insert(key.to_string(), time_h);
        }
    }

    /// Updates the last accessed time for a cache entry based on the provided key.
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

    /// Deletes a cache entry based on the provided key, reducing the current data size.
    pub fn delete(&mut self, key: &str) {
        self.current_data_size -= self.time_data.get(key).unwrap().data_length;
        self.data.remove(key);
        self.time_data.remove(key);
    }
}

/// Expands the cache by removing the entry with the lowest access time if the size limit is exceeded.
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