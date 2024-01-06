//! Module for deserializing TCP stream data.

use std::{net::TcpStream, io::Read};

/// Number of sections expected in the transmission.
const SECTIONS_IN_TX: i32 = 2;

/// Represents the headers of a request.
#[derive(Debug, Clone)]
pub struct RequestHeaders {
    /// Request flag.
    pub rud: String,

    /// Database name.
    pub db: String,

    /// Table name.
    pub table: String,

    /// Record key.
    pub key: String,

    /// Type of value data.
    pub _type: String
}

/// Parses a header string into a request type and [`RequestHeaders`].
///
/// # Arguments
///
/// * `header` - String containing the serialized request header
///
/// # Returns
///
/// Tuple containing:
///
/// - Request type string
/// - Populated [`RequestHeaders`] struct
///
/// # Examples
///
/// ```rust
/// let header_str = `
///     req: get_row
///     rud: ADSDFS4245ASDF8779T8ASDT
///     db: users
///     table: accounts
///     key: 0
/// `;
/// let (req_type, headers) = get_header(header_str);
/// ```
pub fn get_header(header: String) -> (String, RequestHeaders) {
    let mut req_type: String = String::new();
    let mut req_struct: RequestHeaders = RequestHeaders{
        rud: String::new(),
        db: String::new(),
        table: String::new(),
        key: String::new(),
        _type: String::new()
    };

    let req: Vec<&str> = header.split("\n").collect();
    for data in req {
        let slice: Vec<&str> = data.split(" ").collect();

        match slice[0] {
            "req:" => {req_type = slice[1].to_string()},
            "rud:" => {req_struct.rud = slice[1].to_string()},
            "db:" => {req_struct.db = slice[1].to_string()},
            "table:" => {req_struct.table = slice[1].to_string()},
            "key:" => {req_struct.key = slice[1].to_string()}
            "type:" => {req_struct._type = slice[1].to_string()}
            _ => {},
        }
    }

    return (req_type, req_struct);
}

/// Deserializes a [`TcpStream`] into header and body string sections.
///
/// Reads bytes from the stream and separates into header and body sections,
/// returning each section as a separate string.
///
/// Uses flags and counter to track current section. Each section starts
/// with a 0x1 or 0x2 byte and ends with a 0x17 byte.
///
/// # Arguments
///
/// * `stream` - Mutable reference to the [`TcpStream`] to read from
/// * `address` - String representing the client address
///
/// # Returns
///
/// [`Result`] containing:
///
/// - Ok variant: Tuple with header and body [`String`]s
/// - Err variant: Client [`address`] string on read error
///
/// # Examples
///
/// ```no_run
/// use std::net::TcpStream;
///
/// let stream = TcpStream::connect("127.0.0.1:34254")?;
/// let addr = stream.peer_addr()?;
/// let (header, body) = deserialize(&mut stream, &addr)?;
/// ```
pub fn deserialize(mut stream: &TcpStream, address: &str) -> Result<(String, String), String> {
    let mut is_header_section: bool = false;
    let mut is_body_section: bool = false;
    let mut section_number: i32 = 0;

    let mut header_buf: Vec<u8> = Vec::<u8>::with_capacity(512);
    let mut body_buf: Vec<u8> = Vec::<u8>::with_capacity(512);
    loop {
        if section_number == SECTIONS_IN_TX {
            break;
        }
        let mut temp_buf: [u8; 1] = [0; 1];
        let bytes_read: usize;
        match stream.read(&mut temp_buf) {
            Ok(size) => {bytes_read = size}
            Err(_) => {return Err(address.to_string());}
        }
        match temp_buf[0] {
            1 => {
                is_header_section = true;
                continue;
            },
            2 => {
                is_body_section = true;
                continue;
            },
            23 => {
                if is_header_section {
                    is_header_section = false;
                } else {
                    is_body_section = false
                }
                section_number += 1;
                continue;
            },
            _ => {}
        }
        if is_header_section {
            header_buf.extend_from_slice(&temp_buf[..bytes_read]);
        }
        if is_body_section {
            body_buf.extend_from_slice(&temp_buf[..bytes_read]);
        }
    }

    let head_str: String = String::from_utf8_lossy(&header_buf).to_string();
    let body_str: String = String::from_utf8_lossy(&body_buf).trim().to_string();

    Ok((head_str, body_str))
}