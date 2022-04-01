use crate::expr::Expr;
use crate::expr::Visitor;
use crate::lox_type::LoxValue;
use crate::token::Token;


struct Interpreter {

}

impl Visitor<LoxValue> for Interpreter{
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> LoxValue{
        //TODO: Implement 
        return LoxValue::Nil;
    }
    fn visit_grouping_expr(&mut self, expression : &Box<Expr>) -> LoxValue{
        //TODO: Implement 
        return LoxValue::Nil;
    }
    fn visit_literal_expr(&mut self, value : &LoxValue) -> LoxValue{
        //TODO: Implement 
        return LoxValue::Nil;
    }
    fn visit_unary_expr(&mut self, operator : &Token, expression : &Box<Expr>) -> LoxValue{
        //TODO: Implement 
        return LoxValue::Nil;
    }
}