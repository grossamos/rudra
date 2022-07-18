use std::{path::Path, fs::File, io::{BufReader, BufRead}};

use crate::{models::{Endpoint, Method}, utils::{Error, print_debug_message}, config::RudraConfig};
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_nginx_access_log(config: &RudraConfig) -> Result<Vec<Endpoint>, Error> {
    parse_access_log(config, Path::new("/var/log/nginx/access.log"))
}

fn parse_access_log(config: &RudraConfig, path: &Path) -> Result<Vec<Endpoint>, Error> {
    let mut endpoints = Vec::new();
    let reader = match File::open(path) {
        Ok(file) => BufReader::new(file),
        Err(why) => {
            print_debug_message(config, why.to_string());
            return Err(Error::ProblemOpeningFile(Box::from(path)));
        }
    };

    for line in reader.lines() {
        let line_str = match line {
            Ok(line_str) => line_str,
            Err(why) => {
                print_debug_message(config, why.to_string());
                return Err(Error::ProblemOpeningFile(Box::from(path)));
            }
        };

        endpoints.push(parse_nginx_line(&line_str)?);
    }

    Ok(endpoints)
}

fn parse_nginx_line(line: &str) -> Result<Endpoint, Error> {
    lazy_static! {
        static ref NGINX_LINE_REGEX: Regex = Regex::new("^(\\[.+\\]) \"(\\w{3, 4}) (/\\S*) HTTP/\\d\\.\\d\" (\\d{3})").unwrap();
    }

    let captures = match NGINX_LINE_REGEX.captures(line) {
        Some(captures) => captures,
        None => return Err(Error::InvalidParseSyntax)
    };

    let status = {
        let status_string = match captures.get(4) {
            Some(status_string) => status_string,
            None => return Err(Error::InvalidParseSyntax)
        };

        match status_string.as_str().parse() {
            Ok(status) => status,
            Err(..) => return Err(Error::InvalidParseStatusCode(status_string.as_str().to_string())),
        }
    };

    let method = {
        let method_string = match captures.get(2) {
            Some(method_string) => method_string.as_str(),
            None => return Err(Error::InvalidParseSyntax),
        };

        match Method::from_str(method_string) {
            Some(method) => method,
            None => return Err(Error::InvalidParseMethod(method_string.to_string())),
        }
    };

    let path = match captures.get(3) {
        Some(path) => String::from(path.as_str()),
        None => return Err(Error::InvalidParseSyntax),
    };

    Ok(Endpoint::new(method, path, status))

}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{parser::nginx_parser::{parse_nginx_line, parse_access_log}, models::Method, utils::test::create_mock_config};

    #[test]
    fn parses_correct_status() {
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200").unwrap().status_code, 200);
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:52:45 +0000] \"GET /usus HTTP/1.1\" 404").unwrap().status_code, 404);
    }

    #[test]
    fn parses_correct_method() {
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200").unwrap().method, Method::GET);
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:50:03 +0000] \"POST /weather HTTP/1.1\" 200").unwrap().method, Method::POST);
    }

    #[test]
    fn parses_correct_path() {
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:50:03 +0000] \"GET /weather HTTP/1.1\" 200").unwrap().path, String::from("/weather"));
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:52:45 +0000] \"GET /usus HTTP/1.1\" 404").unwrap().path, String::from("/usus"));
        assert_eq!(parse_nginx_line("[11/Jul/2022:08:52:45 +0000] \"GET / HTTP/1.1\" 404").unwrap().path, String::from("/"));
    }

    #[test]
    fn parses_full_access_log() {
        let path = Path::new("./test/resource/access.log");
        assert_eq!(parse_access_log(&create_mock_config(), &path).unwrap().len(), 9);
    }
}
