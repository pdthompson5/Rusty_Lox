use std::fmt;
use std::rc::Rc;

use crate::lox_function::LoxFunction;
use crate::native_function::NativeFunction;

//This enum represents all possible values in Lox. They must be enumerated as Rust is statically typed.
#[derive(Clone, PartialEq, Debug)]
pub enum LoxValue {
    Boolean(bool),
    Number(f64),
    LoxString(String),
    Nil,
    Function(Rc<LoxFunction>),
    Native(Rc<NativeFunction>),
}

pub fn stringify_double(val: &f64) -> String {
    let string = format!("{}", val);
    //Trim off trailing zeroes 
    if string.len() > 2 && string[string.len() - 2..string.len() - 1].eq(".0") {
        return string[..string.len() - 2].to_string();
    }
    return string;
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(val) => write!(f, "{:?}", val),
            Self::Number(val) => write!(f, "{}", stringify_double(val)),
            Self::LoxString(val) => write!(f, "{}", val),
            Self::Nil => write!(f, "nil"),
            Self::Function(func) => write!(f, "{:?}", func),
            Self::Native(func) => write!(f, "{:?}", func),
        }
    }
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        //Nil is false, Boolean is value, all others are true
        match self {
            Self::Boolean(val) => *val,
            Self::Nil => false,
            _ => true,
        }
    }
}
