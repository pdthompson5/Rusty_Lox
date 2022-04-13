

use crate::environment::Environment;
use crate::expr::{self, Expr};
use crate::lox_callable::{LoxCallable};
use crate::native_function::NativeFunction;
use crate::stmt::{self, Stmt};
use crate::lox_type::LoxValue::{self, *};
use crate::token::{Token, TokenType::*};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RuntimeError{
    pub message: String,
    pub line: u32
}   
pub struct Interpreter{
    //TODO: Refactor to use globals 
    //This environment handling required massive abouts of indrection. 
    //It is required becuase Rust's borrow checker is strict in that you can only have one mutable reference to a value
    globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>
}

impl  Interpreter{
    pub fn new() -> Self {
        //Pattern for handling environment references from: https://github.com/UncleScientist/lox-ast

        let globals = Rc::new(RefCell::new(Environment::new()));
        //Clone used on an Rc creates just another reference to the same data 
        let environment = RefCell::new(globals.clone());

        fn clock(_arguments: Vec<LoxValue>, _interpreter: &Interpreter) -> LoxValue{
            //Code for clock from https://stackoverflow.com/questions/26593387/how-can-i-get-the-current-time-in-milliseconds
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            LoxValue::Number(since_the_epoch.as_millis() as f64)
        }

        globals.borrow_mut().define("clock".to_string(), LoxValue::Native(
            Rc::new(NativeFunction{
                arity : 0,
                function: clock
            })
        ));
        
        Interpreter {  
            globals,
            environment
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

    fn evaluate(&self, expr: Rc<Expr>) -> Result<LoxValue, RuntimeError> {
        expr.accept(self)
    }

    fn call_function(&self, func: &dyn LoxCallable, arguments: Vec<LoxValue>, paren: &Token) -> Result<LoxValue, RuntimeError>{
        if arguments.len() != func.arity() as usize{
            Err(error(paren, ["Expected ".to_string() , func.arity().to_string(), " arguments but got ".to_string(), arguments.len().to_string(), ".".to_string()].concat()))
        } else{
            Ok(func.call(self, arguments))
        }
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

    fn visit_binary_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<LoxValue, RuntimeError>{
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


    fn visit_call_expr(&self, callee: Rc<Expr>, paren : &Token, arguments : &Vec<Rc<Expr>>) -> Result<LoxValue, RuntimeError>{
        let callee_val = self.evaluate(callee)?;

        let mut argument_vals = vec![];
        for argument in arguments{
            argument_vals.push(self.evaluate(argument.clone())?);
        }

        match callee_val{
            Function(func) => self.call_function(func.as_ref(), argument_vals, paren),
            Native(func) => self.call_function(func.as_ref(), argument_vals, paren),
            _ => Err(error(paren, "Can only call functions and classes.".to_string()))
        }
    }

    fn visit_grouping_expr(&self, expression : Rc<Expr>) -> Result<LoxValue, RuntimeError>{
        self.evaluate(expression)
    }

    fn visit_literal_expr(&self, value : &LoxValue) -> Result<LoxValue, RuntimeError>{
        Ok(value.clone())
    }

    fn visit_logical_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<LoxValue, RuntimeError>{
        let left = self.evaluate(left)?;

        if let OR = operator.kind{
            if left.is_truthy() {
                return Ok(left)
            }
        } else if let AND = operator.kind {
            if !left.is_truthy(){
                return Ok(left)
            }
        } 
        
        self.evaluate(right)
    }

    fn visit_unary_expr(&self, operator : &Token, expression : Rc<Expr>) -> Result<LoxValue, RuntimeError>{
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

    fn visit_assign_expr(&self, name: &Token, value: Rc<Expr>) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(value)?;
        self.environment.borrow().borrow_mut().assign(name, &value)?;
        Ok(value)
    }

}


impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter{
    fn visit_expression_stmt(&self, expression: Rc<Expr>) -> Result<(), RuntimeError>{
        match self.evaluate(expression){
            Ok(_val) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn visit_print_stmt(&self, expression: Rc<Expr>) -> Result<(), RuntimeError>{
        match self.evaluate(expression){
            Ok(val) => {
                println!("{}", val);
                Ok(())
            },
            Err(error) => Err(error),
        }
    }

    fn visit_var_stmt(&self, name: &Token, initializer: Rc<Expr>) -> Result<(), RuntimeError>{
        //initializer can always be evaluated becuase if it is empty it is a literal nil expression
        let value = self.evaluate(initializer)?;
        self.environment.borrow().borrow_mut().define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_while_stmt(&self, condition: Rc<Expr>, body: &Box<Stmt>) -> Result<(), RuntimeError>{
        while self.evaluate(condition.clone())?.is_truthy(){
            self.execute(body)?;
        }
        Ok(())
    }



    fn visit_block_stmt(&self, statements: &Vec<Box<Stmt>>) -> Result<(), RuntimeError>{
        let env = Environment::new_enclosed(self.environment.borrow().clone());
        self.execute_block(statements, env)?;
        Ok(())
    }

    fn visit_if_stmt(&self, condition: Rc<Expr>, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> Result<(), RuntimeError>{
        if self.evaluate(condition)?.is_truthy(){
            Ok(self.execute(then_branch)?)
        } else {
            match else_branch{
                Some(else_branch) => Ok(self.execute(else_branch)?),
                None => Ok(()),
            }
        }
    }

    fn visit_function_stmt(&self, name: &Token, params: &Vec<Token>, body: &Vec<Box<Stmt>>) -> Result<(), RuntimeError> {
        todo!()
    }
}