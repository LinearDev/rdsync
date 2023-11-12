// use std::env;

pub mod protos;
pub mod db;
pub mod config;
pub mod rdsync;

fn main() {
    // env::set_var("OUT_DIR", "./");
    // build::run();

    config::read_config().expect("[ ERROR ] Main: Can not read config");

    if !db::init() {
        print!("[ ERROR ] Main: Can not init DataBase")
    };
}