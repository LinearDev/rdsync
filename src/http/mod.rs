pub mod row_methods;
pub mod table_methods;
pub mod db_methods;
pub mod receiver;

use std::{net::{TcpListener, TcpStream}, thread, sync::{Mutex, MutexGuard}, collections::HashMap, io::Write};
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::{tx_pool::add_tx, config::CONFIG};
use receiver::RequestHeaders;

pub struct Clients {
    pub writeble: HashMap<String, TcpStream>
}

impl Clients {
    pub fn new() -> Self {
        Self{
            writeble: HashMap::new()
        }
    }
}

lazy_static! {
    pub static ref CLIENTS: Mutex<Clients> = Mutex::new(Clients::new());
}

fn add_stream(stream: TcpStream) -> String {
    let mut cl: MutexGuard<'_, Clients> = CLIENTS.lock().unwrap();
    let address: Uuid = Uuid::new_v4();
    cl.writeble.insert(address.to_string(), stream.try_clone().unwrap());
    return address.to_string();
}

pub fn send(rud: &str, data: &str, to: &str) {
    let cl: MutexGuard<'_, Clients> = CLIENTS.lock().unwrap();

    let mut client: &TcpStream = cl.writeble.get(to).unwrap();
    client.write(format!("rud: {}\n{}", rud, data).as_bytes()).unwrap();

    return;
}

fn handle_client(stream: TcpStream) {
    let address: String = add_stream(stream.try_clone().unwrap());
    loop {
        let req: (String, String);
        match receiver::deserialize(stream.try_clone().as_mut().unwrap(), &address) {
            Ok(data) => {req = data},
            Err(a) => {
                let mut c: MutexGuard<'_, Clients> = CLIENTS.lock().unwrap();
                c.writeble.remove(&a);
                break;
            }
        }

        let head: (String, RequestHeaders) = receiver::get_header(req.0);

        add_tx(&head.0, head.1, &req.1, &address);
    }
}

pub fn start() {
    let config_ip = CONFIG.ip.clone();
    let config_port = CONFIG.port.to_string();
    let listener: TcpListener = TcpListener::bind(format!("{}:{}", config_ip, config_port)).unwrap();
    
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        
        thread::spawn(|| {
            handle_client(stream)
        });
    }
}
