use std::{net::TcpStream, io::Read};

const SECTIONS_IN_TX: i32 = 2;

#[derive(Debug, Clone)]
pub struct RequestHeaders {
    pub rud: String,
    pub db: String,
    pub table: String,
    pub key: String,
    pub _type: String
}

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

pub fn deserialize(mut stream: &TcpStream, address: &str) -> Result<(String, String), String> {
    let mut is_header_section: bool = false;
    let mut is_body_section: bool = false;
    let mut section_number = 0;

    let mut header_buf = Vec::<u8>::with_capacity(512);
    let mut body_buf = Vec::<u8>::with_capacity(512);
    loop {
        if section_number == SECTIONS_IN_TX {
            break;
        }
        let mut temp_buf = [0; 1];
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

// [1, - start header section
// 114, 101, 113, 58, 32, 97, 100, 100, 95, 114, 111, 119, 10,
// 114, 117, 100, 58, 32, 48, 49, 72, 74, 83, 69, 90, 72, 56, 49, 55, 74, 74, 71, 83, 53, 70, 53, 77, 72, 68, 83, 54, 77, 74, 87, 10,
// 100, 98, 58, 32, 116, 101, 115, 116, 95, 100, 98, 10,
// 116, 97, 98, 108, 101, 58, 32, 116, 101, 115, 116, 95, 116, 97, 98, 108, 101, 95, 106, 115, 111, 110, 10,
// 107, 101, 121, 58, 32, 49, 10,
// 116, 121, 112, 101, 58, 32, 106, 115, 111, 110, 10,
// 23, - end header section
// 2, - start body section
// 123, 34, 107, 101, 121, 34, 58, 34, 75, 70, 121, 116, 107, 66, 97, 66, 78, 100, 111, 117, 71, 75, 102, 67, 98, 118, 90, 78, 113, 111, 75, 106, 34, 44, 34, 118, 97, 108, 117, 101, 34, 58, 123, 34, 105, 100, 34, 58, 34, 75, 70, 121, 116, 107, 66, 97, 66, 78, 100, 111, 117, 71, 75, 102, 67, 98, 118, 90, 78, 113, 111, 75, 106, 34, 44, 34, 101, 109, 97, 105, 108, 34, 58, 34, 74, 97, 107, 79, 115, 78, 69, 119, 95, 89, 67, 121, 114, 121, 84, 64, 121, 97, 104, 111, 111, 46, 99, 111, 109, 34, 44, 34, 114, 111, 108, 101, 115, 34, 58, 91, 34, 103, 117, 101, 115, 116, 34, 44, 34, 97, 100, 109, 105, 110, 34, 44, 34, 109, 101, 109, 98, 101, 114, 34, 93, 44, 34, 97, 112, 105, 75, 101, 121, 34, 58, 34, 53, 55, 100, 50, 97, 102, 102, 101, 45, 56, 101, 100, 98, 45, 52, 98, 56, 102, 45, 98, 57, 54, 52, 45, 99, 53, 99, 56, 49, 99, 48, 53, 56, 50, 55, 54, 34, 44, 34, 112, 114, 111, 102, 105, 108, 101, 34, 58, 123, 34, 100, 111, 98, 34, 58, 34, 49, 57, 57, 56, 45, 48, 55, 45, 50, 53, 34, 44, 34, 110, 97, 109, 101, 34, 58, 34, 84, 86, 107, 100, 102, 77, 67, 74, 32, 86, 76, 68, 77, 83, 89, 34, 44, 34, 97, 98, 111, 117, 116, 34, 58, 34, 109, 103, 71, 81, 106, 89, 108, 107, 103, 75, 81, 69, 107, 73, 111, 107, 88, 107, 90, 68, 100, 117, 90, 71, 120, 89, 84, 116, 112, 106, 118, 82, 82, 67, 65, 118, 116, 119, 87, 99, 105, 90, 66, 74, 113, 117, 71, 89, 121, 69, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 81, 106, 65, 89, 103, 110, 72, 90, 69, 102, 44, 32, 117, 80, 113, 114, 86, 101, 116, 65, 122, 72, 44, 32, 119, 110, 115, 98, 97, 72, 73, 88, 109, 117, 34, 44, 34, 99, 111, 109, 112, 97, 110, 121, 34, 58, 34, 79, 103, 67, 97, 107, 110, 110, 68, 34, 44, 34, 108, 111, 99, 97, 116, 105, 111, 110, 34, 58, 123, 34, 108, 97, 116, 34, 58, 50, 50, 46, 53, 48, 54, 53, 57, 49, 44, 34, 108, 111, 110, 103, 34, 58, 49, 49, 46, 52, 53, 50, 53, 50, 57, 125, 125, 44, 34, 117, 115, 101, 114, 110, 97, 109, 101, 34, 58, 34, 100, 79, 89, 115, 68, 110, 85, 114, 34, 44, 34, 99, 114, 101, 97, 116, 101, 100, 65, 116, 34, 58, 34, 50, 48, 50, 51, 45, 49, 49, 45, 50, 49, 84, 48, 49, 58, 51, 54, 58, 48, 52, 46, 50, 50, 52, 56, 55, 50, 90, 34, 44, 34, 117, 112, 100, 97, 116, 101, 100, 65, 116, 34, 58, 34, 50, 48, 50, 51, 45, 49, 49, 45, 50, 50, 84, 48, 49, 58, 51, 54, 58, 48, 52, 46, 50, 50, 52, 56, 55, 51, 90, 34, 125, 125,
// 23] - end body section