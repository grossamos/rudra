use crate::models::Endpoint;

pub fn compare_endpoints(set_a: &Vec<Endpoint>, set_b: &Vec<Endpoint>) -> bool {
    let mut set_a = (*set_a).clone();
    let mut set_b = (*set_b).clone();
    set_a.sort();
    set_b.sort();

    set_a.iter().zip(set_b).filter(|(a, b)| a != &b).count() == 0
}

#[cfg(test)]
mod test {
    use crate::models::{Endpoint, Method};

    use super::compare_endpoints;

    fn create_endpoint_a() -> Endpoint {
        Endpoint::new(Method::GET, String::from("/"), 200)
    }

    fn create_endpoint_b() -> Endpoint {
        Endpoint::new(Method::POST, String::from("/"), 200)
    }

    fn create_endpoint_c() -> Endpoint {
        Endpoint::new(Method::POST, String::from("/99"), 200)
    }

    fn create_endpoint_d() -> Endpoint {
        Endpoint::new(Method::GET, String::from("/99"), 201)
    }

    #[test]
    fn flags_equal_sets_same_order_as_right() {
        let set_a = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        let set_b = vec![
            create_endpoint_a(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        assert!(compare_endpoints(&set_a, &set_b));
    }

    #[test]
    fn flags_equal_sets_different_order_as_right() {
        let set_a = vec![
            create_endpoint_d(),
            create_endpoint_b(),
            create_endpoint_c(),
            create_endpoint_a(),
        ];

        let set_b = vec![
            create_endpoint_b(),
            create_endpoint_a(),
            create_endpoint_c(),
            create_endpoint_d(),
        ];

        assert!(compare_endpoints(&set_a, &set_b));
    }

    #[test]
    fn flags_different_lengths_as_incorrect() {
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

        assert!(!compare_endpoints(&set_a, &set_b));
        assert!(!compare_endpoints(&set_b, &set_a));
    }

    #[test]
    fn flags_different_sets_as_incorrect() {
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

        assert!(!compare_endpoints(&set_a, &set_b));
        assert!(!compare_endpoints(&set_b, &set_a));
    }
}
