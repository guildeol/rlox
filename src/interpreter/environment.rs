use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::token::Token;
use crate::{error::RuntimeEvent, interpreter::Interpretable};

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub values: ValueMap,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: ValueMap::new(),
            enclosing: None,
        };
    }

    pub fn from(parent: Rc<RefCell<Environment>>) -> Self {
        return Environment {
            values: ValueMap::new(),
            enclosing: Some(parent),
        };
    }

    pub fn get(&self, name: &Token) -> Result<Interpretable, RuntimeEvent> {
        if let Some(v) = self.values.get(name) {
            return Ok(v.clone());
        }

        if let Some(e) = &self.enclosing {
            return e.borrow().get(name);
        }

        Err(RuntimeEvent::interpreter_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: &Interpretable) -> Result<Interpretable, RuntimeEvent> {
        if let Some(v) = self.values.assign(name, value) {
            // Found in the current environment
            return Ok(v.clone());
        }

        if let Some(e) = &self.enclosing {
            return e.borrow_mut().assign(name, value);
        }

        Err(RuntimeEvent::interpreter_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn define(&mut self, name: String, value: Interpretable) -> Option<Interpretable> {
        return self.values.define(name, value);
    }
}

#[derive(Clone, Debug, PartialEq)]
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
        } else {
            return None;
        }
    }

    pub fn define(&mut self, name: String, value: Interpretable) -> Option<Interpretable> {
        return self.values.insert(name, value);
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Environment {{")?;
        for (key, value) in &self.values.values {
            writeln!(f, "  {}: {}", key, value)?;
        }

        if let Some(enclosing) = &self.enclosing {
            writeln!(f, "  Enclosing: {}", enclosing.borrow())?;
        }

        writeln!(f, "}}")
    }
}
