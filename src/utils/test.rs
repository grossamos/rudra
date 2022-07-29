use std::{collections::HashMap, str::FromStr};

use reqwest::Url;

use crate::config::{OpenapiSource, RudraConfig, Runtime};

pub fn create_mock_config() -> RudraConfig {
    let mut env_vars = HashMap::new();

    env_vars.insert("RUDRA_DEBUG".to_string(), "1".to_string());
    env_vars.insert(
        "RUDRA_APP_BASE_URL".to_string(),
        "http://example.com".to_string(),
    );
    env_vars.insert("RUDRA_OPENAPI_SOURCE".to_string(), "./example".to_string());

    RudraConfig::from_raw(&env_vars).unwrap()
}

pub fn create_mock_runtime() -> Runtime {
    Runtime {
        openapi_source: OpenapiSource::Url(Url::from_str("https://example.com").unwrap()),
        app_base_url: Url::from_str("https://example.com").unwrap(),
        port: 8080,
    }
}
