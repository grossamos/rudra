use std::{path::Path, collections::HashMap, str::FromStr, env};
use url::Url;
use crate::utils::Error;

use super::RudraConfig;

const ENV_VAR_APP_BASE_URL: &str = "RUDRA_APP_BASE_URL";
const ENV_VAR_DEBUG: &str = "RUDRA_DEBUG";
const ENV_VAR_OPENAPI_PATH: &str = "RUDRA_OPENAPI_PATH";

impl RudraConfig {
    fn from_raw(env_vars: &HashMap<String, String>) -> Result<RudraConfig, Error> {
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
        let debug = match env_vars.get(ENV_VAR_DEBUG) {
            Some(debug) => debug.as_str() != "0" && debug.as_str() != "",
            None => false,
        };
        let openapi_path = Box::from(Path::new(&env_vars[ENV_VAR_OPENAPI_PATH]));
        let app_base_url = match Url::from_str(&env_vars[ENV_VAR_APP_BASE_URL]) {
            Ok(app_base_url) => app_base_url,
            Err(parse_error) => return Err(Error::InvalidApplicationURL(parse_error.to_string())),
        };

        Ok(RudraConfig{debug, openapi_path, app_base_url})
    }

    pub fn from_env() -> Result<RudraConfig, Error> {
        let mut env_vars = HashMap::new();
        for var in env::vars() {
            env_vars.insert(var.0, var.1);
        }
        RudraConfig::from_raw(&env_vars)
    }
}



#[cfg(test)]
mod test {
    use std::collections::HashMap;
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
    fn nonzero_debug_is_true() {
        let mut config_map = generate_config_map();

        assert!(
            RudraConfig::from_raw(&config_map)
                .unwrap()
                .debug
        );

        config_map.insert(ENV_VAR_DEBUG.to_string(), String::from("2"));
        assert!(
            RudraConfig::from_raw(&config_map)
                .unwrap()
                .debug
        );
    }

    #[test]
    fn zero_or_empty_debug_is_false() {
        let mut config_map = generate_config_map();

        config_map.insert(ENV_VAR_DEBUG.to_string(), String::from("0"));
        assert!(
            !RudraConfig::from_raw(&config_map)
                .unwrap()
                .debug
        );

        config_map.insert(ENV_VAR_DEBUG.to_string(), String::from(""));
        assert!(
            !RudraConfig::from_raw(&config_map)
                .unwrap()
                .debug
        );
    }

    #[test]
    fn non_existant_debug_is_false_no_error() {
        let mut config_map = generate_config_map();
        config_map.remove(ENV_VAR_DEBUG);

        assert!(
            !RudraConfig::from_raw(&config_map)
                .unwrap()
                .debug
        );
    }

}
