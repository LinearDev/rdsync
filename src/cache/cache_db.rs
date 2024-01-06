use crate::db::db;
use crate::cache;

/// Deletes a database and its associated entries from both the file database and the cache.
///
/// # Arguments
///
/// * `name` - Name of the database to be deleted.
///
/// # Returns
///
/// Returns a Result indicating the status of the database deletion operation.
/// - If the deletion is successful, it returns a JSON-formatted string with a success message.
/// - If an error occurs during the deletion, it returns an error message.
pub fn delete_db(name: &str) -> Result<String, String> {
    if let Err(err) = db::delete(name) {
        return Err(err);
    }

    let c_keys: Vec<String> = cache::keys();

    c_keys.iter().for_each(|key| {
        if key.find(name).is_some() {
            let k = cache::from_cache_string(key.to_owned());
            cache::delete(&k.db, &k.table, &k.key).unwrap();
        }
    });

    return Ok("{\"code\": 200, \"message\": \"DB was delete\"}".to_string());
}