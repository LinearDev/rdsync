use std::sync::MutexGuard;

use crate::tx_pool::{POOL, tx_table::{TX, TxPool}};

/// Checks if the transaction pool is empty.
///
/// # Returns
///
/// * `true` if the pool is empty, otherwise `false`.
pub fn is_pool_empty() -> bool {
    let pool: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    return pool.keys().len() == 0;
}

/// Retrieves the latest transaction from the pool.
///
/// # Returns
///
/// * A tuple containing a clone of the latest transaction and its key.
pub fn get_latest_tx() -> (TX, String) {
    let pool: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    let (tx, k): (&TX, String) = pool.get_one().clone();
    return (tx.clone(), k);
}

/// Deletes the latest transaction from the pool based on its key.
///
/// # Arguments
///
/// * `key` - The key of the transaction to be deleted.
pub fn delete_latest_tx(key: &str) {
    let mut pool_mut: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    pool_mut.delete(key);
}