use std::process::{Command, Stdio};

use config::{configure_nginx, RudraConfig};
use evaluator::{Evaluation, create_diff_from_endpoints};
use models::EndpointConfiguration;
use parser::{get_openapi_endpoint_configs, get_pre_merge_openapi_endpoint_configs_from_file};
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
    print_debug_message("Starting nginx");
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

pub fn initialize_rudra() -> (RudraConfig, Vec<EndpointConfiguration>) {
    let config = match RudraConfig::from_env() {
        Ok(config) => config,
        Err(error) => error.display_error_and_exit(),
    };

    let mut openapi_endpoints = vec![];
    for runtime in &config.runtimes {
        let mut endpoints = match get_openapi_endpoint_configs(runtime.clone()) {
            Ok(endpoints) => endpoints,
            Err(err) => err.display_error_and_exit(),
        };
        openapi_endpoints.append(&mut endpoints);
    }

    // filter out impossible szenarios, where they require only_account_for_merge but nothing can
    // be compared
    if config.only_account_for_merge && !config.can_print_merge_info() {
        if config.is_merge {
            print_error_and_exit("Your configuration contains a dynamically loaded openapi spec. Rudra needs it to be a local file when only accounting for the difference between commits.");
        } else {
            print_error_and_exit("You need to have two commits to compare (ex. pull/merge request) when only accounting for the difference between commits.");
        }
    }

    let mut old_openapi_endpoints = vec![];
    for runtime in &config.runtimes {
        let mut endpoints = match get_pre_merge_openapi_endpoint_configs_from_file(runtime.clone()) {
            Ok(endpoints) => endpoints,
            Err(err) => err.display_error_and_exit(),
        };
        old_openapi_endpoints.append(&mut endpoints);
    }

    let _merge_diff_endpoints = create_diff_from_endpoints(&openapi_endpoints, &old_openapi_endpoints);

    (config, openapi_endpoints)
}

pub fn run_eval(config: &RudraConfig, openapi_endpoints: Vec<EndpointConfiguration>) -> Evaluation {
    print_debug_message("Evaluating endpoint coverage");

    let nginx_endpoints = match parse_nginx_access_log(&config.runtimes) {
        Ok(nginx_endpoints) => nginx_endpoints,
        Err(_) => print_error_and_exit("An unexpected error occured while parsing the nginx logs"),
    };

    Evaluation::new(openapi_endpoints, nginx_endpoints)
}

pub fn publish_results(config: &RudraConfig, eval: &Evaluation) {
    if eval.has_gateway_issues() {
        println!("Warning: A large number of 502 errors occured. Is your service running?")
    }
    let coverage = eval.calc_test_coverage();
    println!("Results:");
    eval.print_results(config);
    println!("Test coverage: {}%", coverage * 100.0);
    if coverage < config.test_coverage {
        print_error_and_exit("Coverage not sufficient");
    }
}
