use super::misc::Method;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Endpoint {
    pub method: Method,
    pub path: String,
    pub status_code: u16
}

impl Endpoint {
    pub fn new(method: Method, path: String, status_code: u16) -> Endpoint {
        Endpoint { method, path, status_code }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Method;

    use super::Endpoint;

    #[test]
    fn equality_checks_work() {
        let endpoint_a = Endpoint::new(Method::GET, String::from("/test"), 200);
        let endpoint_b = Endpoint::new(Method::GET, String::from("/test"), 200);

        assert!(endpoint_a == endpoint_b);
    }
}
