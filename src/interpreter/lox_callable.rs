use std::{
    fmt::{Debug, Display},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    ast::Stmt,
    error::RuntimeError,
    token::Token,
};

use super::{Environment, Interpretable, Interpreter};

type FunctionBody = fn(&mut Interpreter, &mut Vec<Interpretable>) -> Result<Interpretable, RuntimeError>;

pub trait LoxCallable: Debug + Clone {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Interpretable>) -> Result<Interpretable, RuntimeError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeCallable {
    arity: usize,
    body: Box<FunctionBody>,
}

impl LoxCallable for NativeCallable {
    fn arity(&self) -> usize {
        return self.arity;
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Interpretable>) -> Result<Interpretable, RuntimeError> {
        return (self.body)(interpreter, arguments);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserCallable {
    name: Token,
    parameters: Vec<Token>,
    body: Vec<Stmt>,
}

impl LoxCallable for UserCallable {
    fn arity(&self) -> usize {
        return self.parameters.len();
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Interpretable>) -> Result<Interpretable, RuntimeError> {
        let mut environment = Environment::from(Rc::clone(&interpreter.globals));

        for (param, arg) in self.parameters.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), arg.clone());
        }

        interpreter.execute_block(&self.body, environment)?;

        return Ok(Interpretable::Nil);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoxFunction {
    NativeFunction(NativeCallable),
    UserFunction(UserCallable),
}

impl LoxFunction {
    pub fn new_native_function(arity: usize, body: FunctionBody) -> Self {
        let native_call = NativeCallable {
            arity,
            body: Box::new(body),
        };
        return LoxFunction::NativeFunction(native_call);
    }

    pub fn new_user_function(name: &Token, parameters: &Vec<Token>, body: &Vec<Stmt>) -> Self {
        let user_call = UserCallable {
            name: name.clone(),
            parameters: parameters.clone(),
            body: body.clone(),
        };

        return LoxFunction::UserFunction(user_call);
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        match self {
            LoxFunction::NativeFunction(n) => n.arity(),
            LoxFunction::UserFunction(u) => u.arity()
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Interpretable>) -> Result<Interpretable, RuntimeError> {
        match self {
            LoxFunction::NativeFunction(n) => n.call(interpreter, arguments),
            LoxFunction::UserFunction(u) => u.call(interpreter, arguments)
        }
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxFunction::NativeFunction(_) => return write!(f, "<fn native>"),
            LoxFunction::UserFunction(u) => return write!(f, "<fn {}>", u.name),
        }
    }
}

// Native Call implementations
pub fn native_clock_call(
    _interpreter: &mut Interpreter,
    _args: &mut Vec<Interpretable>,
) -> Result<Interpretable, RuntimeError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!");
    return Ok(Interpretable::Number((now.as_millis() / 1000) as f64));
}
