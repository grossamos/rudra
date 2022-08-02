use std::sync::Arc;

use crate::config::Runtime;

use super::misc::Method;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
#[derive(Hash)]
pub struct EndpointConfiguration {
    pub method: Method,
    pub path: String,
    pub status_code: u16,
    pub runtime: Arc<Runtime>,
    pub is_generated: bool
}

impl EndpointConfiguration {
    pub fn new(method: Method, path: String, status_code: u16, runtime: Arc<Runtime>, is_generated: bool) -> EndpointConfiguration {
        EndpointConfiguration { method, path, status_code, runtime, is_generated}
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{models::Method, utils::test::create_mock_runtime};

    use super::EndpointConfiguration;

    #[test]
    fn equality_checks_work() {
        let endpoint_a = EndpointConfiguration::new(Method::GET, String::from("/test"), 200, Arc::from(create_mock_runtime()), false);
        let endpoint_b = EndpointConfiguration::new(Method::GET, String::from("/test"), 200, Arc::from(create_mock_runtime()), false);

        assert!(endpoint_a == endpoint_b);
    }
}
