use std::process::{Command, Stdio};

use config::{RudraConfig, configure_nginx};
use evaluator::Evaluation;
use models::EndpointConfiguration;
use parser::parse_openapi;
use utils::print_debug_message;

use crate::{parser::parse_nginx_access_log, utils::print_error_and_exit};

pub mod config;
pub mod evaluator;
pub mod models;
pub mod parser;
pub mod utils;

pub fn run_nginx(config: &RudraConfig) {
    // insert application URL to nginx file
    match configure_nginx(config) {
        Ok(_) => (),
        Err(error) => error.display_error_and_exit(),
    }

    // spawn nginx as a subprocess
    print_debug_message(config, "Starting nginx");
    let mut nginx_cmd = Command::new("nginx");
    nginx_cmd.arg("-g").arg("daemon off;");

    if !config.debug {
        nginx_cmd.stdout(Stdio::null());
    }


    match nginx_cmd.stdout(Stdio::null()).status() {
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

pub fn initialize_rudra() -> (RudraConfig, Option<Vec<EndpointConfiguration>>) {
    let config = match RudraConfig::from_env() {
        Ok(config) => config,
        Err(error) => error.display_error_and_exit(),
    };

    let openapi_endpoints = match parse_openapi(&config) {
        Ok(openapi_endpoints) => openapi_endpoints,
        Err(error) => error.display_error_and_exit(),
    };

    (config, Some(openapi_endpoints))
}

pub fn run_eval(config: &RudraConfig, openapi_endpoints: Option<Vec<EndpointConfiguration>>) -> Evaluation {
    print_debug_message(config, "Evaluating endpoint coverage");

    // TODO replace with dynamic fetch of spec
    let openapi_endpoints = openapi_endpoints.unwrap();

    let nginx_endpoints = match parse_nginx_access_log(config) {
        Ok(nginx_endpoints) => nginx_endpoints,
        Err(_) => print_error_and_exit("An unexpected error occured while parsing the nginx logs"),
    };

    Evaluation::new(&openapi_endpoints, &nginx_endpoints)
}

pub fn publish_results(config: &RudraConfig, eval: &Evaluation) {
    let coverage = eval.calc_test_coverage();
    println!("-------------------");
    println!("       Results     ");
    println!("-------------------");
    eval.print_results();
    println!("Test coverage: {}%", coverage * 100.0);
    if coverage < config.test_coverage {
        print_error_and_exit("Coverage not sufficient");
    } 
}
