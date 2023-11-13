#[cfg(test)]
mod test {
    use crate::db::{row::{add_row, read_row, delete_row, is_row_exist}, create_db, table::create_table};

    #[test]
    fn add_row_test() {
        assert_eq!(add_row("test_db", "sass", "test", "ar"), true);
    }

    #[test]
    fn read_row_test() {
        let res = read_row("test_db", "table", "sass");

        match res {
            Ok(_) => {},
            Err(_) => {assert_eq!(true, true)}
        }

        assert_eq!(add_row("test_db", "table", "sass", "{\"id\": \"1\"}"), true);
        let res = read_row("test_db", "table", "sass");

        match res {
            Ok(row) => {assert_eq!(row.type_(), "json")},
            Err(_) => {}
        }
    }

    #[test]
    fn delete_row_test() {
        assert_eq!(delete_row("test_db", "table", "kvas"), false);
        assert_eq!(add_row("test_db", "table", "kvas", "{\"id\": \"1\"}"), true);
        assert_eq!(delete_row("test_db", "table", "kvas"), true);
    }

    #[test]
    fn is_row_exist_test() {
        let row = is_row_exist("row_test", "row_table", "asdf");

        match row {
            Ok(_) => {},
            Err(msg) => {assert_eq!(msg, "[ ERROR ] Row: DB is not exist")}
        }

        create_db("row_test");

        let row = is_row_exist("row_test", "row_table", "asdf");

        match row {
            Ok(_) => {},
            Err(msg) => {assert_eq!(msg, "[ ERROR ] Row: Table is not exist")}
        }

        create_table("row_test", "row_table");

        let row = is_row_exist("row_test", "row_table", "asdf");

        match row {
            Ok(_) => {},
            Err(msg) => {assert_eq!(msg, "[ ERROR ] Row: is not exist")}
        }

        add_row("row_test", "row_table", "asdf", "0");

        let row = is_row_exist("row_test", "row_table", "asdf");

        match row {
            Ok(data) => assert_eq!(data, ()),
            Err(_) => {}
        }
    }
}