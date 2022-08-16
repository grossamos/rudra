use lazy_static::__Deref;

use crate::{
    config::{OpenapiSource, RudraConfig},
    models::EndpointConfiguration,
    utils::{print_endpoints, sort_by_runtime},
};

#[derive(Debug)]
pub struct Evaluation {
    missed_endpoint_configurations: Vec<EndpointConfiguration>,
    missed_openapi_configurations: Vec<EndpointConfiguration>,
    original_openapi_endpoint_count: usize,
}

impl Evaluation {
    pub fn new(
        mut openapi_endpoints: Vec<EndpointConfiguration>,
        mut logged_endpoints: Vec<EndpointConfiguration>,
    ) -> Evaluation {
        let original_openapi_endpoint_count = openapi_endpoints.len();
        remove_matching_endpoints(&mut openapi_endpoints, &mut logged_endpoints);

        let missed_endpoint_configurations = openapi_endpoints;
        let missed_openapi_configurations = logged_endpoints;

        // filter generated endpoints out
        let missed_openapi_configurations = missed_openapi_configurations
            .into_iter()
            .filter(|x| !x.is_generated)
            .collect();

        Evaluation {
            missed_endpoint_configurations,
            missed_openapi_configurations,
            original_openapi_endpoint_count,
        }
    }

    pub fn print_results(&self, config: &RudraConfig) {
        let sorted_missed_endpoint_configs = sort_by_runtime(&self.missed_endpoint_configurations);
        let sorted_missed_openapi_configs = sort_by_runtime(&self.missed_openapi_configurations);
        for runtime in &config.runtimes {
            let mut nothing_missed = true;
            if config.runtimes.len() > 1 {
                println!(
                    "From OpenAPI spec in \"{}\" pointing to endpoint at \"{}\":",
                    match &runtime.openapi_source {
                        OpenapiSource::Url(url) => url.as_str(),
                        OpenapiSource::Path(path) => path.to_str().unwrap_or("<path unknown>"),
                    },
                    runtime.app_base_url.as_str(),
                );
            }
            match sorted_missed_endpoint_configs.get(runtime) {
                Some(missed) => {
                    nothing_missed = false;
                    println!("Missed endpoint configurations:");
                    print_endpoints(missed.iter().map(|x| x.deref()));
                }
                None => (),
            }
            match sorted_missed_openapi_configs.get(runtime) {
                Some(missed) => {
                    nothing_missed = false;
                    println!("The following configurations were logged during the tests, but do not exist in the openapi spec:");
                    print_endpoints(missed.iter().map(|x| x.deref()));
                }
                None => (),
            }
            if nothing_missed {
                println!(" - Nothing missed");
            }
        }
    }

    pub fn calc_test_coverage(&self) -> f32 {
        if self.original_openapi_endpoint_count == 0 {
            return 1.0;
        }
        (self.original_openapi_endpoint_count - self.missed_endpoint_configurations.len()) as f32
            / (self.original_openapi_endpoint_count as f32)
    }

    pub fn has_gateway_issues(&self) -> bool {
        if self.missed_openapi_configurations.len() == 0 {
            return false;
        }
        for endpoint_config in &self.missed_openapi_configurations {
            if endpoint_config.status_code != 502 {
                return false;
            }
        }
        true
    }
}

fn remove_matching_endpoints(
    required_set: &mut Vec<EndpointConfiguration>,
    actual_set: &mut Vec<EndpointConfiguration>,
) {
    required_set.sort();
    actual_set.sort();

    filter_consecutive_duplicates(required_set);
    filter_consecutive_duplicates(actual_set);

    let mut index_required = 0;

    while index_required < required_set.len() {
        let mut index_actual = 0;
        let item_required = required_set.get(index_required).unwrap();

        let mut is_found = false;

        while index_actual < actual_set.len()
            && item_required >= actual_set.get(index_actual).unwrap()
        {
            let item_actual = actual_set.get(index_actual).unwrap();

            if item_required == item_actual {
                is_found = true;
                actual_set.remove(index_actual);
            } else {
                index_actual += 1;
            }
        }

        if is_found {
            required_set.remove(index_required);
        } else {
            index_required += 1;
        }
    }
}

