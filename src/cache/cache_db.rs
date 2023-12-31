use crate::db::db;
use crate::cache;

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