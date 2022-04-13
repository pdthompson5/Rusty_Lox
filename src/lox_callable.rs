use crate::interpreter::Interpreter;
use crate::lox_type::LoxValue;

pub trait LoxCallable{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> LoxValue;
    fn arity(&self) -> u32;
}