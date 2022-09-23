use std::{fmt::Display, str::FromStr, sync::Arc};

use crate::{config::Runtime, utils::Error};

use super::misc::Method;

#[derive(Debug, Clone)]
pub struct EndpointConfiguration {
    pub method: Method,
    pub path: OpenapiPath,
    pub status_code: u16,
    pub runtime: Arc<Runtime>,
    pub is_generated: bool,
}

impl EndpointConfiguration {
    pub fn new(
        method: Method,
        openapi_path: &str,
        status_code: u16,
        runtime: Arc<Runtime>,
        is_generated: bool,
    ) -> Result<EndpointConfiguration, Error> {
        Ok(EndpointConfiguration {
            method,
            path: OpenapiPath::from_str(openapi_path)?,
            status_code,
            runtime,
            is_generated,
        })
    }

    pub fn incompases_endpoint(&self, other: EndpointConfiguration) -> bool {
        // if other is not complete
        // i as index of component
        // j as index fro string
        // fixed component:
        // -> j to j + component.len() == component
        //
        // variable component:
        // -> is last: increase j until first /
        // -> is not last: increase j until first / or first occurence of next fixed
        // -> yes: fixed at other starts with same, if equally long increase index, if not add to
        //
        // if j is at end of string -> true

        if self.method != other.method || self.status_code != other.status_code || self.runtime != other.runtime {
            return false;
        }

        let mut parse_index = 0;
        let other_path = other.path.original_source;

        for component_index in 0..self.path.components.len() {
            let component = &self.path.components[component_index];
            match component {
                OpenapiPathComponent::Fixed(fixed) => {
                    if fixed.len() + parse_index > other_path.len()
                        || fixed != &other_path[parse_index..parse_index + fixed.len()]
                    {
                        return false;
                    }
                    parse_index += fixed.len();
                }
                OpenapiPathComponent::Variable => {
                    let next_string = match self.path.components.get(component_index + 1) {
                        Some(next_component) => match next_component {
                            OpenapiPathComponent::Fixed(original_source) => &original_source,
                            OpenapiPathComponent::Variable => "",
                        },
                        None => "",
                    };

                    while parse_index < other_path.len() {
                        if &other_path[parse_index..parse_index + 1] == "/"
                            || (other_path.len() > next_string.len() + parse_index
                                && &other_path[parse_index..parse_index + next_string.len()]
                                    == next_string)
                        {
                            break;
                        }
                        parse_index += 1;
                    }
                }
            }
        }
        
        parse_index == other_path.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenapiPath {
    components: Vec<OpenapiPathComponent>,
    original_source: String,
}

impl Display for OpenapiPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original_source)
    }
}

