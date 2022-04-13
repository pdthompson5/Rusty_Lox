use std::rc::Rc;
use crate::interpreter::Interpreter;
use crate::lox_type::LoxValue;
use crate::lox_callable::LoxCallable;
use crate::stmt::Stmt;

#[derive(Clone, PartialEq, Debug)]
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