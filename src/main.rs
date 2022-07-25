use rudra::{initialize_rudra, run_eval, run_nginx, publish_results};

fn main() {
    let (config, openapi_endpoints) = initialize_rudra();

    if config.debug {
        config.print();
    }
    run_nginx(&config);



    let eval = run_eval(&config, openapi_endpoints);
    publish_results(&config, &eval);
}
