#[cfg(test)]
mod test {
    use crate::db::table::{create_table, is_table_exist, delete_table};

    #[test]
    fn create_table_test() {
        assert_eq!(create_table("test_db", "sass"), true);
    }

    #[test]
    fn is_table_exist_test() {
        assert_eq!(is_table_exist("test_db", "ttt"), false);
    }

    #[test]
    fn delete_table_test() {
        assert_eq!(delete_table("test_db", "sass"), true);
    }
}