
use crate::models::EndpointConfiguration;

pub fn evaluate(
    openapi_endpoints: Vec<EndpointConfiguration>,
    pre_merge_endpoints: Option<Vec<EndpointConfiguration>>,
    nginx_endpoints: Vec<EndpointConfiguration>,
) -> Evaluation {
    // start with openapi_endpoints
    // check if openapi_endpoint matches group
    // if group is alread matched -> skip
    // check if matches any nginx_endpoint -> if matches mark nginx_endpoint as matched
    // -> if it does not add to missed endpoints list
    // if matches group -> tag group as matched
    
    Evaluation { has_gateway_issues: false, test_coverage: 1.0, endpoints_missing_in_spec: vec![] }
}

#[derive(Debug)]
pub struct Evaluation {
    has_gateway_issues: bool,
    test_coverage: f32,
    endpoints_missing_in_spec: Vec<EndpointConfiguration>,
}

