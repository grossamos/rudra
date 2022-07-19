mod json_parser;
mod yaml_parser;
mod nginx_parser;
mod common;

pub use nginx_parser::parse_nginx_access_log;

use crate::{models::Endpoint, utils::{Error, read_file_to_string_or_err}, config::RudraConfig};

use self::{json_parser::parse_json_doc, yaml_parser::parse_yaml_doc};

pub fn parse_openapi(config: &RudraConfig) -> Result<Vec<Endpoint>, Error> {
    println!("{:?}", config.openapi_path.as_os_str());
    let extension = match config.openapi_path.extension() {
        Some(extension) => extension,
        None => return Err(Error::UnknownOpenApiFormat),
    };
    if extension == "json" {
        parse_json_doc(&read_file_to_string_or_err(
                config,
                config.openapi_path.as_ref(),
                Error::ProblemOpeningFile(config.openapi_path.clone()),
                )?)
    } else if extension == "yaml" || extension == "yml" {
        parse_yaml_doc(&read_file_to_string_or_err(
                config,
                config.openapi_path.as_ref(),
                Error::ProblemOpeningFile(config.openapi_path.clone()),
                )?)
    } else {
        Err(Error::UnknownOpenApiFormat)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{utils::test::create_mock_config, parser::parse_openapi};

    #[test]
    fn parses_json_file_correctly() {
        let path = Path::new("./test/resource/swagger.json");
        let mut config = create_mock_config();
        config.openapi_path = Box::from(path);
        assert_eq!(parse_openapi(&config).unwrap().len(), 6);
    }

    #[test]
    fn parses_yaml_file_correctly() {
        let path = Path::new("./test/resource/swagger.yaml");
        let mut config = create_mock_config();
        config.openapi_path = Box::from(path);
        assert_eq!(parse_openapi(&config).unwrap().len(), 6);
    }
}
