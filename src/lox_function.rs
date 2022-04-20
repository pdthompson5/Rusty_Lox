use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::lox_callable::LoxCallable;
use crate::lox_type::LoxValue;
use crate::stmt::Stmt;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct LoxFunction {
    pub arity: u32,
    pub declaration: Rc<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> u32 {
        self.arity
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError>{
        let mut environment = Environment::new_enclosed(self.closure.clone());

        match self.declaration.as_ref() {
            Stmt::Function {
                name: _,
                params,
                body,
            } => {
                for i in 0..params.len() {
                    environment.define(
                        params.get(i).unwrap().lexeme.clone(),
                        arguments.get(i).unwrap().clone(),
                    );
                }

                match interpreter.execute_block(&body, environment) {
                    //Check for return packaged in a RuntimeError
                    Err(error) => match error.return_value {
                        Some(value) => Ok(value),
                        None => Err(error),
                    },
                    _ => Ok(LoxValue::Nil),
                }
            }
            //This should never happen
            _ => Err(RuntimeError::new(
                "Error in Parsing: Non-function statement in LoxFunction".to_string(),
                0,
            )),
        }
    }
}

//Equality is defined as being the same function in memory 
impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && Rc::ptr_eq(&self.declaration, &other.declaration)
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.declaration.as_ref() {
            Stmt::Function {
                name,
                params: _,
                body: _,
            } => write!(f, "<fn {}>", name.lexeme),
            _ => write!(f, "Function formatting error: Improper parsing"), //Should never happen
        }
    }
}
