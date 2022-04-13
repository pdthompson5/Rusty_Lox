use crate::{expr::Expr, token::Token};
use std::rc::Rc;
pub enum Stmt {
    Expression{expression: Rc<Expr>}, 
    Print{expression: Rc<Expr>},
    Var{name : Token, initializer: Rc<Expr>},
    Block{statements: Vec<Rc<Stmt>>},
    If{condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: Option<Rc<Stmt>>},
    While{condition: Rc<Expr>, body: Rc<Stmt>},
    Function{name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>}
    
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&self, expression: Rc<Expr>) -> T;
    fn visit_print_stmt(&self, expression: Rc<Expr>) -> T;
    fn visit_var_stmt(&self, name: Token, initializer: Rc<Expr>) -> T;
    fn visit_block_stmt(&self, statements: &Vec<Rc<Stmt>>) -> T;
    fn visit_if_stmt(&self, condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: &Option<Rc<Stmt>>) -> T;
    fn visit_while_stmt(&self, condition: Rc<Expr>, body: Rc<Stmt>) -> T;
    fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>) -> T;
}

impl Stmt{
    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T{
        match self{ 
            Self::Expression {expression } => visitor.visit_expression_stmt(expression.clone()),
            Self::Print {expression } => visitor.visit_print_stmt(expression.clone()),
            Self::Var {name, initializer } => visitor.visit_var_stmt(name.clone(), initializer.clone()),
            Self::Block {statements} => visitor.visit_block_stmt(statements), 
            Self::If {condition, then_branch, else_branch} => visitor.visit_if_stmt(condition.clone(), then_branch.clone(), else_branch),
            Self::While {condition, body} => visitor.visit_while_stmt(condition.clone(), body.clone()),
            Self::Function {name, params, body} => visitor.visit_function_stmt(name.clone(), params.clone(), body.clone()),
        }
    }
}