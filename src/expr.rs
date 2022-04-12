use crate::token::Token;
use crate::lox_type::LoxValue;
//Exprs need to be boxed to avoid reccursive enums which Rust does not allow
pub enum Expr<'a> {
    Binary{left: Box<Expr<'a>>, operator: &'a Token, right: Box<Expr<'a>>}, 
    Grouping{expression: Box<Expr<'a>>},
    Literal{value: LoxValue},
    Unary{operator: &'a Token, right: Box<Expr<'a>>},
    Variable{name: &'a Token},
    Assign{name: &'a Token, value: Box<Expr<'a>>},
    Logical{left: Box<Expr<'a>>, operator: &'a Token, right: Box<Expr<'a>>}
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> T;
    fn visit_grouping_expr(&self, expression : &Box<Expr>) -> T;
    fn visit_literal_expr(&self, expr : &LoxValue) -> T;
    fn visit_unary_expr(&self, operator : &Token, right : &Box<Expr>) -> T;
    fn visit_variable_expr(&self, name : &Token) -> T;
    fn visit_assign_expr(&self, name: &Token, value: &Box<Expr>) -> T;
    fn visit_logical_expr(&self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> T;
}

impl<'a> Expr<'a>{
    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T{
        match self{
            Self::Binary { left, operator, right} => visitor.visit_binary_expr(left, operator, right),
            Self::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Self::Literal { value } => visitor.visit_literal_expr(value),
            Self::Unary { operator, right} => visitor.visit_unary_expr(operator, right),
            Self::Variable {name} => visitor.visit_variable_expr(name),
            Self::Assign {name, value} => visitor.visit_assign_expr(name, value),
            Self::Logical { left, operator, right} => visitor.visit_logical_expr(left, operator, right),
        }
    }
}