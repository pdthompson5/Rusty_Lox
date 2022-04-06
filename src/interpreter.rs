use crate::expr::Expr;
use crate::expr::Visitor;
use crate::lox_type::LoxValue::{self, *};
use crate::token::{Token, TokenType::*};


struct RuntimeError{
    message: String,
    line: u32
}   
pub struct Interpreter {

}

impl Interpreter{
    pub fn interpret(&mut self, expr : &Box<Expr>){
        let result = self.evaluate(expr);
        match result{
            Ok(val) => println!("{}", val),
            Err(error) => crate::error(error.line, &error.message),
        }
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> Result<LoxValue, RuntimeError> {
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


impl Visitor<Result<LoxValue, RuntimeError>> for Interpreter{

    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> Result<LoxValue, RuntimeError>{
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

    fn visit_grouping_expr(&mut self, expression : &Box<Expr>) -> Result<LoxValue, RuntimeError>{
        self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value : &LoxValue) -> Result<LoxValue, RuntimeError>{
        Ok(value.clone())
    }

    fn visit_unary_expr(&mut self, operator : &Token, expression : &Box<Expr>) -> Result<LoxValue, RuntimeError>{
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
}