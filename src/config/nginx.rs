use std::{
    fs::{File, OpenOptions},
    path::Path, io::{Read, Write},
};

use crate::utils::Error;
use super::RudraConfig;

pub fn configure_nginx(config: &RudraConfig) -> Result<(), Error> {
    replace_url_in_file(config, Path::new("/etc/nginx/nginx.conf"))
}

fn replace_url(base: &String, url: &str) -> String {
    base.replace("INSERT_URL_HERE", url)
}

fn replace_error_log(base: &String) -> String {
    base.replace("error_log  off;", "error_log  /var/log/nginx/error.log notice;")
}

fn replace_port_number(base: &String, port: u16) -> String {
    base.replace("INSERT_PORT_HERE", &port.to_string())
}

fn open_config_file(path: &Path, for_writing: bool) -> Result<File, Error> {
    match OpenOptions::new().write(for_writing).read(true).truncate(for_writing).open(path) {
        Ok(file) => Ok(file),
        Err(why) => {
            return Err(Error::UnexpectedIOIssue(format!(
                "issue opening file {:?} due to: {}",
                path, why
            )))
        }
    }
}

fn replace_url_in_file(config: &RudraConfig, path: &Path) -> Result<(), Error> {
    let mut file = open_config_file(path, false)?;

    let mut config_string = String::new();
    match file.read_to_string(&mut config_string) {
        Ok(_) => (),
        Err(why) => {
            return Err(Error::UnexpectedIOIssue(format!(
                "issue reading file {:?} due to: {}",
                path, why
            )))
        }
    }

    let mut config_string = replace_url(&config_string, config.app_base_url.as_str());
    if config.debug {
        config_string = replace_error_log(&config_string);
    }
    config_string = replace_port_number(&config_string, config.port);

    let mut file = open_config_file(path, true)?;
    match file.write_all(config_string.as_bytes()) {
        Ok(_) => (),
        Err(why) => {
            return Err(Error::UnexpectedIOIssue(format!(
                "issue writing file {:?} due to: {}",
                path, why
            )))
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Read, Write},
        path::Path, str::FromStr,
    };

    use url::Url;

    use crate::{config::nginx::{replace_url, replace_url_in_file, replace_error_log, replace_port_number}, utils::test::create_mock_config};

    use super::open_config_file;

    #[test]
    fn changes_marker_from_string() {
        let test_string = String::from("proxy_pass INSERT_URL_HERE");
        assert_eq!(
            replace_url(&test_string, "https://example.com"),
            "proxy_pass https://example.com"
        );
    }

    #[test]
    fn replaces_file_correctly() {
        write_default_config();

        let nginx_path = Path::new("./test/resource/nginx.conf");
        let mut config = create_mock_config();
        config.app_base_url = Url::from_str("https://example.com").unwrap();
        replace_url_in_file(&config, &nginx_path).unwrap();
        let mut conf_string = String::from("");
        File::open(&nginx_path)
            .unwrap()
            .read_to_string(&mut conf_string)
            .unwrap();
        assert_eq!(
            conf_string,
            "...some other conf\nproxy_pass https://example.com/\n13750\n...some more conf\n"
        );

        write_default_config();
    }

    fn write_default_config() {
        let mut file = open_config_file(Path::new("./test/resource/nginx.conf"), true).unwrap();
        file.write_all(
            "...some other conf\nproxy_pass INSERT_URL_HERE\nINSERT_PORT_HERE\n...some more conf\n".as_bytes(),
        )
        .unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn replaces_log_when_debug_on() {
        let test_string = String::from("... stuff ... error_log  off; ... stuff ...");
        assert_eq!(
            replace_error_log(&test_string),
            "... stuff ... error_log  /var/log/nginx/error.log notice; ... stuff ..."
        );
    }

    #[test]
    fn repaces_port_number() {
        let test_string = String::from("... stuff ... INSERT_PORT_HERE ... stuff ...");
        assert_eq!(
            replace_port_number(&test_string, 13567),
            "... stuff ... 13567 ... stuff ..."
        );
    }
}
