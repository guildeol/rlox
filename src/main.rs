use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser as ClapParser;

mod ast;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

use error::ErrorHandler;
use interpreter::interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

/// Rust based Lox language interpreter
#[derive(ClapParser)]
struct CommandLineArguments {
    /// Lox script to run (optional)
    #[arg()]
    script: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args = CommandLineArguments::parse();

    match &args.script {
        Some(script_path) => {
            println!("Running script: {:?}", script_path);
            // Call your function to run the file here
            return run_file(script_path);
        }
        None => {
            println!("No script provided, entering interactive mode.");
            // Call your function to run the prompt here
            return run_prompt();
        }
    }
}

fn run_file(script_path: &PathBuf) -> ExitCode {
    let content: String = fs::read_to_string(script_path).expect("Failed to read lox script");

    let mut error_handler = ErrorHandler::new();
    let mut scanner = Scanner::new(&content, &mut error_handler);
    let tokens = scanner.scan_tokens();

    if error_handler.had_error {
        return ExitCode::FAILURE;
    }

    let mut parser = Parser::new(tokens, &mut error_handler);
    match parser.parse() {
        Ok(expression) => {
            let mut interpreter = Interpreter::new(&mut error_handler);
            let result = interpreter.interpret(&expression);

            println!("{}", result);
        }
        Err(error) => eprintln!("{}", error),
    }

    return ExitCode::SUCCESS;
}

fn run_prompt() -> ExitCode {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        match stdin.read_line(&mut input) {
            Ok(0) => {
                // If read_line returns Ok(0), this means EOF was reached
                println!("EOF reached. Exiting...");
                return ExitCode::SUCCESS;
            }

            Ok(_) => {
                let mut error_handler: ErrorHandler = ErrorHandler::new();
                let trimmed_input = input.trim();

                let mut scanner = Scanner::new(trimmed_input, &mut error_handler);
                let tokens = scanner.scan_tokens();

                let mut parser = Parser::new(tokens, &mut error_handler);
                match parser.parse() {
                    Ok(expression) => {
                        let mut interpreter = Interpreter::new(&mut error_handler);
                        let result = interpreter.interpret(&expression);

                        println!("{}", result);
                    }
                    Err(error) => eprintln!("{}", error),
                }

                input.clear();
            }

            Err(error) => {
                eprintln!("Error reading input: {}", error);

                return ExitCode::FAILURE;
            }
        }
    }
}
