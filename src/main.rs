use rudra::{initialize_rudra, run_eval, run_nginx};

fn main() {
    let (config, openapi_endpoints) = initialize_rudra();

    run_nginx(&config);

    run_eval(config, openapi_endpoints);
}
