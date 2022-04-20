use crate::expr::{self, Expr};
use crate::interpreter::Interpreter;
use crate::interpreter::RuntimeError;
use crate::stmt::{self, Stmt};
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Resolver {
    interpreter: Rc<Interpreter>,
    scopes: Rc<RefCell<Vec<HashMap<String, bool>>>>,
}

impl Resolver {
    pub fn new(interpreter: Rc<Interpreter>) -> Self {
        Resolver {
            interpreter,
            scopes: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn resolve(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        statement.accept_stmt(self, statement.clone())
    }

    pub fn resolve_vec(&self, statements: &Vec<Rc<Stmt>>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.resolve(statement.clone())?;
        }
        Ok(())
    }

    fn resolve_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        expr.accept_expr(self, expr.clone())
    }

    fn begin_scope(&self) -> () {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    fn end_scope(&self) -> (){
        self.scopes.borrow_mut().pop().expect("Resolver attempted to end nonexistent scope");
    }

    fn declare(&self, name: &Token) -> Result<(), RuntimeError> {
        if self.scopes.borrow().is_empty() {
            return Ok(());
        }
        let last_scope_index = self.last_scope_index();
        //Aka: If name is in scope 
        if self
            .scopes
            .borrow()
            .get(last_scope_index)
            .unwrap()
            .contains_key(&name.lexeme)
        {
            return Err(RuntimeError::new_token(
                name,
                "Already a variable with this name in this scope".to_string(),
            ));
        }
        self.scopes
            .borrow_mut()
            .get_mut(last_scope_index)
            .unwrap()
            .insert(name.lexeme.clone(), false);
        Ok(())
    }

    fn define(&self, name: String) -> () {
        if self.scopes.borrow().is_empty() {
            return;
        }
        let last_scope_index = self.last_scope_index();
        self.scopes
            .borrow_mut()
            .get_mut(last_scope_index)
            .unwrap()
            .insert(name, true);
    }

    fn scopes_is_empty(&self) -> bool {
        self.scopes.borrow().is_empty()
    }

    fn last_scope_index(&self) -> usize {
        self.scopes.borrow().len() - 1
    }

    fn resolve_local(&self, expr: Rc<Expr>, name: &Token) -> () {
        for i in (0..self.scopes.borrow().len()).rev() {
            if self
                .scopes
                .borrow()
                .get(i)
                .unwrap()
                .contains_key(&name.lexeme)
            {
                self.interpreter.resolve(expr, self.last_scope_index() - i);
                return;
            }
        }
    }

    fn resolve_function(
        &self,
        params: &Vec<Token>,
        body: &Vec<Rc<Stmt>>,
    ) -> Result<(), RuntimeError> {
        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param.lexeme.clone());
        }

        self.resolve_vec(body)?;
        self.end_scope();
        Ok(())
    }
}

impl expr::VisitorExpr<Result<(), RuntimeError>> for Resolver {
    fn visit_binary_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let (left, right) = match expr.as_ref() {
            Expr::Binary {
                left,
                operator: _,
                right,
            } => (left, right),
            _ => panic!(),
        };

        self.resolve_expr(left.clone())?;
        self.resolve_expr(right.clone())?;

        Ok(())
    }

    fn visit_grouping_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let expression = match expr.as_ref() {
            Expr::Grouping { expression } => expression,
            _ => panic!(),
        };

        self.resolve_expr(expression.clone())?;

        Ok(())
    }

    fn visit_literal_expr(&self, _expr: Rc<Expr>) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_unary_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let right = match expr.as_ref() {
            Expr::Unary { operator: _, right } => right,
            _ => panic!(),
        };

        self.resolve_expr(right.clone())?;
        Ok(())
    }

    fn visit_variable_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let name = match expr.as_ref() {
            Expr::Variable { name } => name,
            _ => panic!(), //never happens
        };

        if !self.scopes_is_empty() {
            match self
                .scopes
                .borrow()
                .get(self.last_scope_index())
                .unwrap()
                .get(&name.lexeme)
            {
                Some(is_defined) => {
                    if !is_defined {
                        return Err(RuntimeError::new(
                            "Can't read local variable in its own initializer.".to_string(),
                            name.line,
                        ));
                    }
                }
                None => (),
            }
        }

        self.resolve_local(expr.clone(), name);
        Ok(())
    }

    fn visit_assign_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let (value, name) = match expr.as_ref() {
            Expr::Assign { name, value } => (value, name),
            _ => panic!(), //this should never happen
        };
        self.resolve_expr(value.clone())?;
        self.resolve_local(expr.clone(), name);

        Ok(())
    }

    fn visit_logical_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let (left, right) = match expr.as_ref() {
            Expr::Logical {
                left,
                operator: _,
                right,
            } => (left, right),
            _ => panic!(),
        };
        self.resolve_expr(left.clone())?;
        self.resolve_expr(right.clone())?;

        Ok(())
    }

    fn visit_call_expr(&self, expr: Rc<Expr>) -> Result<(), RuntimeError> {
        let (callee, arguments) = match expr.as_ref() {
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => (callee, arguments),
            _ => panic!(),
        };

        self.resolve_expr(callee.clone())?;
        for argument in arguments {
            self.resolve_expr(argument.clone())?;
        }

        Ok(())
    }
}

impl stmt::VisitorStmt<Result<(), RuntimeError>> for Resolver {
    fn visit_expression_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let expression = match statement.as_ref() {
            Stmt::Expression { expression } => expression,
            _ => panic!(),
        };

        self.resolve_expr(expression.clone())?;
        Ok(())
    }

    fn visit_print_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let expression = match statement.as_ref() {
            Stmt::Print { expression } => expression,
            _ => panic!(),
        };

        self.resolve_expr(expression.clone())?;

        Ok(())
    }

    fn visit_var_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let (name, initializer) = match statement.as_ref() {
            Stmt::Var { name, initializer } => (name, initializer),
            _ => panic!(),
        };
        self.declare(name)?;
        self.resolve_expr(initializer.clone())?;
        self.define(name.lexeme.clone());
        Ok(())
    }

    fn visit_block_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let statements = match statement.as_ref() {
            Stmt::Block { statements } => statements,
            _ => panic!(),
        };
        self.begin_scope();
        self.resolve_vec(statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let (condition, then_branch, else_branch) = match statement.as_ref() {
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => (condition, then_branch, else_branch),
            _ => panic!(),
        };

        self.resolve_expr(condition.clone())?;
        self.resolve(then_branch.clone())?;
        match else_branch {
            Some(else_stmt) => self.resolve(else_stmt.clone())?,
            None => (),
        };

        Ok(())
    }

    fn visit_while_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let (condition, body) = match statement.as_ref() {
            Stmt::While { condition, body } => (condition, body),
            _ => panic!(),
        };

        self.resolve_expr(condition.clone())?;
        self.resolve(body.clone())?;

        Ok(())
    }

    fn visit_function_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let (name, params, body) = match statement.as_ref() {
            Stmt::Function { name, params, body } => (name, params, body),
            _ => panic!(),
        };

        self.declare(name)?;
        self.define(name.lexeme.clone());

        self.resolve_function(params, body)?;

        Ok(())
    }

    fn visit_return_stmt(&self, statement: Rc<Stmt>) -> Result<(), RuntimeError> {
        let expression = match statement.as_ref() {
            Stmt::Return { keyword: _, value } => value,
            _ => panic!(),
        };

        self.resolve_expr(expression.clone())?;
        Ok(())
    }
}