impl FromStr for OpenapiPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path = vec![];
        let mut current_component = String::new();
        let mut cached_component = String::new();
        let mut is_in_variable = false;

        for character in s.chars() {
            if is_in_variable && character.to_string() == "}" {
                is_in_variable = false;
                if cached_component.len() > 0 {
                    path.push(OpenapiPathComponent::Fixed(cached_component.to_string()));
                    cached_component = String::new();
                }
                path.push(OpenapiPathComponent::Variable);
                current_component = String::new();
            } else if !is_in_variable && character.to_string() == "{" {
                is_in_variable = true;
                cached_component = current_component.clone();
                current_component = String::new();
            } else {
                current_component.push(character);
            }
        }

        if current_component.len() > 0 {
            // deal with opened brackets
            let infix = if is_in_variable { "{" } else { "" };

            path.push(OpenapiPathComponent::Fixed(format!(
                "{}{}{}",
                cached_component, infix, current_component
            )));
        } else if cached_component.len() > 0 {
            path.push(OpenapiPathComponent::Fixed(current_component.to_string()));
        }

        Ok(OpenapiPath {
            components: path,
            original_source: s.to_string(),
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum OpenapiPathComponent {
    Fixed(String),
    Variable,
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use crate::{models::Method, utils::test::create_mock_runtime};

    use super::{EndpointConfiguration, OpenapiPath, OpenapiPathComponent};

    #[test]
    fn parses_fixed_path() {
        let expected = OpenapiPath {
            components: vec![OpenapiPathComponent::Fixed("/foo/bar".to_string())],
            original_source: "/foo/bar".to_string(),
        };
        let got = OpenapiPath::from_str("/foo/bar").unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn parses_variable_path() {
        let expected = OpenapiPath {
            components: vec![
                OpenapiPathComponent::Fixed("/foo/".to_string()),
                OpenapiPathComponent::Variable,
                OpenapiPathComponent::Fixed("/moo".to_string()),
            ],
            original_source: "/foo/{bar}/moo".to_string(),
        };
        let got = OpenapiPath::from_str("/foo/{bar}/moo").unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn ignores_single_opening_bracket() {
        let expected = OpenapiPath {
            components: vec![OpenapiPathComponent::Fixed("/foo/{bar".to_string())],
            original_source: "/foo/{bar".to_string(),
        };
        let got = OpenapiPath::from_str("/foo/{bar").unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn ignores_single_closing_bracket() {
        let expected = OpenapiPath {
            components: vec![OpenapiPathComponent::Fixed("/foo/}bar".to_string())],
            original_source: "/foo/}bar".to_string(),
        };
        let got = OpenapiPath::from_str("/foo/}bar").unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn correctly_identifies_variable_end() {
        let expected = OpenapiPath {
            components: vec![
                OpenapiPathComponent::Fixed("/foo/".to_string()),
                OpenapiPathComponent::Variable,
            ],
            original_source: "/foo/{bar}".to_string(),
        };
        let got = OpenapiPath::from_str("/foo/{bar}").unwrap();
        assert_eq!(expected, got);
    }

    fn get_encopmas_result_from_str(a: &str, b: &str) -> bool {
        let endpoint_cfg_a = EndpointConfiguration::new(
            Method::GET,
            a,
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();
        let endpoint_cfg_b = EndpointConfiguration::new(
            Method::GET,
            b,
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();

        endpoint_cfg_a.incompases_endpoint(endpoint_cfg_b)
    }

    #[test]
    fn fixed_endpoints_encompas_eachother() {
        assert!(get_encopmas_result_from_str("/foo/bar", "/foo/bar"));
    }

    #[test]
    fn expoints_with_different_methods_dont_encompas_eachother() {
        let endpoint_cfg_a = EndpointConfiguration::new(
            Method::PUT,
            "/foo/bar",
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();
        let endpoint_cfg_b = EndpointConfiguration::new(
            Method::GET,
            "/foo/bar",
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();

        assert!(!endpoint_cfg_a.incompases_endpoint(endpoint_cfg_b));
    }

    #[test]
    fn expoints_with_different_status_codes_dont_encompas_eachother() {
        let endpoint_cfg_a = EndpointConfiguration::new(
            Method::GET,
            "/foo/bar",
            400,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();
        let endpoint_cfg_b = EndpointConfiguration::new(
            Method::GET,
            "/foo/bar",
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();

        assert!(!endpoint_cfg_a.incompases_endpoint(endpoint_cfg_b));
    }

    #[test]
    fn expoints_with_different_runtimes_dont_encompas_eachother() {
        let endpoint_cfg_a = EndpointConfiguration::new(
            Method::GET,
            "/foo/bar",
            400,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();
        let endpoint_cfg_b = EndpointConfiguration::new(
            Method::GET,
            "/foo/bar",
            200,
            Arc::from(create_mock_runtime()),
            false,
        )
        .unwrap();

        assert!(!endpoint_cfg_a.incompases_endpoint(endpoint_cfg_b));
    }

    #[test]
    fn dynamic_endpoints_encompas_eachother() {
        assert!(get_encopmas_result_from_str("/foo/{bar}/moo", "/foo/bar/moo"));
    }

    #[test]
    fn different_endpoints_dont_encompas_eachother() {
        assert!(!get_encopmas_result_from_str("/foo/{bar}", "/foo/bar/moo"));
        assert!(!get_encopmas_result_from_str("/foo/{bar}/moo", "/foo/bar"));
    }

    #[test]
    fn same_variable_endpoints_encompas_eachother() {
        assert!(get_encopmas_result_from_str("/foo/{bar}/moo", "/foo/{moo}/moo"));
    }
}
