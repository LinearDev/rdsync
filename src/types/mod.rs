pub mod strings;

pub static TYPES: [&str; 8] = [
    "string",
    "int",
    "uint",
    "float",
    "bool",
    "date",
    "timestamp",
    "json"
];

pub fn is_valid_data(data: &str, data_type: &str) -> Result<(), String> {
    match data_type {
        "string" => Ok(()),
        "int" => match data.parse::<i64>() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        },
        "uint" => match data.parse::<u64>() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        },
        "float" => match data.parse::<f64>() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        },
        "bool" => {
            if (data == "1" || data == "0" || data.to_lowercase() == "false" || data.to_lowercase() == "true") {
                return Ok(())
            } else {
                return  Err("not a boolean value".to_string());
            }},
        "date" => match chrono::NaiveDateTime::parse_from_str(data, "%Y-%m-%d %H:%M:%S") {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        },
        "timestamp" => match data.parse::<u64>() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        },
        "json" => match serde_json::from_str::<serde_json::Value>(data) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        },
        _ => Err("provided invalid type".to_string()),
    }
}