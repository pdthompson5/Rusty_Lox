use crate::token::Token;
use crate::lox_type::LoxValue;
//Exprs need to be boxed to avoid reccursive enums which Rust does not allow
pub enum Expr<'a> {
    Binary{left: Box<Expr<'a>>, operator: &'a Token, right: Box<Expr<'a>>}, 
    Grouping{expression: Box<Expr<'a>>},
    Literal{value: LoxValue},
    Unary{operator: &'a Token, right: Box<Expr<'a>>},
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expression : &Box<Expr>) -> T;
    fn visit_literal_expr(&mut self, expr : &LoxValue) -> T;
    fn visit_unary_expr(&mut self, operator : &Token, right : &Box<Expr>) -> T;
}

impl<'a> Expr<'a>{
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T{
        match self{
            Self::Binary { left, operator, right} => visitor.visit_binary_expr(left, operator, right),
            Self::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Self::Literal { value } => visitor.visit_literal_expr(value),
            Self::Unary { operator, right} => visitor.visit_unary_expr(operator, right),
        }
    }
}