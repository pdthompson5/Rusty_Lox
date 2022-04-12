

use crate::environment::Environment;
use crate::expr::{self, Expr};
use crate::stmt::{self, Stmt};
use crate::lox_type::LoxValue::{self, *};
use crate::token::{Token, TokenType::*};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RuntimeError{
    pub message: String,
    pub line: u32
}   
pub struct Interpreter{
    environment: RefCell<Rc<RefCell<Environment>>>
}

impl  Interpreter{
    pub fn new() -> Self {
        Interpreter {  
            //Pattern for handling environment references from: https://github.com/UncleScientist/lox-ast
            environment : RefCell::new(Rc::new(RefCell::new(Environment::new())))
        }
    }
    pub fn interpret(&mut self, statements : Vec<Box<Stmt>>) -> Result<(), RuntimeError>{    
        for statement in statements{
            match self.execute(&statement){
                Ok(()) => (),
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    fn execute(&self, stmt : &Box<Stmt>) -> Result<(), RuntimeError>{
        stmt.accept(self)
    }

    fn execute_block(&self, statements: &Vec<Box<Stmt>>, environment : Environment) -> Result<(), RuntimeError>{
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        
        for statement in statements{
            self.execute(statement)?;
        }

        self.environment.replace(previous);
        Ok(())
    }

    fn evaluate(&self, expr: &Box<Expr>) -> Result<LoxValue, RuntimeError> {
        expr.accept(self)
    }
}

fn error(token: &Token, message: String) -> RuntimeError{
    RuntimeError{
        message : ["at '", token.lexeme.as_str(), "'", message.as_str()].concat(),
        line: token.line
    }
}

fn invalid_operand_number(operator: &Token) -> RuntimeError{
    error(operator, "Operand must be a number.".to_string())
}


impl expr::Visitor<Result<LoxValue, RuntimeError>> for Interpreter{

    fn visit_binary_expr(&self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> Result<LoxValue, RuntimeError>{
        let left_eval = self.evaluate(left)?;
        let right_eval = self.evaluate(right)?;

        match operator.kind{
            PLUS => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Number(left_val + right_val)),
                    _ => Err(error(operator, "Operand types do not match".to_string()))
                },
                LoxString(left_val) => match right_eval{
                    LoxString(right_val) => Ok(LoxString([left_val.as_str(), right_val.as_str()].concat().to_string())),
                    _ => Err(error(operator, "Operand types do not match".to_string()))
                },
                _ => Err(error(operator, "Invalid operands. Operands must be numbers or Strings".to_string()))
            },

            MINUS => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Number(left_val - right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },

            STAR => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Number(left_val * right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },

            SLASH => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Number(left_val / right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },
            //Todo: Determine floating point comparision in rust -> I think it is good 
            GREATER => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Boolean(left_val > right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },

            GREATER_EQUAL => match left_eval{
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Boolean(left_val > right_val || left_val == right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },
            LESS => match left_eval{
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Boolean(left_val < right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },
            LESS_EQUAL =>match left_eval{
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Boolean(left_val < right_val || left_val == right_val)),
                    _ => Err(invalid_operand_number(operator))
                },
                _ => Err(invalid_operand_number(operator))
            },

            //LoxValue implements PartialEq so simple equality comparisons work 
            EQUAL_EQUAL => Ok(Boolean(left_eval == right_eval)),
            BANG_EQUAL => Ok(Boolean(left_eval != right_eval)),
            _ => Err(error(operator, "Missed Parser Error".to_string())) //Unreachable if parser operated properly 
        }
    }

    fn visit_grouping_expr(&self, expression : &Box<Expr>) -> Result<LoxValue, RuntimeError>{
        self.evaluate(expression)
    }

    fn visit_literal_expr(&self, value : &LoxValue) -> Result<LoxValue, RuntimeError>{
        Ok(value.clone())
    }

    fn visit_unary_expr(&self, operator : &Token, expression : &Box<Expr>) -> Result<LoxValue, RuntimeError>{
        let right = self.evaluate(expression)?;

        match operator.kind {
            MINUS => match right{
                Number(val) => Ok(Number(-val)),
                _ => Err(invalid_operand_number(operator))
            }
            BANG => Ok(Boolean(right.is_truthy())),
            _ => Err(error(operator, "Missed Parser Error".to_string())) //Unreachable if parser operated properly 
        }
    }
    //in Lox: pass by value: all of the values that I have so far. I think functions should be passed by reference
    //Can I do that by just having a Lox value for functions that contains the reference rather than making it a LoxValue reference?
    fn visit_variable_expr(&self, name: &Token) -> Result<LoxValue, RuntimeError> {
        self.environment.borrow().borrow().get(name)
    }

    fn visit_assign_expr(&self, name: &Token, value: &Box<Expr>) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(value)?;
        self.environment.borrow().borrow_mut().assign(name, &value)?;
        Ok(value)
    }

}


impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter{
    fn visit_expression_stmt(&self, expression: &Box<Expr>) -> Result<(), RuntimeError>{
        match self.evaluate(expression){
            Ok(_val) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn visit_print_stmt(&self, expression: &Box<Expr>) -> Result<(), RuntimeError>{
        match self.evaluate(expression){
            Ok(val) => {
                println!("{}", val);
                Ok(())
            },
            Err(error) => Err(error),
        }
    }

    fn visit_var_stmt(&self, name: &Token, initializer: &Box<Expr>) -> Result<(), RuntimeError>{
        //initializer can always be evaluated becuase if it is empty it is a literal nil expression
        let value = self.evaluate(initializer)?;
        self.environment.borrow().borrow_mut().define(name.lexeme.clone(), value);
        Ok(())
    }



    fn visit_block_stmt(&self, statements: &Vec<Box<Stmt>>) -> Result<(), RuntimeError>{
        let env = Environment::new_enclosed(self.environment.borrow().clone());
        self.execute_block(statements, env)?;
        Ok(())
    }

    fn visit_if_stmt(&self, condition: &Box<Expr>, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> Result<(), RuntimeError>{
        if self.evaluate(condition)?.is_truthy(){
            Ok(self.execute(then_branch)?)
        } else {
            match else_branch{
                Some(else_branch) => Ok(self.execute(else_branch)?),
                None => Ok(()),
            }
        }
    }
}