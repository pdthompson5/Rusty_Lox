use std::fmt;
use std::rc::Rc;

use crate::native_function::NativeFunction;
use crate::lox_function::LoxFunction;
#[derive(Clone, PartialEq, Debug)]
pub enum LoxValue{
    Boolean(bool),
    Number(f64),
    LoxString(String),
    Nil,
    Function(Rc<LoxFunction>),
    Native(Rc<NativeFunction>)
}

pub fn stringify_double(val: &f64) -> String{  
    let string = format!("{}", val);
    if string.len() > 2 && string[string.len()-2 .. string.len()-1].eq(".0") {
        return string[.. string.len()-2].to_string();
    } 
    return string;
}

impl fmt::Display for LoxValue{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Self::Boolean(val) => write!(f, "{:?}", val),
            Self::Number(val) => write!(f, "{}", stringify_double(val)),
            Self::LoxString(val) => write!(f, "{}", val),
            Self::Nil => write!(f, "nil"),
            //Todo: Implment function printing
            Self::Function(_func) => write!(f, "Function"),
            Self::Native(_func) => write!(f, "Native Function")
        }           
    }
}

impl LoxValue{
    pub fn is_truthy(&self) -> bool{
        match self{
            Self::Boolean(val) => *val,
            Self::Nil =>  false,
            _ => true
        }
    }

}