fn filter_consecutive_duplicates<T: PartialEq>(set: &mut Vec<T>) {
    if set.len() == 0 {
        return;
    }

    let mut index = 0;

    while index < set.len() - 1 {
        let reduced_index = false;
        while set.get(index) == set.get(index + 1) {
            set.remove(index);
        }
        if !reduced_index {
            index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use float_eq::assert_float_eq;

    use crate::{
        evaluator::compare::filter_consecutive_duplicates,
        models::{EndpointConfiguration, Method},
        utils::test::create_mock_runtime,
    };

    use super::{remove_matching_endpoints, Evaluation};

    fn create_endpoint_a() -> EndpointConfiguration {
        EndpointConfiguration::new(
            Method::GET,
            String::from("/a"),
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
    }

    fn create_endpoint_b() -> EndpointConfiguration {
        EndpointConfiguration::new(
            Method::POST,
            String::from("/b"),
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
    }

    fn create_endpoint_c() -> EndpointConfiguration {
        EndpointConfiguration::new(
            Method::POST,
            String::from("/c"),
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
    }

    fn create_endpoint_d() -> EndpointConfiguration {
        EndpointConfiguration::new(
            Method::GET,
            String::from("/d"),
            201,
            Arc::from(create_mock_runtime()),
            false,
        )
    }

    #[test]
    fn flags_equal_sets_same_order_as_right() {
        let mut required_set = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let mut actual_set = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 0);
    }

    #[test]
    fn flags_equal_sets_different_order_as_right() {
        let mut required_set = vec![
            create_endpoint_d(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_a(),
        ];

        let mut actual_set = vec![
            create_endpoint_b(),
            create_endpoint_a(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 0);
    }

    #[test]
    fn diff_keeps_non_overlaping_present_in_both_directions() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
        ];

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let mut required_set = set_a.clone();
        let mut actual_set = set_b.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);

        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 1);

        let mut required_set = set_b.clone();
        let mut actual_set = set_a.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);

        assert_eq!(required_set.len(), 1);
        assert_eq!(actual_set.len(), 0);

        let endpoint_d = create_endpoint_d();
        assert!(required_set.iter().any(|x| *x == endpoint_d));
    }

    #[test]
    fn flags_different_sets_have_positive_diff() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
        ];

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_d(),
        ];

        let mut required_set = set_a.clone();
        let mut actual_set = set_b.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 1);
        assert_eq!(actual_set.len(), 1);

        let mut required_set = set_b.clone();
        let mut actual_set = set_a.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 1);
        assert_eq!(actual_set.len(), 1);
    }

    #[test]
    fn specifies_all_different_endpoints() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
        ];

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_d(),
        ];
        let mut required_set = set_a.clone();
        let mut actual_set = set_b.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);

        assert!(required_set.iter().any(|x| *x == create_endpoint_c()));
        assert!(actual_set.iter().any(|x| *x == create_endpoint_d()));
    }

    #[test]
    fn diff_takes_duplicates_into_account() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_a(),
            create_endpoint_a(),
            create_endpoint_a(),
            create_endpoint_b(),
        ];

        let set_b = vec![create_endpoint_a(), create_endpoint_b()];

        let mut required_set = set_a.clone();
        let mut actual_set = set_b.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 0);

        let mut required_set = set_b.clone();
        let mut actual_set = set_a.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 0);
    }

    #[test]
    fn filter_leaves_regular_vecs_unchanged() {
        let mut set_a = vec![1, 2, 3, 4, 5];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn filter_keeps_one_of_duplicates() {
        let mut set_a: Vec<u32> = vec![1, 1];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![1]);
    }

    #[test]
    fn filter_works_with_empty_list() {
        let mut set_a: Vec<u32> = vec![];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![] as Vec<u32>);
    }

    #[test]
    fn filters_consecutive_duplicates() {
        let mut set_a = vec![1, 2, 2, 3, 4, 5];
        filter_consecutive_duplicates(&mut set_a);

        assert_eq!(set_a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn can_process_empty_lists() {
        let set_a = vec![];
        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let mut required_set = set_a.clone();
        let mut actual_set = set_b.clone();
        remove_matching_endpoints(&mut required_set, &mut actual_set);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 4);
    }

    #[test]
    fn empty_configurations_lead_to_full_test_coverage() {
        let openapi_endpoints = vec![];
        let logged_endpoints = vec![];

        let eval = Evaluation::new(openapi_endpoints.clone(), logged_endpoints.clone());
        assert_eq!(eval.calc_test_coverage(), 1.0);

        let logged_endpoints = vec![create_endpoint_a(), create_endpoint_b()];
        let eval = Evaluation::new(openapi_endpoints.clone(), logged_endpoints.clone());
        assert_float_eq!(eval.calc_test_coverage(), 1.0, abs <= 0.0001);
    }

    #[test]
    fn eval_is_relative_to_tested_endpoints() {
        let openapi_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];
        let logged_endpoints = vec![create_endpoint_a(), create_endpoint_b()];

        let eval = Evaluation::new(openapi_endpoints, logged_endpoints);
        assert_float_eq!(eval.calc_test_coverage(), 0.5, abs <= 0.0002);

        let openapi_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];
        let logged_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
        ];

        let eval = Evaluation::new(openapi_endpoints, logged_endpoints);
        assert_float_eq!(eval.calc_test_coverage(), 0.75, abs <= 0.0002);
    }

    #[test]
    fn can_tell_if_proxy_issues() {
        let openapi_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
        ];
        let logged_endpoints = vec![
            EndpointConfiguration {
                method: Method::GET,
                path: "/a".to_string(),
                runtime: Arc::from(create_mock_runtime()),
                status_code: 502,
                is_generated: false,
            },
            EndpointConfiguration {
                method: Method::GET,
                path: "/b".to_string(),
                runtime: Arc::from(create_mock_runtime()),
                status_code: 502,
                is_generated: false,
            },
        ];
        let eval = Evaluation::new(openapi_endpoints, logged_endpoints);
        assert!(eval.has_gateway_issues());
    }

    #[test]
    fn doesnt_flag_valid_as_gateway_issue() {
        let openapi_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];
        let logged_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            EndpointConfiguration {
                method: Method::GET,
                path: "/b".to_string(),
                runtime: Arc::from(create_mock_runtime()),
                status_code: 502,
                is_generated: false,
            },
        ];

        let eval = Evaluation::new(openapi_endpoints, logged_endpoints);
        assert!(!eval.has_gateway_issues());
    }

    #[test]
    fn doesnt_flag_emty_as_gateway_issue() {
        let openapi_endpoints = vec![create_endpoint_a(), create_endpoint_c()];
        let logged_endpoints = vec![];
        let eval = Evaluation::new(openapi_endpoints, logged_endpoints);
        assert!(!eval.has_gateway_issues());
    }

    #[test]
    fn automatically_generated_endpoints_are_ignored_for_missed_openapi() {
        let openapi_endpoints = vec![];
        let logged_endpoints = vec![EndpointConfiguration::new(
            Method::GET,
            "/".to_string(),
            200,
            Arc::new(create_mock_runtime()),
            true,
        )];
        let eval = Evaluation::new(openapi_endpoints, logged_endpoints);
        assert_eq!(eval.missed_openapi_configurations.len(), 0)
    }

    #[test]
    fn can_get_positive_diff_between_merges() {
        // this test is currently duplicate but is needed if we ever change the internl eval logic
        let mut pre_merge = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
        ];

        let mut post_merge = vec![
            create_endpoint_a(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        remove_matching_endpoints(&mut post_merge, &mut pre_merge);

        let new_endpoints = post_merge;
        assert_eq!(new_endpoints, vec![create_endpoint_d()]);
    }
}
