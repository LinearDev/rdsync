// use std::env;

pub mod protos;
pub mod db;
pub mod config;

fn main() {
    // env::set_var("OUT_DIR", "./");
    // build::run();

    let status = config::read_config();
    if !status.1 {
        println!("[ ERROR ] main can not read config");
        return;
    }
}