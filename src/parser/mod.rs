mod json_parser;
mod yaml_parser;
mod nginx_parser;
mod http;
mod common;

pub use nginx_parser::parse_nginx_access_log;
pub use http::fetch_openapi_endpoints_ota;

use crate::{models::EndpointConfiguration, utils::{Error, read_file_to_string_or_err}, config::{RudraConfig, OpenapiSource}};

use self::{json_parser::parse_json_doc, yaml_parser::parse_yaml_doc};

pub fn parse_openapi(config: &RudraConfig) -> Result<Option<Vec<EndpointConfiguration>>, Error> {
    let openapi_path = match &config.openapi_source {
        OpenapiSource::Path(path) => path,
        _ => return Ok(None),
    };
    let extension = match openapi_path.extension() {
        Some(extension) => extension,
        None => return Err(Error::UnknownOpenApiFormat),
    };
    if extension == "json" {
        Ok(Some(parse_json_doc(&read_file_to_string_or_err(
                config,
                openapi_path.as_ref(),
                Error::ProblemOpeningFile(openapi_path.clone()),
                )?)?))
    } else if extension == "yaml" || extension == "yml" {
        Ok(Some(parse_yaml_doc(&read_file_to_string_or_err(
                config,
                openapi_path.as_ref(),
                Error::ProblemOpeningFile(openapi_path.clone()),
                )?)?))
    } else {
        Err(Error::UnknownOpenApiFormat)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{utils::test::create_mock_config, parser::parse_openapi, config::OpenapiSource};

    #[test]
    fn parses_json_file_correctly() {
        let path = Path::new("./test/resource/swagger.json");
        let mut config = create_mock_config();
        config.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(parse_openapi(&config).unwrap().unwrap().len(), 6);
    }

    #[test]
    fn parses_yaml_file_correctly() {
        let path = Path::new("./test/resource/swagger.yaml");
        let mut config = create_mock_config();
        config.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(parse_openapi(&config).unwrap().unwrap().len(), 6);
    }
}
