use crate::token::Token;
use crate::lox_type::LoxValue;
use std::rc::Rc;
//Exprs need to be boxed to avoid reccursive enums which Rust does not allow
pub enum Expr {
    Binary{left: Rc<Expr>, operator: Token, right: Rc<Expr>}, 
    Grouping{expression: Rc<Expr>},
    Literal{value: LoxValue},
    Unary{operator: Token, right: Rc<Expr>},
    Variable{name: Token},
    Assign{name: Token, value: Rc<Expr>},
    Logical{left: Rc<Expr>, operator: Token, right: Rc<Expr>},
    Call{callee: Rc<Expr>, paren: Token, arguments: Vec<Rc<Expr>>}
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> T;
    fn visit_grouping_expr(&self, expression : Rc<Expr>) -> T;
    fn visit_literal_expr(&self, expr : &LoxValue) -> T;
    fn visit_unary_expr(&self, operator : &Token, right : Rc<Expr>) -> T;
    fn visit_variable_expr(&self, name : &Token) -> T;
    fn visit_assign_expr(&self, name: &Token, value: Rc<Expr>) -> T;
    fn visit_logical_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> T;
    fn visit_call_expr(&self, callee: Rc<Expr>, paren : &Token, arguments : &Vec<Rc<Expr>>) -> T;

}

pub trait VisitorExpr<T> {
    fn visit_binary_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_grouping_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_literal_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_unary_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_variable_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_assign_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_logical_expr(&self, expr: Rc<Expr>) -> T;
    fn visit_call_expr(&self, expr: Rc<Expr>) -> T;

}



impl Expr{
    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T{
        match self{
            Self::Binary { left, operator, right} => visitor.visit_binary_expr(left.clone(), operator, right.clone()),
            Self::Grouping { expression } => visitor.visit_grouping_expr(expression.clone()),
            Self::Literal { value } => visitor.visit_literal_expr(value),
            Self::Unary { operator, right} => visitor.visit_unary_expr(operator, right.clone()),
            Self::Variable {name} => visitor.visit_variable_expr(name),
            Self::Assign {name, value} => visitor.visit_assign_expr(name, value.clone()),
            Self::Logical { left, operator, right} => visitor.visit_logical_expr(left.clone(), operator, right.clone()),
            Self::Call { callee, paren, arguments} => visitor.visit_call_expr(callee.clone(), paren, arguments),
        }
    }

    pub fn accept_expr<T>(&self, visitor: &impl VisitorExpr<T>, expr: Rc<Expr>) -> T{
        match self{
            Self::Binary { left, operator, right} => visitor.visit_binary_expr(expr),
            Self::Grouping { expression } => visitor.visit_grouping_expr(expr),
            Self::Literal { value } => visitor.visit_literal_expr(expr),
            Self::Unary { operator, right} => visitor.visit_unary_expr(expr),
            Self::Variable {name} => visitor.visit_variable_expr(expr),
            Self::Assign {name, value} => visitor.visit_assign_expr(expr),
            Self::Logical { left, operator, right} => visitor.visit_logical_expr(expr),
            Self::Call { callee, paren, arguments} => visitor.visit_call_expr(expr),
        }
    }

    
}