use crate::http::receiver::RequestHeaders;

use std::collections::HashMap;

/// Represents a transaction (TX) with request details.
#[derive(Debug, Clone)]
pub struct TX {
    /// The raw request data.
    pub req: String,

    /// The request headers.
    pub head: RequestHeaders,

    /// The request body.
    pub body: String,

    /// The destination of the transaction (session id).
    pub to: String
}

/// A pool of transactions (TX) organized as a HashMap.
pub struct TxPool {
    /// The internal data structure holding transactions.
    pub data: HashMap<String, TX>,
}

impl TxPool {
    /// Creates a new `TxPool` instance.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Retrieves the latest transaction and its key.
    ///
    /// # Returns
    ///
    /// * A tuple containing a reference to the latest transaction and its key.
    pub fn get_one(&self) -> (&TX, String) {
        let mut keys: Vec<String> = self.data.keys().cloned().collect();
        //TODO: remove reverse
        keys.reverse();
        let key = &keys[keys.len()-1];
        return (self.data.get(key).unwrap(), key.to_string())
    }

    /// Inserts a new transaction into the pool.
    ///
    /// # Arguments
    ///
    /// * `event` - The transaction to be inserted.
    ///
    /// # Returns
    ///
    /// * The index assigned to the inserted transaction.
    pub fn insert(&mut self, event: TX) -> usize {
        let index = self.data.len();
        self.data.insert(index.to_string(), event);
        return index;
    }

    /// Retrieves all keys in the pool.
    ///
    /// # Returns
    ///
    /// * A vector containing all keys in the pool.
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Clears all transactions from the pool.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Deletes a transaction from the pool based on its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the transaction to be deleted.
    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}