use super::misc::Method;

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
