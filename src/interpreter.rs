

use crate::environment::Environment;
use crate::expr::{self, Expr};
use crate::lox_callable::{LoxCallable};
use crate::lox_function::LoxFunction;
use crate::native_function::NativeFunction;
use crate::stmt::{self, Stmt};
use crate::lox_type::LoxValue::{self, *};
use crate::token::{Token, TokenType::*};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use std::fmt::Write;

pub struct RuntimeError{
    pub message: String,
    pub line: u32,
    pub return_value: Option<LoxValue>,
}   

impl RuntimeError{
    pub fn new(message: String, line: u32) -> Self{
        RuntimeError { message, line, return_value: None }
    }

    pub fn new_token(token: &Token, message: String ) -> Self{
        RuntimeError{
            message : ["at '", token.lexeme.as_str(), "'", message.as_str()].concat(),
            line: token.line,
            return_value: None
        }
    }
    
    pub fn new_with_return(token: &Token, message: String, return_value: LoxValue) -> Self{
        RuntimeError{
            message : ["at '", token.lexeme.as_str(), "'", message.as_str()].concat(),
            line: token.line,
            return_value: Some(return_value)
        }
    }
}
pub struct Interpreter{
    //This environment handling required massive amounts of indrection. 
    //It is required becuase Rust's borrow checker is strict in that you can only have one mutable reference to a value
    pub globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    locals: RefCell<HashMap<usize, usize>>,
    output: RefCell<String>,
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

        // fn print_env(_arguments: Vec<LoxValue>, interpreter: &Interpreter) -> LoxValue{
        //     println!("{:?}", interpreter.environment);
        //     LoxValue::Nil
        // }

        globals.borrow_mut().define("clock".to_string(), LoxValue::Native(
            Rc::new(NativeFunction{
                arity : 0,
                function: clock
            })
        ));

        // globals.borrow_mut().define("print_env".to_string(), LoxValue::Native(
        //     Rc::new(NativeFunction{
        //         arity : 0,
        //         function: print_env 
        //     })
        // ));
        
        Interpreter {  
            globals,
            environment,
            locals: RefCell::new(HashMap::new()),
            output: RefCell::new("".to_string())
        }
    }
    pub fn interpret(&self, statements : Vec<Rc<Stmt>>, output_stream: &mut dyn io::Write) -> Result<(), RuntimeError>{    
        for statement in statements{
            match self.execute(statement){
                Ok(()) => (),
                Err(error) => return Err(error),
            }
            output_stream.write(self.output.borrow().as_bytes()).expect("Could not write to provided output buffer.");
            self.output.borrow_mut().clear();
        }
        Ok(())
    }

    fn execute(&self, stmt : Rc<Stmt>) -> Result<(), RuntimeError>{
        stmt.accept(self)
    }

    
    pub fn resolve(&self, expr: Rc<Expr>, depth: usize) -> (){
        //Stores the memory location of the expression as a raw usize value. 
        //I needed a way to get a unique identifier for each expression. 
        //I could have just added an ID in the parser but that would require storing a significant amount of data
        //It turns out that we already have a unique id: The memory address of the expression
        //Pattern inspired by: https://github.com/UncleScientist/lox-ast/blob/4f56ce6979a3e5eb21b26aaa9b0dbef4860b1474/generate_ast/mod.rs#L106
        //TODO: Determine if this works and such
        let pointer_val = Rc::as_ptr(&expr) as usize;
        self.locals.borrow_mut().insert(pointer_val, depth);
    }

    pub fn execute_block(&self, statements: &Vec<Rc<Stmt>>, environment : Environment) -> Result<(), RuntimeError>{
        //When a call goes to execute block it should save the current env
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        
        for statement in statements{
            //This match statement ensure that the environments will be swapped back even if there is a RuntimeError
            //This is essential if there is a return statement in 'statements'
            match self.execute(statement.clone()){
                Ok(()) => (),
                Err(error) => {
                    self.environment.replace(previous);
                    return Err(error)
                }
            }
        }

        self.environment.replace(previous);
        Ok(())
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<LoxValue, RuntimeError> {
        expr.accept(self)
    }

    fn call_function(&self, func: &dyn LoxCallable, arguments: Vec<LoxValue>, paren: &Token) -> Result<LoxValue, RuntimeError>{
        if arguments.len() != func.arity() as usize{
            Err(RuntimeError::new_token(paren, ["Expected ".to_string() , func.arity().to_string(), " arguments but got ".to_string(), arguments.len().to_string(), ".".to_string()].concat()))
        } else{
            func.call(self, arguments)
        }
    }

    fn look_up_variable(&self, name: &Token, expr_pointer_id: usize) -> Result<LoxValue, RuntimeError>{
        match self.locals.borrow().get(&expr_pointer_id){
            Some(dist) => self.environment.borrow().borrow().get_at(dist.clone(), name),
            None => self.globals.borrow().get(name)
        }

    }
}



