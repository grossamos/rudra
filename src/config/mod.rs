use std::{path::Path, sync::Arc};

use url::Url;

mod environment;
mod nginx;

pub use nginx::configure_nginx;

#[derive(Debug)]
pub struct RudraConfig {
    pub debug: bool,
    pub security_accounts_for_forbidden: bool,
    pub security_accounts_for_unautorized: bool,
    pub test_coverage: f32,
    pub runtimes: Vec<Arc<Runtime>>,
    pub is_merge: bool,
    pub only_account_for_merge: bool,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Runtime {
    pub openapi_source: OpenapiSource,
    pub app_base_url: Url,
    pub port: u16,
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            println!(
                " - app_base_url: {}",
                self.runtimes[runtime_index].app_base_url
            );
            println!(" - port: {}", self.runtimes[runtime_index].port);
        }
        println!(
            " - account_for_security: {}",
            self.security_accounts_for_forbidden
        );
        println!(" - test_coverage: {}", self.test_coverage);
    }

    pub fn can_print_merge_info(&self) -> bool {
        if !self.is_merge {
            return false;
        }
        for runtime in &self.runtimes {
            match runtime.openapi_source {
                OpenapiSource::Url(_) => return false,
                OpenapiSource::Path(_) => (),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc, path::Path};

    use reqwest::Url;

    use crate::utils::test::create_mock_config;

    use super::{OpenapiSource, Runtime};

    #[test]
    fn should_only_print_merge_if_is_merge() {
        let mut config = create_mock_config();
        config.is_merge = false;
        config.runtimes = vec![];

        assert!(!config.can_print_merge_info())
    }

    #[test]
    fn should_only_print_merge_if_openapi_source_is_file() {
        let mut config = create_mock_config();
        config.is_merge = true;
        config.runtimes = vec![Arc::new(Runtime {
            openapi_source: OpenapiSource::Url(Url::from_str("https://example.com").unwrap()),
            app_base_url: Url::from_str("https://example.com").unwrap(),
            port: 8080,
        })];

        assert!(!config.can_print_merge_info())
    }

    #[test]
    fn shoul_print_merge_if_is_merge_and_openapi_is_file() {
        let mut config = create_mock_config();
        config.is_merge = true;
        config.runtimes = vec![Arc::new(Runtime {
            openapi_source: OpenapiSource::Path(Box::from(Path::new("./test"))),
            app_base_url: Url::from_str("https://example.com").unwrap(),
            port: 8080,
        })];

        assert!(config.can_print_merge_info())
    }
}
