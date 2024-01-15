//! JSON Data filter
use serde_json::{Map, Value};
use serde_json::map::Keys;

use crate::{db::table::get_table_with_keys, http::row_methods::Bunch};

pub fn filter(db: &str, name: &str, filter_json: &str) -> Result<Vec<Bunch>, String> {
    let filter: Map<String, Value> = serde_json::from_str(filter_json).unwrap();

    let data: Result<Vec<Bunch>, String> = get_table_with_keys(db, name);
    let bunch: Vec<Bunch>;
    match data {
        Ok(_bunch) => {bunch = _bunch},
        Err(e) => {return Err(e)}
    }

    let mut matches: Vec<Bunch> = Vec::with_capacity(512);

    for x in bunch {
        if x._type == "json" {
            let obj: Map<String, Value> = serde_json::from_str(x.value.as_str().unwrap()).unwrap();

            if apply_filter(&obj, &filter) {
                matches.push(x);
            }
        }
    }

    Ok(matches)
}

fn apply_filter(json: &Map<String, Value>, filter: &Map<String, Value>) -> bool {
    //TODO: Make more then for one key filter
    fn recursive_apply(json: &Map<String, Value>, filter: &Map<String, Value>) -> bool {
        let keys: Keys = filter.keys();
        let keys_collect: Vec<&String> = keys.collect::<Vec<_>>();

        if keys_collect.len() == 0 {
            return false;
        }

        let key: String = keys_collect[0].clone();

        let filter_value: &Value = filter.get(&key).unwrap();
        let json_value: &Value = json.get(&key).unwrap();

        let mut new_filter_value: &Map<String, Value>;
        let new_json_value: &Map<String, Value>;

        match filter_value {
            Value::Array(value) => {
                new_filter_value = value.get(0).unwrap().as_object().unwrap();
                new_json_value = json_value.as_array().unwrap().get(0).unwrap().as_object().unwrap();
            },
            Value::Object(value) => {
                new_filter_value = value;
                new_json_value = json_value.as_object().unwrap();
            }
            _ => {
                if filter_value == json_value {
                    return true;
                }

                return false;
            }
        }

        return recursive_apply(new_json_value, new_filter_value);
    }

    recursive_apply(json, filter)
}
