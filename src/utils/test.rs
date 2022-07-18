use std::collections::HashMap;

use crate::config::RudraConfig;

pub fn create_mock_config() -> RudraConfig {
    let mut env_vars = HashMap::new();

    env_vars.insert("RUDRA_DEBUG".to_string(), "1".to_string());
    env_vars.insert("RUDRA_APP_BASE_URL".to_string(), "http://example.com".to_string());
    env_vars.insert("RUDRA_OPENAPI_PATH".to_string(), "./example".to_string());

    RudraConfig::from_raw(&env_vars).unwrap()
}
