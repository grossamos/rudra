use crate::{models::EndpointConfiguration, utils::print_endpoints};

#[derive(Debug)]
pub struct Evaluation {
    missed_endpoint_configurations: Vec<EndpointConfiguration>,
    missed_openapi_configurations: Vec<EndpointConfiguration>,
    original_openapi_endpoint_count: usize,
}

impl Evaluation {
    pub fn new(openapi_endpoints: &Vec<EndpointConfiguration>, logged_endpoints: &Vec<EndpointConfiguration>) -> Evaluation {
        let (missed_endpoint_configurations, missed_openapi_configurations) = remove_matching_endpoints(openapi_endpoints, logged_endpoints);
        Evaluation { missed_endpoint_configurations, missed_openapi_configurations, original_openapi_endpoint_count: openapi_endpoints.len() }
    }
    
    pub fn print_results(&self) {
        if self.missed_endpoint_configurations.len() > 0 {
            println!("Missed endpoint configurations:");
            print_endpoints(&mut self.missed_endpoint_configurations.iter());
        }
        if self.missed_openapi_configurations.len() > 0 {
            println!("The following configurations were logged during the tests, but do not exist in the openapi spec:");
            print_endpoints(&mut self.missed_openapi_configurations.iter());
        }
        if self.missed_endpoint_configurations.len() + self.missed_openapi_configurations.len() > 0 {
            println!("");
        }
    }
    
    pub fn calc_test_coverage(&self) -> f32 {
        if self.original_openapi_endpoint_count == 0 {
            return 1.0
        }
        (self.original_openapi_endpoint_count - self.missed_endpoint_configurations.len()) as f32 / (self.original_openapi_endpoint_count as f32)
    }
}


fn remove_matching_endpoints(required_set: &Vec<EndpointConfiguration>, actual_set: &Vec<EndpointConfiguration>) -> (Vec<EndpointConfiguration>, Vec<EndpointConfiguration>) {
    let mut required_set = (*required_set).clone();
    let mut actual_set = (*actual_set).clone();
    required_set.sort();
    actual_set.sort();

    filter_consecutive_duplicates(&mut required_set);
    filter_consecutive_duplicates(&mut actual_set);

    let mut index_required = 0;

    while index_required < required_set.len() {
        let mut index_actual = 0;
        let item_required = required_set.get(index_required).unwrap();

        let mut is_found = false;

        while index_actual < actual_set.len() && item_required >= actual_set.get(index_actual).unwrap() {
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

    (required_set, actual_set)
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
    use float_eq::assert_float_eq;

    use crate::{models::{EndpointConfiguration, Method}, evaluator::compare::filter_consecutive_duplicates};

    use super::{remove_matching_endpoints, Evaluation};

    fn create_endpoint_a() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::GET, String::from("/a"), 200)
    }

    fn create_endpoint_b() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::POST, String::from("/b"), 200)
    }

    fn create_endpoint_c() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::POST, String::from("/c"), 200)
    }

    fn create_endpoint_d() -> EndpointConfiguration {
        EndpointConfiguration::new(Method::GET, String::from("/d"), 201)
    }

    #[test]
    fn flags_equal_sets_same_order_as_right() {
        let required_set = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let actual_set = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let (required_set, actual_set) = remove_matching_endpoints(&required_set, &actual_set);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 0);
    }

    #[test]
    fn flags_equal_sets_different_order_as_right() {
        let required_set = vec![
            create_endpoint_d(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_a(),
        ];

        let actual_set = vec![
            create_endpoint_b(),
            create_endpoint_a(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let (required_set, actual_set) = remove_matching_endpoints(&required_set, &actual_set);
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

        let (required_set, actual_set) = remove_matching_endpoints(&set_a, &set_b);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 1);

        let (required_set, actual_set) = remove_matching_endpoints(&set_b, &set_a);
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

        let (required_set, actual_set) = remove_matching_endpoints(&set_a, &set_b);
        assert_eq!(required_set.len(), 1);
        assert_eq!(actual_set.len(), 1);

        let (required_set, actual_set) = remove_matching_endpoints(&set_b, &set_a);
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
        let (required_set, actual_set) = remove_matching_endpoints(&set_a, &set_b);

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

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
        ];

        let (required_set, actual_set) = remove_matching_endpoints(&set_a, &set_b);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 0);

        let (required_set, actual_set) = remove_matching_endpoints(&set_b, &set_a);
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

        let (required_set, actual_set) = remove_matching_endpoints(&set_a, &set_b);
        assert_eq!(required_set.len(), 0);
        assert_eq!(actual_set.len(), 4);
    }

    // which endpoints are missed
    // what percentage of endpoints are hit
    // which configurations are not covered by openapi
    
    #[test]
    fn empty_configurations_lead_to_full_test_coverage() {
        let openapi_endpoints = vec![];
        let logged_endpoints = vec![];

        let eval = Evaluation::new(&openapi_endpoints, &logged_endpoints);
        assert_eq!(eval.calc_test_coverage(), 1.0);

        let logged_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
        ];
        let eval = Evaluation::new(&openapi_endpoints, &logged_endpoints);
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
        let logged_endpoints = vec![
            create_endpoint_a(),
            create_endpoint_b(),
        ];

        let eval = Evaluation::new(&openapi_endpoints, &logged_endpoints);
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

        let eval = Evaluation::new(&openapi_endpoints, &logged_endpoints);
        assert_float_eq!(eval.calc_test_coverage(), 0.75, abs <= 0.0002);
    }
}
