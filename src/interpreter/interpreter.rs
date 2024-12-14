use std::fmt::Display;

use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::error::{ProcessingErrorHandler, RuntimeError};
use crate::token::types::{Literal, TokenKind};
use crate::token::Token;

use super::environment::Environment;

#[derive(Clone, PartialEq)]
pub enum Interpretable {
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
    environment: &'a mut Environment,
    error_handler: &'a mut ErrorHandler,
}

impl<'a, ErrorHandler: ProcessingErrorHandler> Interpreter<'a, ErrorHandler> {
    pub fn new(env: &'a mut Environment, error_handler: &'a mut ErrorHandler) -> Self {
        return Interpreter {
            environment: env,
            error_handler: error_handler,
        };
    }

    fn evaluate(&self, expression: &Expr) -> Result<Interpretable, RuntimeError> {
        return expression.accept(self);
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Interpretable, RuntimeError> {
        return statement.accept(self);
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            let result = self.execute(&statement);

            if result.is_err() {
                self.error_handler
                    .runtime_error(result.err().expect("Invalid interpreter error state"));
            }
        }
    }
}

impl<'a, ErrorHandler: ProcessingErrorHandler> ExprVisitor<Result<Interpretable, RuntimeError>> for Interpreter<'a, ErrorHandler> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<Interpretable, RuntimeError> {
        let l_eval = self.evaluate(left)?;
        let r_eval = self.evaluate(right)?;

        match (&operator.kind, l_eval, r_eval) {
            (TokenKind::Minus, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Number(left_number - right_number));
            }

            (TokenKind::Star, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Number(left_number * right_number));
            }

            (TokenKind::Slash, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Number(left_number / right_number));
            }

            (TokenKind::Plus, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Number(left_number + right_number));
            }

            (TokenKind::Plus, Interpretable::String(s_left), Interpretable::String(s_right)) => {
                return Ok(Interpretable::String(format!("{}{}", s_left, s_right)));
            }

            (TokenKind::Greater, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Boolean(left_number > right_number));
            }

            (TokenKind::GreaterEqual, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Boolean(left_number >= right_number));
            }

            (TokenKind::Less, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
                return Ok(Interpretable::Boolean(left_number < right_number));
            }

            (TokenKind::LessEqual, Interpretable::Number(left_number), Interpretable::Number(right_number)) => {
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
                    &format!("Invalid operands {} and {} to operator '{}'", left, right, operator.lexeme),
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

    fn visit_unary_expr(&self, operator: &Token, expr: &Expr) -> Result<Interpretable, RuntimeError> {
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

    fn visit_variable_expr(&self, name: &Token) -> Result<Interpretable, RuntimeError> {
        return self.environment.get(name);
    }
}

impl<'a, ErrorHandler: ProcessingErrorHandler> StmtVisitor<Result<Interpretable, RuntimeError>> for Interpreter<'a, ErrorHandler> {
    fn visit_expr_stmt(&self, expr: &Expr) -> Result<Interpretable, RuntimeError> {
        return self.evaluate(expr);
    }

    fn visit_print_stmt(&self, expr: &Expr) -> Result<Interpretable, RuntimeError> {
        match self.evaluate(expr) {
            Ok(object) => {
                println!("{}", object);
                return Ok(object);
            }
            Err(e) => return Err(e),
        }
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<Interpretable, RuntimeError> {
        let mut value: Interpretable = Interpretable::Nil;
        if initializer.is_some() {
            value = self.evaluate(initializer.as_ref().unwrap())?;
        }

        self.environment.define(name.lexeme.clone(), value);
        return Ok(Interpretable::Nil);
    }
}
