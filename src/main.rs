use rudra::{initialize_rudra, publish_results, run_eval, run_nginx};

fn main() {
    let (config, relevant_openapi_endpoints, other_opeapi_endpoints) = initialize_rudra();

    if config.debug {
        config.print();
    }
    run_nginx(&config);

    let eval = run_eval(&config, relevant_openapi_endpoints);
    publish_results(&config, &eval, other_opeapi_endpoints);
}
