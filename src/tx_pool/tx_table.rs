use crate::http::receiver::RequestHeaders;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TX {
    pub req: String,
    pub head: RequestHeaders,
    pub body: String,
    pub to: String
}

pub struct TxPool {
    pub data: HashMap<String, TX>,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get_one(&self) -> (&TX, String) {
        let keys: Vec<String> = self.data.keys().cloned().collect();
        let key = &keys[keys.len()-1];
        return (self.data.get(key).unwrap(), key.to_string())
    }

    pub fn insert(&mut self, event: TX) -> usize {
        let index = self.data.len();
        self.data.insert(index.to_string(), event);
        return index;
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}