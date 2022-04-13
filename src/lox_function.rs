use std::rc::Rc;
use crate::interpreter::Interpreter;
use crate::lox_type::LoxValue;
use crate::lox_callable::LoxCallable;
use crate::stmt::Stmt;
use std::fmt;

#[derive(Clone)]
pub struct LoxFunction{
    arity: u32,
    declaration: Rc<Stmt>
}
//TODO: Instead of Boxes use RCs for everything so this is not an issue 


impl LoxCallable for LoxFunction{
    fn arity(&self) -> u32 {
        self.arity
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>) -> LoxValue{
        //TODO: Impl
        LoxValue::Nil
    }

}


impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && 
        Rc::ptr_eq(&self.declaration, &other.declaration)
    }
}

//TODO: Determine if I want to print more. Realistically there is little use case to print a native function
impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.declaration.as_ref(){
            Stmt::Function{name, params: _, body: _} => write!(f, "<fn {}>", name.lexeme),
            _ => write!(f, "Function formatting error: Improper parsing") //Should never happen
        } 
    }
}