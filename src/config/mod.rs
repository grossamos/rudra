use std::path::Path;

use url::Url;

mod nginx;
mod environment;

pub use nginx::configure_nginx;

#[derive(Debug)]
pub struct RudraConfig {
    pub debug: bool,
    pub openapi_source: OpenapiSource,
    pub app_base_url: Url,
    pub account_for_security: bool,
    pub test_coverage: f32,
    pub port: u16,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpenapiSource {
    Path(Box<Path>),
    Url(Url),
}

impl RudraConfig {
    pub fn print(&self) {
        println!("Configuration for Rudra:");
        println!(" - debug: {}", self.debug);
        match &self.openapi_source {
            OpenapiSource::Path(path) => println!(" - openapi path: {:?}", path),
            OpenapiSource::Url(url) => print!(" - openapi url: {}", url),
        };
        println!(" - app_base_url: {}", self.app_base_url);
        println!(" - account_for_security: {}", self.account_for_security);
        println!(" - test_coverage: {}", self.test_coverage);
    }
}