fn invalid_operand_number(operator: &Token) -> RuntimeError{
    RuntimeError::new_token(operator, "Operand must be a number.".to_string())
}


impl expr::Visitor<Result<LoxValue, RuntimeError>> for Interpreter{

    fn visit_binary_expr(&self, left: Rc<Expr>, operator : &Token, right : Rc<Expr>) -> Result<LoxValue, RuntimeError>{
        let left_eval = self.evaluate(left)?;
        let right_eval = self.evaluate(right)?;

        match operator.kind{
            PLUS => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Ok(Number(left_val + right_val)),
                    _ => Err(RuntimeError::new_token(operator, "Operand types do not match".to_string()))
                },
                LoxString(left_val) => match right_eval{
                    LoxString(right_val) => Ok(LoxString([left_val.as_str(), right_val.as_str()].concat().to_string())),
                    _ => Err(RuntimeError::new_token(operator, "Operand types do not match".to_string()))
                },
                _ => Err(RuntimeError::new_token(operator, "Invalid operands. Operands must be numbers or Strings".to_string()))
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
            _ => Err(RuntimeError::new_token(operator, "Missed Parser Error".to_string())) //Unreachable if parser operated properly 
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
            _ => Err(RuntimeError::new_token(paren, "Can only call functions and classes.".to_string()))
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
            _ => Err(RuntimeError::new_token(operator, "Missed Parser Error".to_string())) //Unreachable if parser operated properly 
        }
    }


    fn visit_variable_expr(&self, name: &Token, expr_pointer_id: usize) -> Result<LoxValue, RuntimeError> {
        self.look_up_variable(name, expr_pointer_id)
    }

    fn visit_assign_expr(&self, name: &Token, value: Rc<Expr>, expr_pointer_id: usize) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(value)?;
        match self.locals.borrow().get(&expr_pointer_id){
            Some(dist) => self.environment.borrow().borrow_mut().assign_at(*dist, name, &value)?,
            None => self.globals.borrow_mut().assign(name, &value)?
        }
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
                writeln!(self.output.borrow_mut(), "{}", val).expect("Interpreter Buffer Write Error");
                Ok(())
            },
            Err(error) => Err(error),
        }
    }

    fn visit_var_stmt(&self, name: Token, initializer: Rc<Expr>) -> Result<(), RuntimeError>{
        //initializer can always be evaluated becuase if it is empty it is a literal nil expression
        let value = self.evaluate(initializer)?;
        self.environment.borrow().borrow_mut().define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_while_stmt(&self, condition: Rc<Expr>, body: Rc<Stmt>) -> Result<(), RuntimeError>{
        while self.evaluate(condition.clone())?.is_truthy(){
            self.execute(body.clone())?;
        }
        Ok(())
    }



    fn visit_block_stmt(&self, statements: &Vec<Rc<Stmt>>) -> Result<(), RuntimeError>{
        let env = Environment::new_enclosed(self.environment.borrow().clone());
        self.execute_block(statements, env)?;
        Ok(())
    }

    fn visit_if_stmt(&self, condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: &Option<Rc<Stmt>>) -> Result<(), RuntimeError>{
        if self.evaluate(condition)?.is_truthy(){
            Ok(self.execute(then_branch)?)
        } else {
            match else_branch{
                Some(else_branch) => Ok(self.execute(else_branch.clone())?),
                None => Ok(()),
            }
        }
    }

    fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<Rc<Stmt>>) -> Result<(), RuntimeError> {
        //Deep-ish copy the current env (deep copy values but not the enclosing env)
        let closure = self.environment.borrow().borrow().clone();
        let func = LoxFunction{
            arity: params.len() as u32,
            declaration: Rc::new(Stmt::Function { name: name.clone(), params, body }),
            closure : Rc::new(RefCell::new(closure))
        };

        self.environment.borrow().borrow_mut().define(name.lexeme, Function(Rc::new(func)));
        
        Ok(())
    }

    //Return uses error propigation to return its value packaged in a RuntimeError
    fn visit_return_stmt(&self, keyword: Token, value: Rc<Expr>) -> Result<(), RuntimeError> {
        let value_eval = self.evaluate(value)?;
        Err(RuntimeError::new_with_return(&keyword, "Return called outside of function".to_string(), value_eval))
    }

}