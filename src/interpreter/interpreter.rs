use std::fmt::Display;

use crate::ast::expr;
use crate::ast::expr::Expr;
use crate::error::{ProcessingErrorHandler, RuntimeError};
use crate::token::types::{Literal, TokenKind};
use crate::token::Token;

#[derive(PartialEq)]
enum Interpretable {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Interpretable {
    fn is_truthy(&self) -> bool {
        match self {
            Interpretable::Boolean(b) => return *b,
            Interpretable::Nil => return false,
            _ => return true,
        }
    }
}

impl Display for Interpretable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interpretable::String(s) => write!(f, "{}", s),
            Interpretable::Number(n) => write!(f, "{}", n),
            Interpretable::Boolean(b) => write!(f, "{}", b),
            Interpretable::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter<'a, ErrorHandler: ProcessingErrorHandler> {
    error_handler: &'a mut ErrorHandler,
}

impl<'a, ErrorHandler: ProcessingErrorHandler> Interpreter<'a, ErrorHandler> {
    pub fn new(error_handler: &'a mut ErrorHandler) -> Self {
        return Interpreter {
            error_handler: error_handler,
        };
    }

    fn evaluate(&self, expression: &Expr) -> Result<Interpretable, RuntimeError> {
        return expression.accept(self);
    }

    pub fn interpret(&mut self, expression: &Expr) -> String{
        match self.evaluate(expression) {
            Ok(value) => return format!("{}", value),
            Err(error) => {
                self.error_handler.runtime_error(error);
                return String::from("");
            }
        }
    }
}

