use crate::interpreter::{Interpreter, RuntimeError};
use crate::lox_type::LoxValue;
use crate::lox_callable::LoxCallable;
use std::fmt;

#[derive(Clone)]
pub struct NativeFunction{
    pub arity: u32,
    pub function: fn(Vec<LoxValue>, &Interpreter) -> LoxValue
}


impl LoxCallable for NativeFunction{
    fn arity(&self) -> u32 {
        self.arity
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError>{
        Ok((self.function)(arguments, interpreter))   
    }
}

//This equality function seems to work quite well: "print clock == clock" -> true
impl PartialEq for NativeFunction {
    //Second half of this is borrowed from https://users.rust-lang.org/t/compare-function-pointers-for-equality/52339
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && 
        (self.function as fn(Vec<LoxValue>, &'static _) -> _ == other.function)
    }
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}