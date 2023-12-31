use crate::{db::row, protos::row::Row, cache, types, http::receiver};

use serde_json::Value;
use simd_json::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Bunch {
    pub key: String,
    pub value: Value,
    pub _type: String,
}

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

pub fn delete(req: &receiver::RequestHeaders) -> Result<String, String> {
    let status: Result<String, String> = cache::delete(&req.db, &req.table, &req.key);

    match status {
        Ok(_) => Ok("{\"code\": 200, \"message\": \"Row was delete\"}\njson".to_string()),
        Err(err) => Ok(("{\"code\": 400, \"message\": \"".to_owned()+ &err +"\"}\njson").to_string())
    }
}