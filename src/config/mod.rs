use std::{path::Path, sync::Arc};

use url::Url;

mod nginx;
mod environment;

pub use nginx::configure_nginx;

#[derive(Debug)]
pub struct RudraConfig {
    pub debug: bool,
    pub security_accounts_for_forbidden: bool,
    pub security_accounts_for_unautorized: bool,
    pub test_coverage: f32,
    pub runtimes: Vec<Arc<Runtime>>,
}

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Runtime {
    pub openapi_source: OpenapiSource,
    pub app_base_url: Url,
    pub port: u16,
}

#[derive(Hash)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenapiSource {
    Path(Box<Path>),
    Url(Url),
}

impl RudraConfig {
    pub fn print(&self) {
        println!("Configuration for Rudra:");
        println!(" - debug: {}", self.debug);
        if self.runtimes.len() > 1 {
            println!("- Runtimes:")
        }
        for runtime_index in 0..self.runtimes.len() {
            match &self.runtimes[runtime_index].openapi_source {
                OpenapiSource::Path(path) => println!(" - openapi path: {:?}", path),
                OpenapiSource::Url(url) => print!(" - openapi url: {}", url),
            };
            println!(" - app_base_url: {}", self.runtimes[runtime_index].app_base_url);
            println!(" - port: {}", self.runtimes[runtime_index].port);
        }
        println!(" - account_for_security: {}", self.security_accounts_for_forbidden);
        println!(" - test_coverage: {}", self.test_coverage);
    }
}

