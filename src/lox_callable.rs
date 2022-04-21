use crate::interpreter::{Interpreter, RuntimeError};
use crate::lox_type::LoxValue;

//This trait defines a callable LoxValue
pub trait LoxCallable{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
    fn arity(&self) -> u32;
}
