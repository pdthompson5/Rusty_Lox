
use crate::expr::{Expr, self};
use crate::interpreter::RuntimeError;
use crate::stmt::{Stmt, self};
use crate::interpreter::Interpreter;
use crate::token::Token;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
// pub struct Resolver{
//     interpreter: Interpreter,
//     scopes: Rc<RefCell<Vec<HashMap<String, bool>>>>
// }

// impl Resolver{
//     fn new(interpreter: Interpreter) -> Self{
//         Resolver { 
//             interpreter,
//             scopes : Rc::new(RefCell::new(Vec::new())) 
//         }
//     }

//     fn resolve(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError>{
//         statement.accept(self)
//     }

//     fn resolve_vec(&self, statements: &Vec<Rc<Stmt>>) -> Result<(), RuntimeError>{
//         for statement in statements{
//             self.resolve(statement.clone())?;
//         }
//         Ok(())
//     }

//     fn resolve_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError>{
//         expr.accept(self)
//     }

//     fn begin_scope(&self) -> (){
//         self.scopes.borrow_mut().push(HashMap::new())
//     }

//     fn end_scope(&self) -> (){
//         self.scopes.borrow_mut().pop().expect("Resolver attempted to end nonexistent scope");
//     }

//     fn declare(&self, name : String) -> (){
//         if self.scopes.borrow().is_empty(){
//             return ;
//         }
//         let length = self.scopes.borrow().len();
//         self.scopes.borrow_mut().get_mut(self.last_scope_index()).unwrap().insert(name, false);
//     }

//     fn define(&self, name : String) -> (){
//         if self.scopes.borrow().is_empty(){
//             return ;
//         }
//         self.scopes.borrow_mut().get_mut(self.last_scope_index()).unwrap().insert(name, true);
//     }

//     fn scopes_is_empty(&self) -> bool{
//         self.scopes.borrow().is_empty()
//     }
    
//     fn last_scope_index(&self) -> usize{
//         self.scopes.borrow().len() - 1
//     }





// }


// impl expr::Visitor<Result<(), RuntimeError>> for Resolver{
//     fn visit_binary_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_grouping_expr(&self, expression : Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_literal_expr(&self, expr : &crate::lox_type::LoxValue) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_unary_expr(&self, operator : &Token, right : Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_variable_expr(&self, name : &Token) -> Result<(), RuntimeError> {
//         if !self.scopes_is_empty(){
//             match self.scopes.borrow().get(self.last_scope_index()).unwrap().get(&name.lexeme){
//                 Some(is_defined) => if !is_defined{
//                     return Err(RuntimeError::new("Can't read local variable in its own initializer.".to_string(), name.line))
//                 },
//                 None => ()
//             }
//         } 
            
//         self.resolve_local(name)?;
//         Ok(())
//     }

//     fn visit_assign_expr(&self, name: &Token, value: Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_logical_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_call_expr(&self, callee: Rc<Expr>, paren : &Token, arguments : &Vec<Rc<Expr>>) -> Result<(), RuntimeError> {
//         todo!()
//     }
// }


// impl stmt::Visitor<Result<(), RuntimeError>> for Resolver{
//     fn visit_expression_stmt(&self, expression: Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_print_stmt(&self, expression: Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_var_stmt(&self, name: Token, initializer: Rc<Expr>) -> Result<(), RuntimeError> {
//         self.declare(name.lexeme.clone());
//         self.resolve_expr(initializer)?;
//         self.define(name.lexeme);
//         Ok(())
//     }

//     fn visit_block_stmt(&self, statements: &Vec<Rc<Stmt>>) -> Result<(), RuntimeError> {
//         self.begin_scope();
//         self.resolve_vec(statements)?;
//         self.end_scope();
//         Ok(())
//     }

//     fn visit_if_stmt(&self, condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: &Option<Rc<Stmt>>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_while_stmt(&self, condition: Rc<Expr>, body: Rc<Stmt>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>) -> Result<(), RuntimeError> {
//         todo!()
//     }

