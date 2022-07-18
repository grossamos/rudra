use std::{path::Path, fs::File, io::Read};

use crate::config::RudraConfig;

use super::print_debug_message;

pub fn read_file_to_string_or_err<E>(config: &RudraConfig ,path: &Path, err: E) -> Result<String, E> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            print_debug_message(config, why.to_string());
            return Err(err);
        } ,
    };

    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => Ok(file_str),
        Err(_) => Err(err),
    }
}

