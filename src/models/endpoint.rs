use super::misc::Method;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EndpointConfiguration {
    pub method: Method,
    pub path: String,
    pub status_code: u16
}

impl EndpointConfiguration {
    pub fn new(method: Method, path: String, status_code: u16) -> EndpointConfiguration {
        EndpointConfiguration { method, path, status_code }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Method;

    use super::EndpointConfiguration;

    #[test]
    fn equality_checks_work() {
        let endpoint_a = EndpointConfiguration::new(Method::GET, String::from("/test"), 200);
        let endpoint_b = EndpointConfiguration::new(Method::GET, String::from("/test"), 200);

        assert!(endpoint_a == endpoint_b);
    }
}
