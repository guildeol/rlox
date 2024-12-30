use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::token::Token;
use crate::{error::RuntimeError, interpreter::Interpretable};

#[derive(Clone)]
pub struct Environment {
    values: ValueMap,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: ValueMap::new(),
            enclosing: None,
        };
    }

    pub fn from(parent: &Environment) -> Self {
        return Environment {
            values: ValueMap::new(),
            enclosing: Some(Rc::new(RefCell::new(parent.clone()))),
        };
    }

    pub fn get(&self, name: &Token) -> Result<Interpretable, RuntimeError> {
        if let Some(v) = self.values.get(name) {
            return Ok(v.clone());
        }

        if let Some(e) = &self.enclosing {
            return e.borrow().get(name);
        }

        Err(RuntimeError::interpreter_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }


    pub fn assign(&mut self, name: &Token, value: &Interpretable) -> Result<Interpretable, RuntimeError> {
        if let Some(v) = self.values.assign(name, value) {
            // Found in the current environment
            return Ok(v.clone());
        }

        if let Some(e) = &self.enclosing {
            return e.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::interpreter_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn define(&mut self, name: String, value: Interpretable) {
        return self.values.define(name, value);
    }
}

#[derive(Clone)]
pub struct ValueMap {
    values: HashMap<String, Interpretable>,
}

impl ValueMap {
    pub fn new() -> Self {
        return ValueMap { values: HashMap::new() };
    }

    pub fn get(&self, name: &Token) -> Option<&Interpretable> {
        return self.values.get(&name.lexeme);
    }

    pub fn assign(&mut self, name: &Token, value: &Interpretable) -> Option<Interpretable> {
        if self.values.contains_key(&name.lexeme) {
            return self.values.insert(name.lexeme.to_string(), value.clone());
        }
        else {
            return None;
        }
    }

    pub fn define(&mut self, name: String, value: Interpretable) {
        self.values.insert(name, value);
    }
}