//     fn visit_return_stmt(&self, keyword: Token, value: Rc<Expr>) -> Result<(), RuntimeError> {
//         todo!()
//     }
// }












pub struct Resolver{
    interpreter: Interpreter,
    scopes: Rc<RefCell<Vec<HashMap<String, bool>>>>
}

impl Resolver{
    fn new(interpreter: Interpreter) -> Self{
        Resolver { 
            interpreter,
            scopes : Rc::new(RefCell::new(Vec::new())) 
        }
    }

    fn resolve(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError>{
        statement.accept(self)
    }

    fn resolve_vec(&self, statements: &Vec<Rc<Stmt>>) -> Result<(), RuntimeError>{
        for statement in statements{
            self.resolve(statement.clone())?;
        }
        Ok(())
    }

    fn resolve_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError>{
        expr.accept_expr(self, expr)
    }

    fn begin_scope(&self) -> (){
        self.scopes.borrow_mut().push(HashMap::new())
    }

    fn end_scope(&self) -> (){
        self.scopes.borrow_mut().pop().expect("Resolver attempted to end nonexistent scope");
    }

    fn declare(&self, name : String) -> (){
        if self.scopes.borrow().is_empty(){
            return ;
        }
        let length = self.scopes.borrow().len();
        self.scopes.borrow_mut().get_mut(self.last_scope_index()).unwrap().insert(name, false);
    }

    fn define(&self, name : String) -> (){
        if self.scopes.borrow().is_empty(){
            return ;
        }
        self.scopes.borrow_mut().get_mut(self.last_scope_index()).unwrap().insert(name, true);
    }

    fn scopes_is_empty(&self) -> bool{
        self.scopes.borrow().is_empty()
    }
    
    fn last_scope_index(&self) -> usize{
        self.scopes.borrow().len() - 1
    }





}


impl expr::VisitorExpr<Result<(), RuntimeError>> for Resolver{
    fn visit_binary_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_grouping_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_literal_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_unary_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_variable_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let name = match expr.borrow(){
            Expr::Variable { name } => name,
            _ => panic!() //never happens
        };

        if !self.scopes_is_empty(){
            match self.scopes.borrow().get(self.last_scope_index()).unwrap().get(name.lexeme){
                Some(is_defined) => if !is_defined{
                    return Err(RuntimeError::new("Can't read local variable in its own initializer.".to_string(), name.line))
                },
                None => ()
            }
        } 
            
        self.resolve_local(name)?;
        Ok(())
    }

    fn visit_assign_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_logical_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_call_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        todo!()
    }
    // fn visit_binary_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    // fn visit_grouping_expr(&self, expression : Rc<Expr>) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    // fn visit_literal_expr(&self, expr : &crate::lox_type::LoxValue) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    // fn visit_unary_expr(&self, operator : &Token, right : Rc<Expr>) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    // fn visit_variable_expr(&self, name : &Token) -> Result<(), RuntimeError> {
    //     if !self.scopes_is_empty(){
    //         match self.scopes.borrow().get(self.last_scope_index()).unwrap().get(&name.lexeme){
    //             Some(is_defined) => if !is_defined{
    //                 return Err(RuntimeError::new("Can't read local variable in its own initializer.".to_string(), name.line))
    //             },
    //             None => ()
    //         }
    //     } 
            
    //     self.resolve_local(name)?;
    //     Ok(())
    // }

    // fn visit_assign_expr(&self, name: &Token, value: Rc<Expr>) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    // fn visit_logical_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<(), RuntimeError> {
    //     todo!()
    // }

    // fn visit_call_expr(&self, callee: Rc<Expr>, paren : &Token, arguments : &Vec<Rc<Expr>>) -> Result<(), RuntimeError> {
    //     todo!()
    // }
}


