use crate::db;
use crate::config;

#[test]
fn create_table_test() {
    assert_eq!(create_table("test_db", "sass"), true);
}

/**
 * Creates new table
 */
pub fn create_table(db: &str, name: &str) -> bool {
    let db_path: &str = &config::CONFIG.db_path;

    if !db::is_db_exist(db) {
        db::create_db(db);
    }

    return db::init_dir(&format!("{}/{}/{}", db_path, db, name));
}

#[test]
fn is_table_exist_test() {
    assert_eq!(is_table_exist("test_db", "ttt"), false);
}

/**
 * Checks if table exist
 */
pub fn is_table_exist(db: &str, name: &str) -> bool {
    return db::is_dir_exist(&format!("{}/{}", db, name));
}