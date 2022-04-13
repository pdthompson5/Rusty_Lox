use std::rc::Rc;
use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::lox_type::LoxValue;
use crate::lox_callable::LoxCallable;
use crate::stmt::Stmt;
use std::fmt;

#[derive(Clone)]
pub struct LoxFunction{
    pub arity: u32,
    pub declaration: Rc<Stmt>
}


impl LoxCallable for LoxFunction{
    fn arity(&self) -> u32 {
        self.arity
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError>{
        //TODO: Impl

        let mut environment = Environment::new_enclosed(interpreter.globals.clone());

        match self.declaration.as_ref(){
            Stmt::Function { name: _, params, body } =>{
                for i in 0..params.len(){
                    environment.define(params.get(i).unwrap().lexeme.clone(), 
                        arguments.get(i).unwrap().clone());
                }
        
                interpreter.execute_block(&body, environment)?;
                Ok(LoxValue::Nil)
            },
            _ => Err(RuntimeError { message: "Error in Parsing: Nonfunction statement in LoxFunction".to_string(), line: 0 })
        }
    }

}


impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && 
        Rc::ptr_eq(&self.declaration, &other.declaration)
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.declaration.as_ref(){
            Stmt::Function{name, params: _, body: _} => write!(f, "<fn {}>", name.lexeme),
            _ => write!(f, "Function formatting error: Improper parsing") //Should never happen
        } 
    }
}