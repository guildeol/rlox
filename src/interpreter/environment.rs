use std::collections::HashMap;

use crate::token::Token;
use crate::{error::RuntimeError, interpreter::Interpretable};

pub struct Environment {
    values: HashMap<String, Interpretable>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment { values: HashMap::new() };
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
