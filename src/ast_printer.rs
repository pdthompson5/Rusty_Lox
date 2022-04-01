use crate::expr::{Visitor, Expr};
use crate::token::Token;
use crate::lox_type::LoxValue;
struct AstPrinter{

}

impl AstPrinter{
    fn print(&mut self, expr: Expr) -> String{
        return  expr.accept(self);
    }

    // fn parathesize(&self, name : String, ){
    //     let mut 
    // }
}

impl Visitor<String> for AstPrinter{
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> String{
    
    }
    fn visit_grouping_expr(&mut self, expression : &Box<Expr>) -> String{

    }
    fn visit_literal_expr(&mut self, value : &LoxValue) -> String{

    }
    fn visit_unary_expr(&mut self, operator : &Token, expression : &Box<Expr>) -> String{

    }
}