use crate::{http::{err_resp, ok_resp}, db::table::create_table, cache::delete_table};

use std::convert::Infallible;
use hyper::{Request, Body, Response};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateReq {
    db: String,
    name: String,
}

pub async fn create(req: Request<Body>) -> Result<Response<Body>, Infallible>  {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let mut request_payload: CreateReq = CreateReq { 
        db: "".to_string(),
        name: "".to_string(),
    };
    let mut parse_err: String = "".to_string();

    match serde_json::from_slice(&whole_body) {
        Ok(data) => {request_payload = data},
        Err(err) => {parse_err = err.to_string()}
    }

    if parse_err.len() != 0 {
        return Ok(err_resp(&parse_err));
    }

    let status = create_table(&request_payload.db, &request_payload.name);

    if status {
        Ok(ok_resp(&format!("Table with name \"{}\" has been created", request_payload.name)))
    } else {
        Ok(err_resp(&format!("Table with name \"{}\" has not been created", request_payload.name)))
    }
}

pub async fn delete(req: Request<Body>) -> Result<Response<Body>, Infallible>  {
    let mut db: String = "".to_string();
    let mut table: String = "".to_string();

    if let Some(path) = req.uri().query() {
        for pair in path.split('&') {
            let mut iter = pair.split('=');

            if let Some(key) = iter.next() {
                match key {
                    "db" => if let Some(value) = iter.next() { db = value.to_string() },
                    "table" => if let Some(value) = iter.next() { table = value.to_string() },
                    _ => (),
                }
            }
        }
    } else {
        return Ok(err_resp("can't find some keys in url 'db, table, key'"));
    }

    let status = delete_table(&db, &table);

    if status {
        Ok(ok_resp(&format!("Table with name \"{}\" has been deleted", table)))
    } else {
        Ok(err_resp(&format!("Table with name \"{}\" has not been deleted", table)))
    }
}