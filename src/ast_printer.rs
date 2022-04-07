// use crate::expr::{Visitor, Expr};
// use crate::token::Token;
// use crate::lox_type::LoxValue;
// pub struct AstPrinter{

// }

// impl AstPrinter{
//     pub fn print(&mut self, expr: Box<Expr>) -> String{
//         return  expr.accept(self);
//     }

//     fn parathesize(&mut self, name : &String, exprs: &Vec<&Box<Expr>>) -> String{
//         let mut built_string = "".to_string();
        
//         built_string = [built_string, "(".to_string(), name.clone()].concat();
//         for expr in exprs{
//             built_string = [built_string, " ".to_string()].concat();
//             built_string = [built_string, expr.accept(self)].concat()
//         }

//         built_string = [built_string, ")".to_string()].concat();
//         built_string.to_string()
//     }
// }

// impl Visitor<String> for AstPrinter{
//     fn visit_binary_expr(&mut self, left: &Box<Expr>, operator : &Token, right : &Box<Expr>) -> String{
//         self.parathesize(&operator.lexeme, &vec![&left, &right])
//     }
//     fn visit_grouping_expr(&mut self, expression : &Box<Expr>) -> String{
//         self.parathesize(&"group".to_string(), &vec![&expression])
//     }
//     fn visit_literal_expr(&mut self, value : &LoxValue) -> String{
//         value.to_string()
//     }
//     fn visit_unary_expr(&mut self, operator : &Token, expression : &Box<Expr>) -> String{
//         self.parathesize(&operator.lexeme, &vec![&expression])
//     }
// }

// //AST printer test code 
// // let expression = Expr::Binary { 
// //     left: (Box::new(Expr::Unary 
// //         { operator: Token::new(TokenType::MINUS, "-".to_string(), LoxValue::Nil, 1), 
// //         right: (Box::new(Expr::Literal { value: (LoxValue::Number(123.0)) })) })), 
// //     operator: Token::new(TokenType::STAR, "*".to_string(), LoxValue::Nil, 1), 
// //     right: Box::new(Expr::Grouping { expression: (Box::new(Expr::Literal { value: LoxValue::Number(45.67) })) }) 
// // };
// // let mut printer = AstPrinter {};
// // println!("{}", printer.print(expression));