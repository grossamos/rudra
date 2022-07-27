use crate::{config::{RudraConfig, OpenapiSource}, models::EndpointConfiguration, utils::{Error, print_debug_message}};

use super::{json_parser::parse_json_doc, yaml_parser::parse_yaml_doc};

pub fn fetch_openapi_endpoints_ota(config: &RudraConfig) -> Result<Vec<EndpointConfiguration>, Error> {
    let mut openapi_url = match &config.runtimes[0].openapi_source {
        OpenapiSource::Path(_) => return Err(Error::UnknownInternalError),
        OpenapiSource::Url(openapi_url) => openapi_url.clone(),
    };

    if openapi_url.host_str() == Some("localhost") {
        // unwrap here is fine, since the IP address provided is allways valid
        openapi_url.set_host(Some("172.17.0.1")).unwrap();
    }

    
    // note: using blocking client here because all following steps require it
    let openapi_spec = match reqwest::blocking::get(openapi_url.as_str()) {
        Ok(openapi_response) => match openapi_response.text() {
            Ok(openapi_spec) => openapi_spec,
            Err(why) => {
                print_debug_message(config, format!("{}", why));
                return Err(Error::OpenapiMalformedOnlineComponents)
            }
        },
        Err(why) => {
            print_debug_message(config, format!("{}", why));
            return Err(Error::OpenapiFetchConnectionFailure)
        } 
    };

    // attempt to parse as json -> on syntax err attempt yaml
    match parse_json_doc(&openapi_spec) {
        Ok(endpoints) => Ok(endpoints),
        Err(Error::InvalidParseSyntax) => parse_yaml_doc(&openapi_spec),
        Err(error) => return Err(error),
    }
}
