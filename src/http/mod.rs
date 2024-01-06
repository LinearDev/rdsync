//! Asynchronous transaction pipeline server.

pub mod row_methods;
pub mod table_methods;
pub mod db_methods;
pub mod receiver;

use std::{net::{TcpListener, TcpStream}, thread, sync::{Mutex, MutexGuard}, collections::HashMap, io::Write};
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::{tx_pool::add_tx, config::CONFIG};
use receiver::RequestHeaders;

/// A structure representing the clients connected to the server with writable streams.
pub struct Clients {
    /// Open writeable streams for each client.
    pub writable: HashMap<String, TcpStream>
}

/// Creates a new instance of Clients with an empty HashMap for writable streams.
impl Clients {
    /// Create a new Clients instance.
    pub fn new() -> Self {
        Self{
            writable: HashMap::new()
        }
    }
}

lazy_static! {
    /// Global CLIENTS instance.
    pub static ref CLIENTS: Mutex<Clients> = Mutex::new(Clients::new());
}

/// Adds a new TcpStream to the Clients structure and returns the address (Uuid) associated with the stream.
///
/// # Arguments
///
/// * `stream` - The TcpStream to be added.
///
/// # Returns
///
/// Returns the address (Uuid) associated with the added TcpStream.
fn add_stream(stream: TcpStream) -> String {
    let mut cl: MutexGuard<'_, Clients> = CLIENTS.lock().unwrap();
    let address: Uuid = Uuid::new_v4();
    cl.writable.insert(address.to_string(), stream.try_clone().unwrap());
    return address.to_string();
}

/// Sends a message to a specific client identified by the provided address.
///
/// # Arguments
///
/// * `rud` - The Rudiment identifier.
/// * `data` - The data to be sent.
/// * `to` - The address (Uuid) of the target client.
pub fn send(rud: &str, data: &str, to: &str) {
    let cl: MutexGuard<'_, Clients> = CLIENTS.lock().unwrap();

    let mut client: &TcpStream = cl.writable.get(to).unwrap();
    client.write(format!("rud: {}\n{}", rud, data).as_bytes()).unwrap();

    return;
}

/// Handles the communication with a connected client, receiving and processing messages.
///
/// # Arguments
///
/// * `stream` - The TcpStream representing the connection to the client.
fn handle_client(stream: TcpStream) {
    let address: String = add_stream(stream.try_clone().unwrap());
    loop {
        let req: (String, String);
        match receiver::deserialize(stream.try_clone().as_mut().unwrap(), &address) {
            Ok(data) => {req = data},
            Err(a) => {
                let mut c: MutexGuard<'_, Clients> = CLIENTS.lock().unwrap();
                c.writable.remove(&a);
                break;
            }
        }

        let head: (String, RequestHeaders) = receiver::get_header(req.0);

        add_tx(&head.0, head.1, &req.1, &address);
    }
}

/// Starts the asynchronous transaction pipeline server, listening for incoming connections.
pub fn start() {
    let config_ip: String = CONFIG.ip.clone();
    let config_port: String = CONFIG.port.to_string();
    let listener: TcpListener;
    
    match TcpListener::bind(format!("{}:{}", config_ip, config_port)) {
        Ok(ls) => {
            listener = ls;
            println!("[ LOG ] `async tx pipeline` listens - {}:{}", config_ip, config_port)
        },
        Err(err) => {
            println!("[ ERROR ] `async tx pipeline` not started");
            println!("{:?}", err);
            return;
        }
    }
    
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        
        thread::spawn(|| {
            handle_client(stream)
        });
    }
}
