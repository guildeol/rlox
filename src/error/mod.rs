use std::fmt::Display;

fn report(line: u32, location: &str, message: &str) {
    eprintln!("line {line} Error{location}: {message}");
}
pub trait ProcessingErrorHandler {
    fn scanning_error(&mut self, _line: u32, _message: &str) {
        panic!("scanning_error uninplemented!");
    }

    fn parsing_error(&mut self, _line: u32, _location: &str, _message: &str) {
        panic!("parsing_error uninplemented!");
    }
}

pub struct ErrorHandler {
    had_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        return Self { had_error: false };
    }
}

impl ProcessingErrorHandler for ErrorHandler {
    fn scanning_error(&mut self, line: u32, message: &str) {
        report(line, "", message);
        self.had_error = true;
    }

    fn parsing_error(&mut self, line: u32, location: &str, message: &str) {
        report(line, location, message);
        self.had_error = true;
    }
}

pub enum Error {
    Parse,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse => write!(f, "ParseError"),
        }
    }
}
