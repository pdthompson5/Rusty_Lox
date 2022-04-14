
use std::collections::HashMap;
use crate::{lox_type::LoxValue, interpreter::RuntimeError, token::Token};
use std::cell::RefCell;
use std::rc::Rc;

//TODO: I think the issue is that the enclosing is itself

#[derive(Debug)]
pub struct Environment{
    values : HashMap<String, LoxValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>
}

//I had to find external sources to learn how to implment the environment. 
//I tried extensivley but the way that the environment is both swapped and contains a muttable reference to the
//enclosing environment goes deeply agaist the grain of Rust 

impl Environment{
    pub fn new() -> Self{
        Environment { 
            values: HashMap::new(),
            enclosing: None
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self{
        Environment { 
            values: HashMap::new(), 
            enclosing: Some(enclosing) 
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue){
        self.values.insert(name, value);
    }


    //For now all variables are passed by value. The only value that should be passed by reference is likely a function
    pub fn get(&self, name: &Token) -> Result<LoxValue, RuntimeError>{
        match self.values.get(&name.lexeme){
            Some(val) => Ok(val.clone()),
            //Not found, check enclosing env if it exists
            None => match self.enclosing{
                Some(ref env) => env.borrow().get(name),
                None => Err(RuntimeError::new(
                    ["Undefined variable '".to_string() , name.lexeme.clone(), "'.".to_string()].concat(), 
                    name.line
                ))
            }
        }

    }
    //Currently assign should support pass by reference as long as LoxValue clone does not deep copy 
    //Fix this clone issues, I don't need to clone every time, that is inefficient
    pub fn assign(&mut self, name: &Token, value: &LoxValue) -> Result<(), RuntimeError>{
        let value = value.clone();
        if self.values.contains_key(&name.lexeme){
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else{
            match self.enclosing{
                Some(ref env) => env.borrow_mut().assign(name, &value),
                None => Err(RuntimeError::new(  
                    ["Undefined variable '".to_string() , name.lexeme.clone(), "'.".to_string()].concat(), 
                    name.line
                ))
            }     
        }
    }
}