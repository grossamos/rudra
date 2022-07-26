use crate::utils::Error;
use float_eq::float_eq;
use std::{collections::HashMap, env, path::Path, str::FromStr};
use url::Url;

use super::{RudraConfig, OpenapiSource};

const ENV_VAR_APP_BASE_URL: &str = "RUDRA_APP_BASE_URL";
const ENV_VAR_DEBUG: &str = "RUDRA_DEBUG";
const ENV_VAR_OPENAPI_SOURCE: &str = "RUDRA_OPENAPI_SOURCE";
const ENV_VAR_ACCOUNT_FOR_SECURITY: &str = "RUDRA_ACCOUNT_FOR_SECURITY";
const ENV_VAR_TEST_COVERAGE: &str = "RUDRA_TEST_COVERAGE";
const ENV_VAR_PORT: &str = "RUDRA_PORT";

const DEFAULT_TEST_COVERAGE: f32 = 0.7;

impl RudraConfig {
    pub fn from_raw(env_vars: &HashMap<String, String>) -> Result<RudraConfig, Error> {
        // Check if all enviroment variables exist
        let mut missing_keys = vec![];
        if !env_vars.contains_key(ENV_VAR_APP_BASE_URL) {
            missing_keys.push(String::from(ENV_VAR_APP_BASE_URL));
        }
        if !env_vars.contains_key(ENV_VAR_OPENAPI_SOURCE) {
            missing_keys.push(String::from(ENV_VAR_OPENAPI_SOURCE));
        }
        if missing_keys.len() > 0 {
            return Err(Error::MissingEnvironmentVaribles(missing_keys));
        }

        // fetch values from enviroment variables
        let debug = get_bool_env_var(ENV_VAR_DEBUG, env_vars);
        let account_for_security = get_bool_env_var(ENV_VAR_ACCOUNT_FOR_SECURITY, env_vars);
        let openapi_source = match Url::from_str(&env_vars[ENV_VAR_OPENAPI_SOURCE]) {
            Ok(openapi_url) => OpenapiSource::Url(openapi_url),
            Err(_) => OpenapiSource::Path(Box::from(Path::new(&env_vars[ENV_VAR_OPENAPI_SOURCE]))),
        };
        let app_base_url = match Url::from_str(&env_vars[ENV_VAR_APP_BASE_URL]) {
            Ok(app_base_url) => app_base_url,
            Err(parse_error) => return Err(Error::InvalidApplicationURL(parse_error.to_string())),
        };
        let test_coverage = match env_vars.get(ENV_VAR_TEST_COVERAGE) {
            Some(coverage_str) => translate_test_coverage(coverage_str)?,
            None => 0.7,
        };
        let port = match env_vars.get(ENV_VAR_PORT) {
            Some(port_str) => match port_str.parse() {
                Ok(port) => port,
                Err(_) => return Err(Error::InvalidPortNumber(String::from(port_str))),
            },
            _ => 13750,
        };

        Ok(RudraConfig {
            debug,
            openapi_source,
            app_base_url,
            account_for_security,
            test_coverage,
            port,
        })
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
        Some(bool_var) => bool_var.as_str() != "0" && bool_var.as_str() != "" && bool_var.as_str() != "false",
        None => false,
    }
}

fn translate_test_coverage(coverage_str: &str) -> Result<f32, Error> {
    if coverage_str.trim() == "" {
        return Ok(DEFAULT_TEST_COVERAGE);
    }
    let mut coverage = if coverage_str.trim().ends_with("%") {
        match coverage_str[0..coverage_str.len() - 1].parse() {
            Ok(coverage) => coverage,
            Err(_) => return Err(Error::InvalidTestCoverage),
        }
    } else {
        match coverage_str.parse() {
            Ok(coverage) => coverage,
            Err(_) => return Err(Error::InvalidTestCoverage),
        }
    };
    if coverage > 1.0 {
        coverage /= 100.0;
    }
    if float_eq!(coverage, 0.0, abs <= 0.0001) {
        println!("Warning: test coverage is set to 0%");
    }

    if coverage > 1.0 || coverage < 0.0 {
        Err(Error::InvalidTestCoverage)
    } else {
        Ok(coverage)
    }
}

