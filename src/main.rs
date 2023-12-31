pub mod protos;
pub mod db;
pub mod config;
pub mod rdsync;
pub mod cache;
pub mod tests;
pub mod http;
pub mod types;
pub mod tx_pool;

fn main() {
    config::read_config().expect("[ ERROR ] Main: Can not read config");

    if !db::init() {
        print!("[ ERROR ] Main: Can not init DataBase");
        return;
    };

    tx_pool::start();

    http::start();
}