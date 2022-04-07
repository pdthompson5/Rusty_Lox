use crate::{expr::Expr, token::Token};
pub enum Stmt<'a> {
    Expression{expression: Box<Expr<'a>>}, 
    Print{expression: Box<Expr<'a>>},
    Var{name : &'a Token, initializer: Box<Expr<'a>>}
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, expression: &Box<Expr>) -> T;
    fn visit_print_stmt(&mut self, expression: &Box<Expr>) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Box<Expr>) -> T;
}

impl<'a> Stmt<'a>{
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T{
        match self{ 
            Self::Expression { expression } => visitor.visit_expression_stmt(expression),
            Self::Print { expression } => visitor.visit_print_stmt(expression),
            Self::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
        }
    }
}