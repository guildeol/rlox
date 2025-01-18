use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write;
use std::rc::Rc;

use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::error::{ErrorHandler, ProcessingErrorHandler, RuntimeEvent};
use crate::interpreter::lox_callable::LoxCallable;
use crate::token::types::{Literal, TokenKind};
use crate::token::Token;

use super::lox_callable::{native_clock_call, LoxFunction};
use super::Environment;

#[derive(Clone, Debug, PartialEq)]
pub enum Interpretable {
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(LoxFunction),
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
            Interpretable::String(s) => write!(f, "\"{}\"", s),
            Interpretable::Number(n) => write!(f, "{}", n),
            Interpretable::Boolean(b) => write!(f, "{}", b),
            Interpretable::Callable(c) => write!(f, "{}", c),
            Interpretable::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    error_handler: ErrorHandler,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let environment = Rc::clone(&globals);
        let error_handler = ErrorHandler::new();

        let instance = Interpreter {
            globals,
            environment,
            error_handler,
        };

        let clock_callable = LoxFunction::new_native_function(0, native_clock_call);

        instance
            .globals
            .borrow_mut()
            .define(String::from("clock"), Interpretable::Callable(clock_callable));

        return instance;
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Interpretable, RuntimeEvent> {
        return expression.accept(self);
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Interpretable, RuntimeEvent> {
        return statement.accept(self);
    }

    pub fn execute_block(&mut self, statements: &Vec<Stmt>, enclosing:Environment) -> Result<Interpretable, RuntimeEvent> {
        let mut result = Ok(Interpretable::Nil);

        let previous = self.environment.clone();

        self.environment = Rc::new(RefCell::new(enclosing));
        for statement in statements {
            result = self.execute(statement);

            // Break for error to ensure we clean-up the environment
            if result.is_err() {
                break;
            }
        }

        self.environment = previous;

        return result;
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

impl ExprVisitor<Result<Interpretable, RuntimeEvent>> for Interpreter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Interpretable, RuntimeEvent> {
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
                return Err(RuntimeEvent::interpreter_error(
                    operator.clone(),
                    &format!("Invalid operands {} and {} to operator '{}'", left, right, operator.lexeme),
                ));
            }
        }
    }

    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> Result<Interpretable, RuntimeEvent> {
        let callee_eval = self.evaluate(callee)?;

        let mut args_eval: Vec<Interpretable> = Vec::new();
        for argument in arguments {
            args_eval.push(self.evaluate(argument)?);
        }

        match callee_eval {
            Interpretable::Callable(function) => {
                if arguments.len() != function.arity() {
                    return Err(RuntimeEvent::interpreter_error(
                        paren.clone(),
                        &format!("Expected {}  arguments, but got {}.", function.arity(), arguments.len()),
                    ));
                }

                return function.call(self, &mut args_eval);
            }

            _ => {
                return Err(RuntimeEvent::interpreter_error(
                    paren.clone(),
                    "Can only call functions and classes",
                ))
            }
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Interpretable, RuntimeEvent> {
        return self.evaluate(expression);
    }

    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Interpretable, RuntimeEvent> {
        match value {
            Literal::Number(n) => return Ok(Interpretable::Number(*n)),
            Literal::Boolean(b) => return Ok(Interpretable::Boolean(*b)),
            Literal::String(s) => return Ok(Interpretable::String(s.clone())),
            Literal::Nil => return Ok(Interpretable::Nil),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, expr: &Expr) -> Result<Interpretable, RuntimeEvent> {
        let right = self.evaluate(expr)?;

        match operator.kind {
            TokenKind::Minus => match right {
                Interpretable::Number(n) => return Ok(Interpretable::Number(-n)),
                _ => Err(RuntimeEvent::interpreter_error(
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

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Interpretable, RuntimeEvent> {
        return self.environment.borrow().get(name);
    }

    fn visit_assignment_expr(&mut self, name: &Token, expr: &Expr) -> Result<Interpretable, RuntimeEvent> {
        let value = self.evaluate(expr)?;
        return self.environment.borrow_mut().assign(name, &value);
    }

    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Interpretable, RuntimeEvent> {
        let left_eval = self.evaluate(left)?;

        let is_truthy = left_eval.is_truthy();

        // Handle short-circuit cases
        match operator.kind {
            TokenKind::And => {
                if !is_truthy {
                    return Ok(left_eval);
                }
            }

            TokenKind::Or => {
                if is_truthy {
                    return Ok(left_eval);
                }
            }

            _ => panic!("Unexpected logical operator {}", operator.lexeme),
        }

        return self.evaluate(right);
    }
}

impl StmtVisitor<Result<Interpretable, RuntimeEvent>> for Interpreter {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<Interpretable, RuntimeEvent> {
        return self.evaluate(expr);
    }

    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Stmt>) -> Result<Interpretable, RuntimeEvent> {
        let predicate = self.evaluate(condition)?;
        if predicate.is_truthy() {
            return self.execute(then_branch);
        } else if let Some(else_stmt) = else_branch {
            return self.execute(else_stmt);
        }

        return Ok(Interpretable::Nil);
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<Interpretable, RuntimeEvent> {
        match self.evaluate(expr) {
            Ok(object) => {
                println!("{}", object);
                let _ = std::io::stdout().flush();
                return Ok(object);
            }
            Err(e) => return Err(e),
        }
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<Interpretable, RuntimeEvent> {
        let mut value: Interpretable = Interpretable::Nil;
        if initializer.is_some() {
            value = self.evaluate(initializer.as_ref().unwrap())?;
        }

        self.environment.borrow_mut().define(name.lexeme.clone(), value);
        return Ok(Interpretable::Nil);
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<Interpretable, RuntimeEvent> {
        let mut predicate: Interpretable;

        loop {
            predicate = self.evaluate(condition)?;

            if predicate.is_truthy() {
                let _ = self.execute(body)?;
            } else {
                break;
            }
        }

        return Ok(Interpretable::Nil);
    }

    fn visit_block_stmt(&mut self, declarations: &Vec<Stmt>) -> Result<Interpretable, RuntimeEvent> {
        // Create a new block environment using the current environment as its parent
        let block_environment = Environment::from(Rc::clone(&self.environment));

        // Execute the block
        self.execute_block(declarations, block_environment)
    }

    fn visit_function_stmt(&mut self, name: &Token, parameters: &Vec<Token>, body: &Vec<Stmt>) -> Result<Interpretable, RuntimeEvent> {
        let function = LoxFunction::new_user_function(name, parameters, body);

        self.environment
            .borrow_mut()
            .define(name.to_string(), Interpretable::Callable(function));

        Ok(Interpretable::Nil)
    }

    fn visit_return_stmt(&mut self, _keyword: &Token, value: &Expr) -> Result<Interpretable, RuntimeEvent> {
        let mut result = Interpretable::Nil;

        if *value != Expr::Nil {
            result = self.evaluate(value)?;
        }

        // This is terrible, honestly. It's not an error, but I don't feel like changing the return type for every function.
        // The return type should've been a variant (Interpretable, Error or Return) instead of a Result.
        return Err(RuntimeEvent::new_return(result));
    }
}
