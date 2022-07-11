mod json_parser;
mod yml_parser;
mod nginx_parser;

#[derive(Debug)]
enum ParsingError {
    InvalidSyntax,
    InvalidStatusCode,
    InvalidMethod,
    ProblemOpeningFile,
}

#[cfg(test)]
mod test {
}
