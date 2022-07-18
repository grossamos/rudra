use std::path::Path;

use super::print_error_and_exit;

#[derive(Debug)]
pub enum Error {
    InvalidApplicationURL(String),
    MissingEnvironmentVaribles(Vec<String>),
    UnexpectedIOIssue(String),
    InvalidParseSyntax,
    InvalidParseStatusCode(String),
    InvalidParseMethod(String),
    ProblemOpeningFile(Box<Path>),
}

impl Error {
    fn get_error_msg(&self) -> String {
        match self {
            Error::InvalidApplicationURL(err_msg) => format!("Invalid Application URL provided: {}", err_msg),
            Error::MissingEnvironmentVaribles(vars) => format!("Missing the following env variables: {:?}", vars),
            Error::UnexpectedIOIssue(err_msg) => format!("An issue with IO occured: {}", err_msg),
            Error::ProblemOpeningFile(path) => format!("An issue opening the openapi (\"{:?}\") file occured.", path),
            Error::InvalidParseSyntax => format!("The syntax of the openapi file is incorrect."),
            Error::InvalidParseMethod(method) => format!("The openapi file contains an invalid method: {}", method),
            Error::InvalidParseStatusCode(code) => format!("The openapi file contains an invalid status code: {}", code),
        }
    }

    pub fn display_error_and_exit(&self) -> ! {
        print!("Error: ");
        print_error_and_exit(self.get_error_msg())
    }

    pub fn display_error(&self) {
        eprintln!("{}", self.get_error_msg());
    }
}

