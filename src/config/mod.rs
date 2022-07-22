use std::path::Path;

use url::Url;

mod nginx;
mod environment;

pub use nginx::configure_nginx;

pub struct RudraConfig {
    pub debug: bool,
    pub openapi_path: Box<Path>,
    pub app_base_url: Url,
    pub account_for_security: bool,
    pub test_coverage: f32,
}

impl RudraConfig {
    pub fn print(&self) {
        println!("Configuration for Rudra:");
        println!(" - debug: {}", self.debug);
        println!(" - openapi_path: {:?}", self.openapi_path);
        println!(" - app_base_url: {}", self.app_base_url);
        println!(" - account_for_security: {}", self.account_for_security);
        println!(" - test_coverage: {}", self.test_coverage);
    }
}

