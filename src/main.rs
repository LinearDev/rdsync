// Importing necessary modules
pub mod protos;
pub mod db;
pub mod config;
pub mod rdsync;
pub mod cache;
pub mod tests;
pub mod http;
pub mod types;
pub mod tx_pool;

// The main function
fn main() {
    env!("RUST_BACKTRACE=1");
    // Reading configuration from file
    config::read_config().expect("[ ERROR ] Main: Can not read config");

    // Initializing the database
    if !db::init() {
        print!("[ ERROR ] Main: Can not init DataBase");
        return;
    };

    // Starting the transaction pool
    tx_pool::start();

    db::json_filter::filter("test_db", "test_table");

    // Starting the transactional pipeline server
    http::start();
}