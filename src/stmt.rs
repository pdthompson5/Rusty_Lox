use crate::{expr::Expr, token::Token};
pub enum Stmt<'a> {
    Expression{expression: Box<Expr<'a>>}, 
    Print{expression: Box<Expr<'a>>},
    Var{name : &'a Token, initializer: Box<Expr<'a>>},
    Block{statements: Vec<Box<Stmt<'a>>>},
    If{condition: Box<Expr<'a>>, then_branch: Box<Stmt<'a>>, else_branch: Option<Box<Stmt<'a>>>},
    While{condition: Box<Expr<'a>>, body: Box<Stmt<'a>>},
    Function{name: &'a Token, params: Vec<&'a Token>, body: Vec<Box<Stmt<'a>>>}
    
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&self, expression: &Box<Expr>) -> T;
    fn visit_print_stmt(&self, expression: &Box<Expr>) -> T;
    fn visit_var_stmt(&self, name: &Token, initializer: &Box<Expr>) -> T;
    fn visit_block_stmt(&self, statements: &Vec<Box<Stmt>>) -> T;
    fn visit_if_stmt(&self, condition: &Box<Expr>, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> T;
    fn visit_while_stmt(&self, condition: &Box<Expr>, body: &Box<Stmt>) -> T;
    fn visit_function_stmt(&self, name: &Token, params: &Vec<&Token>, body: &Vec<Box<Stmt>>) -> T;
}

impl<'a> Stmt<'a>{
    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T{
        match self{ 
            Self::Expression {expression } => visitor.visit_expression_stmt(expression),
            Self::Print {expression } => visitor.visit_print_stmt(expression),
            Self::Var {name, initializer } => visitor.visit_var_stmt(name, initializer),
            Self::Block {statements} => visitor.visit_block_stmt(statements), 
            Self::If {condition, then_branch, else_branch} => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Self::While {condition, body} => visitor.visit_while_stmt(condition, body),
            Self::Function {name, params, body} => visitor.visit_function_stmt(name, params, body),

        }
    }
}