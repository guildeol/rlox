use std::fmt::Display;

use crate::{interpreter::Interpretable, token::Token};

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

    fn runtime_error(&mut self, _error: RuntimeEvent) {
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

    fn runtime_error(&mut self, error: RuntimeEvent) {
        eprintln!("{}", error);
        self.had_error = true;
    }
}

#[derive(Debug)]
pub enum RuntimeEvent {
    ParseError(String),
    InterpreterError(Token, String),
    Return(Interpretable),
}

impl RuntimeEvent {
    pub fn parse_error(message: &str) -> Self {
        return RuntimeEvent::ParseError(message.to_string());
    }

    pub fn interpreter_error(token: Token, message: &str) -> Self {
        return RuntimeEvent::InterpreterError(token, message.to_string());
    }

    pub fn new_return(value: Interpretable) -> Self {
        return RuntimeEvent::Return(value);
    }
}

impl Display for RuntimeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeEvent::ParseError(msg) => write!(f, "ParseError: {}", msg),
            RuntimeEvent::InterpreterError(token, msg) => {
                write!(f, "[line {}] InterpretError: {} ", token.line, msg)
            }
            RuntimeEvent::Return(v) => write!(f, "Return value: {}", v),
        }
    }
}
