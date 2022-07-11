#[derive(Debug)]
#[derive(PartialEq)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    OPTIONS,
    HEAD,
    PATCH,
    TRACE,
}

impl Method {
    pub fn from_str(method_str: &str) -> Option<Method> {
        match method_str {
            "GET" => Some(Method::GET),
            "PUT" => Some(Method::PUT),
            "POST" => Some(Method::POST),
            "DELETE" => Some(Method::DELETE),
            "OPTIONS" => Some(Method::OPTIONS),
            "HEAD" => Some(Method::HEAD),
            "PATCH" => Some(Method::PATCH),
            "TRACE" => Some(Method::TRACE),
            &_ => None,
        }
    }
}
