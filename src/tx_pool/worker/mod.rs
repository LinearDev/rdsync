use std::sync::MutexGuard;

use crate::tx_pool::{POOL, tx_table::{TX, TxPool}};

pub fn is_pool_empty() -> bool {
    let pool: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    return pool.keys().len() == 0;
}

pub fn get_latest_tx() -> (TX, String) {
    let pool: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    let (tx, k): (&TX, String) = pool.get_one().clone();
    return (tx.clone(), k);
}

pub fn delete_latest_tx(key: &str) {
    let mut pool_mut: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    pool_mut.delete(key);
}