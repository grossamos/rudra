use std::{path::Path, collections::HashMap, str::FromStr, env};
use url::Url;
use crate::utils::Error;

use super::RudraConfig;

const ENV_VAR_APP_BASE_URL: &str = "RUDRA_APP_BASE_URL";
const ENV_VAR_DEBUG: &str = "RUDRA_DEBUG";
const ENV_VAR_OPENAPI_PATH: &str = "RUDRA_OPENAPI_PATH";
const ENV_VAR_ACCOUNT_FOR_SECURITY: &str = "RUDRA_ACCOUNT_FOR_SECURITY";

impl RudraConfig {
    pub fn from_raw(env_vars: &HashMap<String, String>) -> Result<RudraConfig, Error> {
        // Check if all enviroment variables exist
        let mut missing_keys = vec![];
        if !env_vars.contains_key(ENV_VAR_APP_BASE_URL) {
            missing_keys.push(String::from(ENV_VAR_APP_BASE_URL));
        }
        if !env_vars.contains_key(ENV_VAR_OPENAPI_PATH) {
            missing_keys.push(String::from(ENV_VAR_OPENAPI_PATH));
        }
        if missing_keys.len() > 0 {
            return Err(Error::MissingEnvironmentVaribles(missing_keys));
        }

        // fetch values from enviroment variables
        let debug = get_bool_env_var(ENV_VAR_DEBUG, env_vars);
        let account_for_security = get_bool_env_var(ENV_VAR_ACCOUNT_FOR_SECURITY, env_vars);
        let openapi_path = Box::from(Path::new(&env_vars[ENV_VAR_OPENAPI_PATH]));
        let app_base_url = match Url::from_str(&env_vars[ENV_VAR_APP_BASE_URL]) {
            Ok(app_base_url) => app_base_url,
            Err(parse_error) => return Err(Error::InvalidApplicationURL(parse_error.to_string())),
        };

        Ok(RudraConfig{debug, openapi_path, app_base_url, account_for_security})
    }

    pub fn from_env() -> Result<RudraConfig, Error> {
        let mut env_vars = HashMap::new();
        for var in env::vars() {
            env_vars.insert(var.0, var.1);
        }
        RudraConfig::from_raw(&env_vars)
    }
}

fn get_bool_env_var(key: &str, env_vars: &HashMap<String, String>) -> bool {
    match env_vars.get(key) {
        Some(debug) => debug.as_str() != "0" && debug.as_str() != "",
        None => false,
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::config::environment::get_bool_env_var;

    use super::{RudraConfig, ENV_VAR_APP_BASE_URL, ENV_VAR_DEBUG, ENV_VAR_OPENAPI_PATH};


    fn generate_config_map() -> HashMap<String, String> {
        let mut config_map = HashMap::new();

        config_map.insert(String::from(ENV_VAR_DEBUG), String::from("1"));
        config_map.insert(String::from(ENV_VAR_OPENAPI_PATH), String::from("./test/resource/swagger.json"));
        config_map.insert(String::from(ENV_VAR_APP_BASE_URL), String::from("http://localhost:8080"));
        config_map
    }

    #[test]
    fn can_fetch_valid_openapi_path() {
        let config_map = generate_config_map();
        assert_eq!(
            RudraConfig::from_raw(&config_map)
                .unwrap()
                .openapi_path
                .to_str()
                .unwrap(),
            "./test/resource/swagger.json"
        );
    }

    #[test]
    fn can_fetch_valid_url() {
        assert_eq!(
            RudraConfig::from_raw(&generate_config_map())
                .unwrap()
                .app_base_url
                .as_str(),
            "http://localhost:8080/"
        );
    }

    #[test]
    fn can_catch_invalid_url() {
        let mut config_map = generate_config_map();
        config_map.insert(ENV_VAR_APP_BASE_URL.to_string(), String::from("jjjjjj"));
        match RudraConfig::from_raw(&config_map) {
            Ok(_) => panic!("Should throw error here"),
            Err(_) => (),
        }
    }

    #[test]
    fn missing_keys_lead_to_err() {
        let mut config_map = generate_config_map();
        config_map.remove(ENV_VAR_APP_BASE_URL);
        match RudraConfig::from_raw(&config_map) {
            Ok(_) => panic!("Should throw error here"),
            Err(_) => (),
        }

    }

    #[test]
    fn nonzero_bool_is_true() {
        let mut config_map = generate_config_map();
        assert!(get_bool_env_var(ENV_VAR_DEBUG, &config_map));
        config_map.insert(ENV_VAR_DEBUG.to_string(), String::from("2"));
        assert!(get_bool_env_var(ENV_VAR_DEBUG, &config_map));
    }

    #[test]
    fn zero_or_empty_bool_is_false() {
        let mut config_map = generate_config_map();

        config_map.insert(ENV_VAR_DEBUG.to_string(), String::from("0"));
        assert!(!get_bool_env_var(ENV_VAR_DEBUG, &config_map));

        config_map.insert(ENV_VAR_DEBUG.to_string(), String::from(""));
        assert!(!get_bool_env_var(ENV_VAR_DEBUG, &config_map));
    }

    #[test]
    fn non_existant_bool_is_false_no_error() {
        let mut config_map = generate_config_map();
        config_map.remove(ENV_VAR_DEBUG);
        assert!(!get_bool_env_var(ENV_VAR_DEBUG, &config_map));
    }

    #[test]
    fn debug_val_is_used() {
        let config_map = generate_config_map();
        assert!(
            RudraConfig::from_raw(&config_map)
                .unwrap()
                .debug
        );
    }

    #[test]
    fn account_for_security_val_is_used() {
        let config_map = generate_config_map();
        assert!(
            !RudraConfig::from_raw(&config_map)
                .unwrap()
                .account_for_security
        );
    }

}
