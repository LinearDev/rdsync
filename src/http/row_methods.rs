use crate::http::{err_resp, ok_resp};
use crate::cache;

use std::convert::Infallible;
use hyper::{Request, Body, Response, header::CONTENT_TYPE};
use serde::Deserialize;

#[derive(Deserialize)]
struct AddReq {
    db: String,
    table: String,
    key: String,
    value: String
}

pub async fn get(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut db: String = "".to_string();
    let mut table: String = "".to_string();
    let mut row_key: String = "".to_string();

    if let Some(path) = req.uri().query() {
        for pair in path.split('&') {
            let mut iter = pair.split('=');

            if let Some(key) = iter.next() {
                match key {
                    "db" => if let Some(value) = iter.next() { db = value.to_string() },
                    "table" => if let Some(value) = iter.next() { table = value.to_string() },
                    "key" => if let Some(value) = iter.next() { row_key = value.to_string() },
                    _ => (),
                }
            }
        }
    } else {
        return Ok(err_resp("can't find some keys in url 'db, table, key'"));
    }

    let data = cache::get(&db, &table, &row_key);

    match data {
        Ok(r) => {
            let value = r.value().to_string();
            let mut resp_type = "text/html; charset=UTF-8";
            if r.type_() == "json" {
                resp_type = "application/json";
            }
            
            let resp = Response::builder()
                .header(CONTENT_TYPE, resp_type)
                .body(Body::from(value)).unwrap();
            return Ok(resp);
        },
        Err(err) => {
            return Ok(err_resp(err.as_str()));
        }
    }
}

pub async fn add(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let mut request_payload: AddReq = AddReq { 
        db: "".to_string(),
        table: "".to_string(),
        key: "".to_string(),
        value: "".to_string()
    };
    let mut parse_err: String = "".to_string();

    match serde_json::from_slice(&whole_body) {
        Ok(data) => {request_payload = data},
        Err(err) => {parse_err = err.to_string()}
    }

    if parse_err.len() != 0 {
        return Ok(err_resp(&parse_err));
    }

    let status = cache::add(&request_payload.db, &request_payload.table, &request_payload.key, &request_payload.value);

    if status {
        return Ok(ok_resp("new key was add"));
    } else {
        return Ok(err_resp(&format!("can't add new key - {}", &request_payload.key)));
    }
}

pub fn edit(req: Request<Body>) -> Result<Response<Body>, Infallible> {

    let res = Response::new(
        Body::from("adsf")
    );

    Ok(res)
}

pub async fn delete(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut db: String = "".to_string();
    let mut table: String = "".to_string();
    let mut row_key: String = "".to_string();

    if let Some(path) = req.uri().query() {
        for pair in path.split('&') {
            let mut iter = pair.split('=');

            if let Some(key) = iter.next() {
                match key {
                    "db" => if let Some(value) = iter.next() { db = value.to_string() },
                    "table" => if let Some(value) = iter.next() { table = value.to_string() },
                    "key" => if let Some(value) = iter.next() { row_key = value.to_string() },
                    _ => (),
                }
            }
        }
    } else {
        return Ok(err_resp("can't find some keys in url 'db, table, key'"));
    }

    let status = cache::delete(&db, &table, &row_key);

    match status {
        Ok(ok) => Ok(ok_resp(&ok)),
        Err(err) => Ok(err_resp(&err))
    }
}