impl<'a, ErrorHandler: ProcessingErrorHandler> expr::Visitor<Result<Interpretable, RuntimeError>>
    for Interpreter<'a, ErrorHandler>
{
    fn visit_binary_expr(
        &self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Interpretable, RuntimeError> {
        let l_eval = self.evaluate(left)?;
        let r_eval = self.evaluate(right)?;

        match (&operator.kind, l_eval, r_eval) {
            (
                TokenKind::Minus,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Number(left_number - right_number));
            }

            (
                TokenKind::Star,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Number(left_number * right_number));
            }

            (
                TokenKind::Slash,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Number(left_number / right_number));
            }

            (
                TokenKind::Plus,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Number(left_number + right_number));
            }

            (TokenKind::Plus, Interpretable::String(s_left), Interpretable::String(s_right)) => {
                return Ok(Interpretable::String(format!("{}{}", s_left, s_right)));
            }

            (
                TokenKind::Greater,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Boolean(left_number > right_number));
            }

            (
                TokenKind::GreaterEqual,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Boolean(left_number >= right_number));
            }

            (
                TokenKind::Less,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Boolean(left_number < right_number));
            }

            (
                TokenKind::LessEqual,
                Interpretable::Number(left_number),
                Interpretable::Number(right_number),
            ) => {
                return Ok(Interpretable::Boolean(left_number <= right_number));
            }

            (TokenKind::BangEqual, l, r) => {
                return Ok(Interpretable::Boolean(l != r));
            }

            (TokenKind::EqualEqual, l, r) => {
                return Ok(Interpretable::Boolean(l == r));
            }

            (_, _, _) => {
                return Err(RuntimeError::interpreter_error(
                    operator.clone(),
                    &format!(
                        "Invalid operands {} and {} to operator '{}'",
                        left, right, operator.lexeme
                    ),
                ));
            }
        }
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> Result<Interpretable, RuntimeError> {
        return self.evaluate(expression);
    }

    fn visit_literal_expr(&self, value: &Literal) -> Result<Interpretable, RuntimeError> {
        match value {
            Literal::Number(n) => return Ok(Interpretable::Number(*n)),
            Literal::Boolean(b) => return Ok(Interpretable::Boolean(*b)),
            Literal::String(s) => return Ok(Interpretable::String(s.clone())),
            Literal::Nil => return Ok(Interpretable::Nil),
        }
    }

    fn visit_unary_expr(
        &self,
        operator: &Token,
        expr: &Expr,
    ) -> Result<Interpretable, RuntimeError> {
        let right = self.evaluate(expr)?;

        match operator.kind {
            TokenKind::Minus => match right {
                Interpretable::Number(n) => return Ok(Interpretable::Number(-n)),
                _ => Err(RuntimeError::interpreter_error(
                    operator.clone(),
                    "Cannot handle '-' on non-number type",
                )),
            },

            TokenKind::Bang => {
                return Ok(Interpretable::Boolean(!right.is_truthy()));
            }

            _ => unreachable!("Tried to match unexpected operator for unary expression."),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::expr::Expr;
    use crate::error::ProcessingErrorHandler;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    use super::Interpreter;

    #[derive(Debug, PartialEq)]
    struct ErrorSpy {
        had_error: bool,
    }

    impl ErrorSpy {
        fn new() -> Self {
            return ErrorSpy { had_error: false };
        }
    }

    impl ProcessingErrorHandler for ErrorSpy {
        fn scanning_error(&mut self, _line: u32, _message: &str) {
            todo!();
        }

        fn parsing_error(&mut self, _line: u32, _location: &str, _message: &str) {
            todo!();
        }
    }

    fn make_expression(source: &str) -> Expr {
        let mut error_handler = ErrorSpy::new();
        let mut scanner = Scanner::new(&source, &mut error_handler);

        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens, &mut error_handler);
        match parser.parse() {
            Ok(expression) => return expression,
            Err(error) => panic!("Unparseable source: {} - {}", source, error),
        }
    }

    struct SourceResultPair {
        source: String,
        result: String,
    }

    impl SourceResultPair {
        fn new(source: &str, result: &str) -> Self {
            return SourceResultPair{source: source.to_string(), result: result.to_string()};
        }
    }

    #[test]
    fn should_correctly_interpret_binary_expressions()
    {
        let mut error_handler = ErrorSpy::new();
        let cases = vec![
            SourceResultPair::new("1 + 2", "3"),
            SourceResultPair::new("1 - 2", "-1"),
            SourceResultPair::new("1 * 2", "2"),
            SourceResultPair::new("1 / 2", "0.5"),
            SourceResultPair::new("\"a\" + \"b\"", "ab"),
            SourceResultPair::new("1 > 1", "false"),
            SourceResultPair::new("1 > 2", "false"),
            SourceResultPair::new("1 >= 1", "true"),
            SourceResultPair::new("1 >= 2", "false"),
            SourceResultPair::new("1 < 1", "false"),
            SourceResultPair::new("1 < 2", "true"),
            SourceResultPair::new("1 <= 1", "true"),
            SourceResultPair::new("2 <= 1", "false"),
            SourceResultPair::new("1 == 1", "true"),
            SourceResultPair::new("1 == 2", "false"),
            SourceResultPair::new("1 != 2", "true"),
            SourceResultPair::new("1 != 1", "false"),
            ];
        let mut interpreter = Interpreter::new(&mut error_handler);

        for case in cases {
            let expr = make_expression(&case.source);
            let interpretation = interpreter.interpret(&expr);

            assert_eq!(interpretation, case.result, "Test failed for {}", case.source);
        }
    }

    #[test]
    fn should_correctly_interpret_grouping_expressions()
    {
        let mut error_handler = ErrorSpy::new();
        let cases = vec![
            SourceResultPair::new("1 + (2 * 5)", "11"),
            SourceResultPair::new("(true != true) == false)", "true"),
            ];
        let mut interpreter = Interpreter::new(&mut error_handler);

        for case in cases {
            let expr = make_expression(&case.source);
            let interpretation = interpreter.interpret(&expr);

            assert_eq!(interpretation, case.result, "Test failed for {}", case.source);
        }
    }
}
