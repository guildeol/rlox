use std::io;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

mod token;
mod scanner;

use scanner::Scanner;

/// Rust based Lox language interpreter
#[derive(Parser)]
struct CommandLineArguments
{
    /// Lox script to run (optional)
    #[arg()]
    script  : Option<PathBuf>,
}

fn main() -> ExitCode
{
    let args = CommandLineArguments::parse();
    
    match &args.script
    {
        Some(script_path) =>
        {
            println!("Running script: {:?}", script_path);
            // Call your function to run the file here
            return run_file(script_path);
        }
        None =>
        {
            println!("No script provided, entering interactive mode.");
            // Call your function to run the prompt here
            return run_prompt();
        }
    }
}

fn run_file(script_path: &PathBuf) -> ExitCode
{
    let content: String = fs::read_to_string(script_path)
                                .expect("Failed to read lox script");
    
    let mut scanner = Scanner::new(content);

    scanner.scan_tokens();
    
    return ExitCode::SUCCESS;
}

fn run_prompt() -> ExitCode
{
    let stdin = io::stdin();
    let mut input = String::new();
    
    loop 
    {
        print!("> ");
        match stdin.read_line(&mut input)
        {
            Ok(0) =>
            {
                // If read_line returns Ok(0), this means EOF was reached
                println!("EOF reached. Exiting...");
                return ExitCode::SUCCESS;
            }
            
            Ok(_) =>
            {
                let trimmed_input = input.trim();
                
                let mut scanner = Scanner::new(trimmed_input.to_string());
                scanner.scan_tokens();

                input.clear();
            }
            
            Err(error) =>
            {
                eprint!("Error reading input: {}", error);
                
                return ExitCode::FAILURE;
            }
        }
    }    
}

fn error(line: u32, message: &str)
{
    report(line, "".to_string(), message);
}

fn report(line: u32, location: String, message: &str)
{
    println!("line {line} Error{location}: {message}");
}
