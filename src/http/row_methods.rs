use crate::db::row;
use crate::http::build_response;
use crate::cache;

use simd_json::prelude::*;
use std::convert::Infallible;
use hyper::{Request, Body, Response, header::CONTENT_TYPE, body::Bytes};
use serde::Deserialize;

#[derive(Deserialize)]
struct Bunch {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct BunchReq {
    db: String,
    table: String,
    bunch: Vec<Bunch>
}

fn get_url_data(uri: Option<&str>) -> (&str, &str, &str, &str) {
    let mut db: &str = "";
    let mut table: &str = "";
    let mut row_key: &str = "";

    if let Some(path) = uri {
        for pair in path.split('&') {
            let mut iter = pair.split('=');

            if let Some(key) = iter.next() {
                match key {
                    "db" => if let Some(value) = iter.next() { db = value },
                    "table" => if let Some(value) = iter.next() { table = value },
                    "key" => if let Some(value) = iter.next() { row_key = value },
                    _ => (),
                }
            }
        }
        return (db, table, row_key, "");
    } else {
        return (db, table, row_key, "can't find some keys in url 'db, table, key'");
    }
}

pub async fn get(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (db, table, row_key, err) = get_url_data(req.uri().query());

    if !err.is_empty() {
        return Ok(build_response(err));
    }

    let data = cache::get(&db, &table, &row_key);

    match data {
        Ok(r) => {
            let value = r;
            let resp_type = "text/html; charset=UTF-8";
            // if r.type_() == "json" {
            //     resp_type = "application/json";
            // }
            
            let resp = Response::builder()
                .header(CONTENT_TYPE, resp_type)
                .body(Body::from(value)).unwrap();
            return Ok(resp);
        },
        Err(err) => {
            return Ok(build_response(err.as_str()));
        }
    }
}

pub async fn add(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri = req.uri().clone();
    let query = uri.query();

    let (db, table, row_key, err) = get_url_data(query);

    if !err.is_empty() {
        return Ok(build_response(err));
    }

    let whole_body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(bytes) => bytes,
        Err(_) => return Ok(build_response("0")),
    };
    let value = whole_body.to_vec();

    let status = cache::add(db, table, row_key, &value);

    // if status {
        return Ok(build_response("1"));
    // } else {
    //     return Ok(build_response("0"));
    // }
}

pub async fn bunch(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri = req.uri().clone();
    let query = uri.query();

    let (db, table, row_key, err) = get_url_data(query);

    if !err.is_empty() {
        return Ok(build_response(err));
    }

    let whole_body: Bytes = match hyper::body::to_bytes(req.into_body()).await {
        Ok(bytes) => bytes,
        Err(_) => return Ok(build_response("0")),
    };
    let mut value: Vec<u8> = whole_body.to_vec();

    let res: Result<Vec<Bunch>, simd_json::Error> = simd_json::from_slice(&mut value);
    let mut bunch: Vec<Bunch> = Vec::with_capacity(1024);

    match res {
        Ok(b) => {bunch = b},
        Err(_) => return Ok(build_response("0"))
    }

    // match serde_json::from_slice(&whole_body) {
    //     Ok(data) => {request_payload = data},
    //     Err(err) => {parse_err = err.to_string()}
    // }

    // if parse_err.len() != 0 {
    //     return Ok(build_response(&parse_err));
    // }

    // let status = request_payload.bunch.iter().all(|elem| cache::add(&request_payload.db, &request_payload.table, &elem.key, &elem.value));
    let status = bunch.iter().all(|elem| row::add_row(db, table, &elem.key, &elem.value));

    if status {
        return Ok(build_response("bunch was add"));
    } else {
        return Ok(build_response(&format!("can't add bunch")));
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
        return Ok(build_response("can't find some keys in url 'db, table, key'"));
    }

    let status = cache::delete(&db, &table, &row_key);

    match status {
        Ok(_) => Ok(build_response("1")),
        Err(_) => Ok(build_response("0"))
    }
}