use std::fmt::Display;

use crate::token::Token;

fn report(line: u32, location: &str, message: &str) {
    eprintln!("line {line} Error{location}: {message}");
}
pub trait ProcessingErrorHandler {
    fn scanning_error(&mut self, _line: u32, _message: &str) {
        unimplemented!();
    }

    fn parsing_error(&mut self, _line: u32, _location: &str, _message: &str) {
        unimplemented!();
    }

    fn runtime_error(&mut self, _error: RuntimeError) {
        unimplemented!();
    }
}

pub struct ErrorHandler {
    pub had_error: bool,
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

    fn runtime_error(&mut self, error: RuntimeError) {
        eprintln!("{}", error);
        self.had_error = true;
    }
}

pub enum RuntimeError {
    Parse(String),
    Interpreter(Token, String),
}

impl RuntimeError {
    pub fn parse_error(message: &str) -> Self {
        return RuntimeError::Parse(message.to_string());
    }

    pub fn interpreter_error(token: Token, message: &str) -> Self {
        return RuntimeError::Interpreter(token, message.to_string());
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Parse(msg) => write!(f, "ParseError: {}", msg),
            RuntimeError::Interpreter(token, msg) => {
                write!(f, "[line {}] InterpretError: {} ", token.line, msg)
            }
        }
    }
}
