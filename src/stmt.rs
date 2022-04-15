use crate::{expr::Expr, token::Token};
use std::rc::Rc;
pub enum Stmt {
    Expression{expression: Rc<Expr>}, 
    Print{expression: Rc<Expr>},
    Var{name : Token, initializer: Rc<Expr>},
    Block{statements: Vec<Rc<Stmt>>},
    If{condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: Option<Rc<Stmt>>},
    While{condition: Rc<Expr>, body: Rc<Stmt>},
    Function{name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>},
    Return{keyword: Token, value: Rc<Expr>}
    
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&self, expression: Rc<Expr>) -> T;
    fn visit_print_stmt(&self, expression: Rc<Expr>) -> T;
    fn visit_var_stmt(&self, name: Token, initializer: Rc<Expr>) -> T;
    fn visit_block_stmt(&self, statements: &Vec<Rc<Stmt>>) -> T;
    fn visit_if_stmt(&self, condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: &Option<Rc<Stmt>>) -> T;
    fn visit_while_stmt(&self, condition: Rc<Expr>, body: Rc<Stmt>) -> T;
    fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>) -> T;
    fn visit_return_stmt(&self, keyword: Token, value: Rc<Expr>) -> T;
}

pub trait VisitorStmt<T> {
    fn visit_expression_stmt(&self, statement: Rc<Stmt>) -> T;
    fn visit_print_stmt(&self, statement: Rc<Stmt>) -> T;
    fn visit_var_stmt(&self, statement: Rc<Stmt>) -> T;
    fn visit_block_stmt(&self, statement: Rc<Stmt>) -> T;
    fn visit_if_stmt(&self, statement: Rc<Stmt>) -> T;
    fn visit_while_stmt(&self, statement: Rc<Stmt>) -> T;
    fn visit_function_stmt(&self,statement: Rc<Stmt>) -> T;
    fn visit_return_stmt(&self, statement: Rc<Stmt>) -> T;
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
            Self::Return {keyword, value} => visitor.visit_return_stmt(keyword.clone(), value.clone())
        }
    }


    pub fn accept_stmt<T>(&self, visitor: &impl VisitorStmt<T>, stmt: Rc<Stmt>) -> T{
        match self{ 
            Self::Expression {expression:_ } => visitor.visit_expression_stmt(stmt),
            Self::Print {expression:_ } =>visitor.visit_print_stmt(stmt),
            Self::Var {name:_, initializer:_ } => visitor.visit_var_stmt(stmt),
            Self::Block {statements:_} => visitor.visit_block_stmt(stmt), 
            Self::If {condition:_, then_branch:_, else_branch:_} => visitor.visit_if_stmt(stmt),
            Self::While {condition:_, body:_} => visitor.visit_while_stmt(stmt),
            Self::Function {name:_, params:_, body:_} => visitor.visit_function_stmt(stmt),
            Self::Return {keyword:_, value:_} => visitor.visit_return_stmt(stmt)
        }
    }
}