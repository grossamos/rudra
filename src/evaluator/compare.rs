
use std::{collections::{HashSet, HashMap}, rc::Rc};

use crate::models::{EndpointConfiguration, Grouping};

pub fn evaluate(
    openapi_endpoints: &Vec<EndpointConfiguration>,
    pre_merge_endpoints: &Option<Vec<EndpointConfiguration>>,
    nginx_endpoints: &Vec<EndpointConfiguration>,
    groupings: &HashSet<Grouping>,
) -> Evaluation {
    let mut matched_groupings = HashMap::new();
    for grouping in groupings {
        matched_groupings.insert(grouping, false);
    }

    for openapi_endpoint in openapi_endpoints {
        for grouping in matched_groupings.iter_mut() {
            if grouping.0.incompases_endpoint_config(openapi_endpoint) {
                *grouping.1 = true;
            }
        }
    }
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

#[cfg(test)]
mod tests {
    //#[test]
    //fn evaluate_groups_two_endpoints
}
