use std::fmt;


pub enum LoxValue{
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,


}

pub fn stringify_double(val: &f64) -> String{  
    let string = format!("{}", val);
    if string[string.len()-2 .. string.len()-1].eq(".0") {
        return string[.. string.len()-2].to_string();
    } 
    return string;
}

impl fmt::Display for LoxValue{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Self::Boolean(val) => write!(f, "{:?}", val),
            Self::Number(val) => write!(f, "{:?}", stringify_double(val)),
            Self::String(val) => write!(f, "{:?}", val),
            Self::Nil => write!(f, "nil"),
        }           
    }
}

