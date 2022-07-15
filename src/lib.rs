use std::{process::Command, path::Path};

use config::{RudraConfig, ConfigurationError};
use evaluator::compare_endpoints;
use models::Endpoint;
use parser::{parse_openapi_json, ParsingError};
use utils::print_debug_message;

use crate::{utils::print_error_and_exit, parser::parse_nginx_access_log};

pub mod parser;
pub mod models;
pub mod evaluator;
pub mod config;
pub mod utils;

pub fn run_nginx(config: &RudraConfig) {
    // spawn nginx as a subprocess
    print_debug_message(config, "Starting nginx");
    let mut nginx_cmd = Command::new("nginx");
    nginx_cmd.arg("-g")
               .arg("daemon off;");

    match nginx_cmd.status() {
        Ok(status) => {
            if !status.success() {
                print_error_and_exit("Error: Unexpected non-zero exit code from nginx");
            }
        }
        Err(err) => {
            print_error_and_exit(format!("Error: Running Nginx failed with: {}", err));
        }
    }
}

pub fn initialize_rudra() -> (RudraConfig, Option<Vec<Endpoint>>) {
    let config = match RudraConfig::from_path(Path::new("./rudra.toml")) {
        Ok(config) => config,
        Err(ConfigurationError::IssueOpeningFile) => print_error_and_exit("An issue opening configuration file (\"rudra.toml\") occured"),
        Err(ConfigurationError::IllegalSyntax(err)) => print_error_and_exit(format!("The configuration file syntax is invalid: {}", err)),
    };

    let openapi_endpoints = match parse_openapi_json(config.environment.openapi_path.as_ref()) {
        Ok(openapi_endpoints) => openapi_endpoints,
        Err(ParsingError::ProblemOpeningFile) => print_error_and_exit("An issue opening the openapi file occured."),
        Err(ParsingError::InvalidSyntax) => print_error_and_exit("The syntax of the openapi file is incorrect."),
        Err(ParsingError::InvalidMethod) => print_error_and_exit("The openapi file contains an invalid method."),
        Err(ParsingError::InvalidStatusCode) => print_error_and_exit("The openapi file contains an invalid status code."),
    };

    (config, Some(openapi_endpoints))
}

pub fn run_eval(config: RudraConfig, openapi_endpoints: Option<Vec<Endpoint>>) {
    print_debug_message(&config, "Evaluating endpoint coverage");

    // TODO replace with dynamic fetch of spec
    let openapi_endpoints = openapi_endpoints.unwrap();

    let nginx_endpoints = match parse_nginx_access_log() {
        Ok(nginx_endpoints) => nginx_endpoints,
        Err(_) => print_error_and_exit("An unexpected error occured while parsing the nginx logs"),
    };

    let endpoint_diff = compare_endpoints(&nginx_endpoints, &openapi_endpoints);

    if !endpoint_diff.len() == 0 {
        print_error_and_exit("Not all endpoints were tested!");
    } else {
        println!("Coverage 100%");
    }
}