#[cfg(test)]
mod test {
    use float_eq::assert_float_eq;
    use std::{collections::HashMap, path::Path};

    use crate::config::{environment::{
        get_bool_env_var, translate_test_coverage, DEFAULT_TEST_COVERAGE, ENV_VAR_PORT,
    }, OpenapiSource};

    use super::{
        RudraConfig, ENV_VAR_APP_BASE_URL, ENV_VAR_DEBUG, ENV_VAR_OPENAPI_SOURCE,
    };

    fn generate_config_map() -> HashMap<String, String> {
        let mut config_map = HashMap::new();

        config_map.insert(String::from(ENV_VAR_DEBUG), String::from("1"));
        config_map.insert(
            String::from(ENV_VAR_OPENAPI_SOURCE),
            String::from("./test/resource/swagger.json"),
        );
        config_map.insert(
            String::from(ENV_VAR_APP_BASE_URL),
            String::from("http://localhost:8080"),
        );
        config_map
    }

    #[test]
    fn can_fetch_valid_openapi_path() {
        let config_map = generate_config_map();
        assert_eq!(
            RudraConfig::from_raw(&config_map)
                .unwrap()
                .openapi_source,
            OpenapiSource::Path(Box::from(Path::new("./test/resource/swagger.json")))
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
        assert!(RudraConfig::from_raw(&config_map).unwrap().debug);
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

    #[test]
    fn test_coverage_translator_can_recognise_float() {
        assert_float_eq!(
            translate_test_coverage("0.86").unwrap(),
            0.86,
            abs <= 0.0001
        );
    }

    #[test]
    fn test_coverage_recognises_percentage_with_sign() {
        assert_float_eq!(translate_test_coverage("86%").unwrap(), 0.86, abs <= 0.0001);
        assert_float_eq!(
            translate_test_coverage("85.5%").unwrap(),
            0.855,
            abs <= 0.0001
        );
    }

    #[test]
    fn test_coverage_recognises_percentage_without_sign() {
        assert_float_eq!(translate_test_coverage("86").unwrap(), 0.86, abs <= 0.0001);
    }

    #[test]
    fn test_coverage_throws_error_if_over_100_percent() {
        assert!(translate_test_coverage("866").is_err());
    }

    #[test]
    fn test_coverage_throws_error_if_invalid_number() {
        assert!(translate_test_coverage("foo%").is_err());
    }

    #[test]
    fn test_coverage_empty_sting_leads_to_default() {
        assert_eq!(translate_test_coverage("").unwrap(), DEFAULT_TEST_COVERAGE);
    }

    #[test]
    fn defaults_to_70_percent_test_coverage() {
        let config_map = generate_config_map();
        assert_float_eq!(
            RudraConfig::from_raw(&config_map).unwrap().test_coverage,
            0.7,
            abs <= 0.0001
        );
    }

    #[test]
    fn configuration_defaults_to_port_13750() {
        let config_map = generate_config_map();
        assert_eq!(RudraConfig::from_raw(&config_map).unwrap().port, 13750);
    }

    #[test]
    fn configuration_recognises_port_number() {
        let mut config_map = generate_config_map();
        config_map.insert(ENV_VAR_PORT.to_string(), "9999".to_string());
        assert_eq!(RudraConfig::from_raw(&config_map).unwrap().port, 9999);
    }

    #[test]
    fn configuration_throws_error_for_invalid_port() {
        let mut config_map = generate_config_map();
        config_map.insert(ENV_VAR_PORT.to_string(), "albert".to_string());
        assert!(RudraConfig::from_raw(&config_map).is_err());
        config_map.insert(ENV_VAR_PORT.to_string(), "65537".to_string()); // 2^ 16 + 1 (tcp only
                                                                        // allows 16 bits)
        assert!(RudraConfig::from_raw(&config_map).is_err());
    }
}
