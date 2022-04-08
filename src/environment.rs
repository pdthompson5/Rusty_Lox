
use std::collections::HashMap;
use crate::{lox_type::LoxValue, interpreter::RuntimeError, token::Token};
pub struct Environment{
    values : HashMap<String, LoxValue>
}


impl Environment{
    pub fn new() -> Self{
        Environment { 
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue){
        self.values.insert(name, value);
    }

    //For now all variables are passed by value. The only value that should be passed by reference is likely a function
    pub fn get(&self, name: &Token) -> Result<LoxValue, RuntimeError>{
        match self.values.get(&name.lexeme){
            Some(val) => Ok(val.clone()),
            None => Err(RuntimeError { 
                message: ["Undefined variable '".to_string() , name.lexeme.clone(), "'.".to_string()].concat(), 
                line: name.line
            })
        }

    }
    //Currently assign should support pass by reference as long as LoxValue clone does not deep copy 
    pub fn assign(&mut self, name: &Token, value: &LoxValue) -> Result<(), RuntimeError>{
        let value = value.clone();
        if self.values.contains_key(&name.lexeme){
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else{
            Err(RuntimeError { 
                message: ["Undefined variable '".to_string() , name.lexeme.clone(), "'.".to_string()].concat(), 
                line: name.line
            })        
        }
    }
}