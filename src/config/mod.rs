use std::path::Path;
use serde::{Deserialize, Serialize};
use toml::de::Error;
use url::Url;

use crate::utils::read_file_to_string_or_err;

#[derive(Deserialize)]
pub struct RudraConfig {
    environment: Environment,
}

#[derive(Deserialize)]
pub struct Environment {
    openapi_path: Box<Path>,
    app_base_url: Url,
}

impl RudraConfig {
    fn from_str(config_str: &str) -> Result<RudraConfig, ConfigurationError> {
        match toml::from_str(config_str) {
            Ok(config) => Ok(config),
            Err(err) => Err(ConfigurationError::IllegalSyntax(err)),
        }
    }

    pub fn from_path(path: &Path) -> Result<RudraConfig, ConfigurationError> {
        RudraConfig::from_str(&read_file_to_string_or_err(path, ConfigurationError::IssueOpeningFile)?)
    }
}

#[derive(Debug)]
pub enum ConfigurationError {
    IssueOpeningFile,
    IllegalSyntax(Error),    
} 

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::RudraConfig;

    const CONFIG_STR: &str = r#"
        [environment]
        openapi_path = './test/resource/swagger.json'
        app_base_url = 'http://localhost:8080'
    "#;

    #[test]
    fn can_fetch_valid_path() {
        assert_eq!(RudraConfig::from_str(CONFIG_STR).unwrap().environment.openapi_path.to_str().unwrap(), "./test/resource/swagger.json")
    }

    #[test]
    fn can_fetch_valid_url() {
        assert_eq!(RudraConfig::from_str(CONFIG_STR).unwrap().environment.app_base_url.as_str(), "http://localhost:8080/")
    }

    #[test]
    fn can_read_config_file() {
        // expects this not to panik/have errors
        let path = Path::new("./test/resource/rudra.toml");
        RudraConfig::from_path(path).unwrap();
    }

}
