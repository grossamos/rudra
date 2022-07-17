use std::{fmt::Display, process};

use crate::config::RudraConfig;

pub fn print_debug_message<T: Display>(config: &RudraConfig, debug_message: T) {
    if config.debug {
        println!("{}", debug_message);
    }
}

pub fn print_error_and_exit<T: Display>(debug_message: T) -> ! {
    eprintln!("{}", debug_message);
    process::exit(1);
}

