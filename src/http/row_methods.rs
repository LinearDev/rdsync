use crate::{db::{row, json_filter}, protos::row::Row, cache, types, http::receiver};

use serde_json::Value;
use simd_json::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a key-value pair with type of one value.
#[derive(Deserialize, Serialize, Debug)]
pub struct Bunch {
    pub key: String,
    pub value: Value,
    pub _type: String,
}

/// Retrieves the value associated with the specified key in the given table and database.
///
/// # Arguments
///
/// * `req` - A reference to the `RequestHeaders` containing information about the request.
///
/// # Returns
///
/// A `Result` containing the retrieved value or an error message.
pub fn get(req: &receiver::RequestHeaders) -> Result<String, String> {
    let data: Result<Row, String> = cache::get(&req.db, &req.table, &req.key);

    match data {
        Ok(r) => {
            let value = r.clone();
            
            return Ok(value.value().to_string() + "\n" + r.type_());
        },
        Err(err) => {
            return Err(err);
        }
    }
}

//TODO: filtering non json
pub fn filter(req: &receiver::RequestHeaders, data: &str) -> Result<String, String> {
    if req._type == "json" {
        match json_filter::filter(&req.db, &req.table, data) {
            Ok(d) => {
                let str = serde_json::to_string(&d).unwrap();
                return Ok(str);
            },
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        Err("Now support only json filtering".to_string())
    }
}

/// Adds a new key-value pair to the specified table and database.
///
/// # Arguments
///
/// * `req` - A reference to the `RequestHeaders` containing information about the request.
/// * `value` - The value to be added.
///
/// # Returns
///
/// A `Result` indicating success or an error message.
pub fn add(req: &receiver::RequestHeaders, value: &str) -> Result<String, String> {
    let check: Result<(), String> = types::is_valid_data(&value, &req._type);
    match check {
        Ok(_) => {},
        Err(err) => {
            return Err(("{\"code\": 400, \"message\": \"".to_string()+ &err +"\"}\njson").to_string());
        }
    }

    let status: bool = cache::add(&req.db, &req.table, &req.key, &value, &req._type);

    if status {
        return Ok("{\"code\": 200, \"message\": \"New value was add\"}\njson".to_string());
    } else {
        return Err("0".to_string());
    }
}

/// Adds a bunch of key-value pairs to the specified table and database.
///
/// # Arguments
///
/// * `req` - A reference to the `RequestHeaders` containing information about the request.
/// * `value` - A JSON string representing a list of `Bunch` objects.
///
/// # Returns
///
/// A `Result` indicating success or an error message.
pub fn bunch(req: &receiver::RequestHeaders, value: &str) -> Result<String, String> {
    let res: Result<Vec<Bunch>, serde_json::Error> = serde_json::from_str::<Vec<Bunch>>(&value);
    let mut bunch: Vec<Bunch> = Vec::with_capacity(1024);

    match res {
        Ok(b) => {bunch = b},
        Err(err) => {
            println!("{}", err);
            return Ok("0".to_string())
        }
    }

    let status: bool = bunch.iter().all(|elem| {
        let mut row: Row = Row::new();
        row.set_value(elem.value.to_string());
        row.set_type(elem._type.to_string());
        row::add_row(&req.db, &req.table, &elem.key, &mut row)
    });

    if status {
        return Ok("bunch was add".to_string());
    } else {
        return Ok(format!("can't add bunch"));
    }
}

// pub fn edit(req: Request<Body>) -> Result<Response<Body>, Infallible> {

//     let res = Response::new(
//         Body::from("adsf")
//     );

//     Ok(res)
// }

/// Deletes the specified key-value pair from the given table and database.
///
/// # Arguments
///
/// * `req` - A reference to the `RequestHeaders` containing information about the request.
///
/// # Returns
///
/// A `Result` indicating success or an error message.
pub fn delete(req: &receiver::RequestHeaders) -> Result<String, String> {
    let status: Result<String, String> = cache::delete(&req.db, &req.table, &req.key);

    match status {
        Ok(_) => Ok("{\"code\": 200, \"message\": \"Row was delete\"}\njson".to_string()),
        Err(err) => Ok(("{\"code\": 400, \"message\": \"".to_owned()+ &err +"\"}\njson").to_string())
    }
}