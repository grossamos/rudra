use std::process::{Command, Stdio};

use config::{configure_nginx, RudraConfig};
use evaluator::{get_change_from_merge, Evaluation};
use models::EndpointConfiguration;
use parser::{get_openapi_endpoint_configs, get_pre_merge_openapi_endpoints};
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

pub fn initialize_rudra() -> (RudraConfig, Vec<EndpointConfiguration>, Vec<EndpointConfiguration>) {
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
    let relevant_openapi_endpoints;
    let other_openapi_endpoints;

    // filter out impossible szenarios, where they require only_account_for_merge but nothing can
    // be compared
    if config.only_account_for_merge && !config.all_openapi_sources_are_paths() {
        if config.is_merge {
            print_error_and_exit("Your configuration contains a dynamically loaded openapi spec. Rudra needs it to be a local file when only accounting for the difference between commits.");
        } else {
            print_error_and_exit("You need to have two commits to compare (ex. pull/merge request) when only accounting for the difference between commits.");
        }
    } else if config.is_merge && config.only_account_for_merge {
        let mut pre_merge_endpoints = vec![];

        for runtime in &config.runtimes {
            let mut pre_merge_endpoints_of_runtime = match get_pre_merge_openapi_endpoints(runtime.clone()) {
                Ok(endpoints) => endpoints,
                Err(err) => err.display_error_and_exit(),
            };
            pre_merge_endpoints.append(&mut pre_merge_endpoints_of_runtime);
        }

        relevant_openapi_endpoints =
            get_change_from_merge(&openapi_endpoints, &pre_merge_endpoints);
        other_openapi_endpoints = openapi_endpoints;
    } else {
        relevant_openapi_endpoints = openapi_endpoints;
        other_openapi_endpoints = vec![];
    }
    (config, relevant_openapi_endpoints, other_openapi_endpoints)
}

pub fn run_eval(config: &RudraConfig, relevant_openapi_endpoints: Vec<EndpointConfiguration>) -> Evaluation {
    print_debug_message("Evaluating endpoint coverage");

    let nginx_endpoints = match parse_nginx_access_log(&config.runtimes) {
        Ok(nginx_endpoints) => nginx_endpoints,
        Err(_) => print_error_and_exit("An unexpected error occured while parsing the nginx logs"),
    };

    Evaluation::new(relevant_openapi_endpoints, nginx_endpoints)
}

pub fn publish_results(config: &RudraConfig, eval: &Evaluation, other_openapi_endpoints: Vec<EndpointConfiguration>) {
    if eval.has_gateway_issues() {
        println!("Warning: A large number of 502 errors occured. Is your service running?")
    }
    let coverage = eval.calc_test_coverage();
    println!("Results:");
    eval.print_results(config, other_openapi_endpoints);
    println!("Test coverage: {}%", coverage * 100.0);
    if coverage < config.test_coverage {
        print_error_and_exit("Coverage not sufficient");
    }
}
