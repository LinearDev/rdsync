pub mod tx_table;
pub mod req_handler;
pub mod worker;

use std::{thread::{self, sleep}, sync::{MutexGuard, Mutex}, time};
use lazy_static::lazy_static;

use crate::{http::{self, receiver::RequestHeaders}, config::CONFIG};
use tx_table::{TxPool, TX};

lazy_static! {
    pub static ref POOL: Mutex<TxPool> = Mutex::new(TxPool::new());
}

pub fn add_tx(req: &str, head: RequestHeaders, body: &str, to: &str) {
    let tx: TX = TX{
        req: req.to_string(),
        head: head,
        body: body.to_string(),
        to: to.to_string()
    };

    let mut pool: MutexGuard<'_, TxPool> = POOL.lock().unwrap();
    pool.insert(tx);

    return;
}

fn wsleep() {
    sleep(time::Duration::from_secs(1));
    return;
}

fn worker() {
    loop {
        if worker::is_pool_empty() {
            wsleep();
            continue;
        }
        
        let (tx, key): (TX, String) = worker::get_latest_tx();
        worker::delete_latest_tx(&key);
        let a: Result<String, String> = req_handler::handle_request(&tx.req, &tx.head, &tx.body);
        
        let response: String;
        match a {
            Ok(data) => {response = data},
            Err(err) => {response = err}
        }

        http::send(&tx.head.rud, &response, &tx.to);

        if worker::is_pool_empty() {
            wsleep();
        }
    }
}

pub fn start() {
    for _ in 0..CONFIG.workers_count {
        thread::spawn(worker);
        sleep(time::Duration::from_millis((CONFIG.workers_count * 2).into()));
    }
}