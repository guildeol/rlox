use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::token::Token;
use crate::{error::RuntimeError, interpreter::Interpretable};

pub type Enclosing = Rc<RefCell<Environment>>;

pub struct Environment {
    values: HashMap<String, Interpretable>,
    enclosing: Option<Enclosing>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment { values: HashMap::new(), enclosing: None };
    }

    pub fn from(enclosing: &Enclosing) -> Self {
        return Environment {values: HashMap::new(), enclosing: Some(Rc::clone(enclosing))};
    }

    pub fn get(&self, name: &Token) -> Result<Interpretable, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::interpreter_error(
                name.clone(),
                &format!("Undefined variable '{}'.", name.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Interpretable) -> Result<Interpretable, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value.clone());
            return Ok(value);
        } else {
            return Err(RuntimeError::interpreter_error(
                name.clone(),
                &format!("Undefined variable '{}'.", name.lexeme),
            ));
        }
    }

    pub fn define(&mut self, name: String, value: Interpretable) {
        self.values.insert(name, value);
    }
}
