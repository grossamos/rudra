mod common;
mod http;
mod json_parser;
mod nginx_parser;
mod yaml_parser;

use std::{sync::Arc, path::Path};

pub use nginx_parser::parse_nginx_access_log;

use crate::{
    config::{OpenapiSource, Runtime},
    models::EndpointConfiguration,
    utils::{read_file_to_string_or_err, Error},
};

use self::{json_parser::parse_json_doc, yaml_parser::parse_yaml_doc, http::fetch_openapi_endpoints_for_runtime};

const OPENAPI_MOUNT_POINT: &str = "/repo";

pub fn get_openapi_endpoint_configs(runtime: Arc<Runtime>) -> Result<Vec<EndpointConfiguration>, Error> {
    match runtime.openapi_source {
        OpenapiSource::Url(_) => fetch_openapi_endpoints_for_runtime(runtime),
        OpenapiSource::Path(_) => parse_openapi_file(runtime, OPENAPI_MOUNT_POINT),
    }
}

pub fn parse_openapi_file(runtime: Arc<Runtime>, mount_point: &str) -> Result<Vec<EndpointConfiguration>, Error> {
    let openapi_path = match &runtime.openapi_source {
        OpenapiSource::Path(path) => path,
        OpenapiSource::Url(_) => return Err(Error::UnknownInternalError("open api path read on url".to_string())),
    };
    if openapi_path.has_root() {
        return Err(Error::OpenapiPathIsAbsolute(openapi_path.clone()))
    }
    let openapi_path: Box<Path> = Box::from(Path::new(mount_point).join(openapi_path));
    let extension = match openapi_path.extension() {
        Some(extension) => extension,
        None => return Err(Error::UnknownOpenApiFormat),
    };
    if extension == "json" {
        Ok(parse_json_doc(&read_file_to_string_or_err(
            openapi_path.as_ref(),
            Error::ProblemOpeningFile(openapi_path.clone()),
        )?, runtime)?)
    } else if extension == "yaml" || extension == "yml" {
        Ok(parse_yaml_doc(&read_file_to_string_or_err(
            openapi_path.as_ref(),
            Error::ProblemOpeningFile(openapi_path.clone()),
        )?, runtime)?)
    } else {
        Err(Error::UnknownOpenApiFormat)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use crate::{config::OpenapiSource, parser::parse_openapi_file, utils::test::create_mock_runtime};

    #[test]
    fn parses_json_file_correctly() {
        let path = Path::new("./test/resource/swagger.json");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(parse_openapi_file(Arc::from(runtime), "./").unwrap().len(), 6);
    }

    #[test]
    fn parses_yaml_file_correctly() {
        let path = Path::new("./test/resource/swagger.yaml");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert_eq!(parse_openapi_file(Arc::from(runtime), "./").unwrap().len(), 6);
    }

    #[test]
    fn throws_error_when_providing_absolute_path() {
        let path = Path::new("/test");
        let mut runtime = create_mock_runtime();
        runtime.openapi_source = OpenapiSource::Path(Box::from(path));
        assert!(parse_openapi_file(Arc::from(runtime), "./").is_err())
    }
}
