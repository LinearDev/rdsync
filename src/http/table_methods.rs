use crate::{db::table::{create_table, get_table, get_table_with_keys}, cache::delete_table};

use super::{row_methods::Bunch, receiver};

pub fn get(req: &receiver::RequestHeaders) -> Result<String, String> {
    let status: Result<Vec<String>, String> = get_table(&req.db, &req.table);

    match status {
        Ok(mut data) => {
            let s: String = simd_json::to_string(&mut data).unwrap();
        
            return Ok(s + "\njson");
        },
        Err(err) => {      
            return Ok(err);
        }
    }
}

pub fn get_with_keys(req: &receiver::RequestHeaders) -> Result<String, String> {
    let status: Result<Vec<Bunch>, String> = get_table_with_keys(&req.db, &req.table);

    match status {
        Ok(mut data) => {
            let s: String = simd_json::to_string(&mut data).unwrap();
        
            return Ok(s + "\njson");
        },
        Err(err) => {        
            return Ok(err);
        }
    }
}

pub fn create(req: &receiver::RequestHeaders) -> Result<String, String> {
    let status = create_table(&req.db, &req.table);

    if status {
        Ok("1".to_string())
    } else {
        Ok("0".to_string())
    }
}

pub fn delete(req: &receiver::RequestHeaders) -> Result<String, String>  {
    let status = delete_table(&req.db, &req.table);

    if status {
        Ok("1".to_string())
    } else {
        Ok("0".to_string())
    }
}