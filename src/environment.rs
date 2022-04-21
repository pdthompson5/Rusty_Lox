use crate::{interpreter::RuntimeError, lox_type::LoxValue, token::Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    //All variables are passed by value (clone preforms a copy), functions are passed by reference
    pub fn get(&self, name: &Token) -> Result<LoxValue, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            //Not found, check enclosing env if it exists
            None => match self.enclosing {
                Some(ref env) => env.borrow().get(name),
                None => Err(RuntimeError::new(
                    [
                        "Undefined variable '".to_string(),
                        name.lexeme.clone(),
                        "'.".to_string(),
                    ]
                    .concat(),
                    name.line,
                )),
            },
        }
    }

    //get the ancestor enclosing environment at "distance (>0)"
    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        if distance == 1 {
            self.enclosing.as_ref().unwrap().clone()
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow()
                .ancestor(distance - 1)
                .clone()
        }
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<LoxValue, RuntimeError> {
        if distance == 0 {
            match self.values.get(&name.lexeme) {
                Some(val) => Ok(val.clone()),
                None => Err(RuntimeError::new(
                    [
                        "Undefined variable '".to_string(),
                        name.lexeme.clone(),
                        "'.".to_string(),
                    ]
                    .concat(),
                    name.line,
                )),
            }
        } else {
            match self.ancestor(distance).borrow().values.get(&name.lexeme) {
                Some(val) => Ok(val.clone()),
                None => Err(RuntimeError::new(
                    [
                        "Undefined variable '".to_string(),
                        name.lexeme.clone(),
                        "'.".to_string(),
                    ]
                    .concat(),
                    name.line,
                )),
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &LoxValue) -> Result<(), RuntimeError> {
        let value = value.clone();
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            match self.enclosing {
                Some(ref env) => env.borrow_mut().assign(name, &value),
                None => Err(RuntimeError::new(
                    [
                        "Undefined variable '".to_string(),
                        name.lexeme.clone(),
                        "'.".to_string(),
                    ]
                    .concat(),
                    name.line,
                )),
            }
        }
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token,
        value: &LoxValue,
    ) -> Result<(), RuntimeError> {
        let value = value.clone();
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme.clone(), value);
            Ok(())
        }
    }
}
