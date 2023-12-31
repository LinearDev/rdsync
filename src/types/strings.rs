use std::io::Read;

pub fn is_string(data: &mut &[u8]) -> bool {
    let mut s = String::new();
    
    match data.read_to_string(&mut s) {
        Ok(_) => {return true},
        Err(_) => {return false},
    }
}

