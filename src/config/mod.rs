use std::path::Path;

use url::Url;

mod nginx;
mod environment;

pub use nginx::configure_nginx;

pub struct RudraConfig {
    pub debug: bool,
    pub openapi_path: Box<Path>,
    pub app_base_url: Url,
}

