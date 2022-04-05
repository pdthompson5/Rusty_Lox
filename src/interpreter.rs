use crate::expr::Expr;
use crate::expr::Visitor;
use crate::lox_type::LoxValue::{self, *};
use crate::token::{Token, TokenType::*};


struct Interpreter {

}

impl Interpreter{
    fn evaluate(&mut self, expr: &Box<Expr>) -> LoxValue{
        expr.accept(self)
    }
}




impl Visitor<LoxValue> for Interpreter{

    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> LoxValue{
        let left_eval = self.evaluate(left);
        let right_eval = self.evaluate(right);

        match operator.kind{
            PLUS => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Number(left_val + right_val),
                    _ => Error
                },
                String(left_val) => match right_eval{
                    String(right_val) => String([left_val.as_str(), right_val.as_str()].concat().to_string()),
                    _ => Error
                },
                _ => Error
            },

            MINUS => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Number(left_val - right_val),
                    _ => Error
                },
                _ => Error
            },

            STAR => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Number(left_val * right_val),
                    _ => Error
                },
                _ => Error
            },

            SLASH => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Number(left_val / right_val),
                    _ => Error
                },
                _ => Error
            },
            //Todo: Determine floating point comparision in rust -> I think it is good 
            GREATER => match left_eval {
                Number(left_val) => match right_eval{
                    Number(right_val) => Boolean(left_val > right_val),
                    _ => Error
                },
                _ => Error
            },

            GREATER_EQUAL => match left_eval{
                Number(left_val) => match right_eval{
                    Number(right_val) => Boolean(left_val > right_val || left_val == right_val),
                    _ => Error
                },
                _ => Error
            },
            LESS => match left_eval{
                Number(left_val) => match right_eval{
                    Number(right_val) => Boolean(left_val < right_val),
                    _ => Error
                },
                _ => Error
            },
            LESS_EQUAL =>match left_eval{
                Number(left_val) => match right_eval{
                    Number(right_val) => Boolean(left_val < right_val || left_val == right_val),
                    _ => Error
                },
                _ => Error
            },

            //LoxValue implements PartialEq so simple equality comparisons work 
            EQUAL_EQUAL => Boolean(left_eval == right_eval),
            BANG_EQUAL => Boolean(left_eval != right_eval),
            _ => Error
        }
    }

    fn visit_grouping_expr(&mut self, expression : &Box<Expr>) -> LoxValue{
        self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value : &LoxValue) -> LoxValue{
        value.clone()
    }

    fn visit_unary_expr(&mut self, operator : &Token, expression : &Box<Expr>) -> LoxValue{
        let right = self.evaluate(expression);

        match operator.kind {
            MINUS => match right{
                Number(val) => Number(-val),
                _ => Error
            }
            BANG => Boolean(right.is_truthy()),
            _ => Error
        }
    }
}