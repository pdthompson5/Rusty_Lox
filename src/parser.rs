use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::lox_type::LoxValue;
use crate::token::{Token, TokenType::{self, *}};
use std::rc::Rc;
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: u32,
    had_error: bool
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
            had_error: false
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, ()> {
        let mut statements: Vec<Rc<Stmt>> = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(()) => self.had_error = true,
            };
        }

        if self.had_error {
            Err(())
        } else {
            Ok(statements)
        }
    }

    pub fn declaration(&mut self) -> Result<Rc<Stmt>, ()> {
        let statement = {
            if self.match_token(vec![VAR]) {
                self.var_declaration()
            } else if self.match_token(vec![FUN]) {
                self.function("function".to_string())
            } else {
                self.statement()
            }
        };

        match statement {
            Ok(stmt) => Ok(stmt),
            Err(()) =>{ 
                self.synchronize();     
                self.had_error = true;
                //Return nil statement in order to reset error propagation to avoid continuous synchronization 
                Ok(Rc::new(Stmt::Expression { expression: Rc::new(Expr::Literal { value: LoxValue::Nil }) }))
            }
        }
    }

    pub fn var_declaration(&mut self) -> Result<Rc<Stmt>, ()> {
        let name = match self.consume(IDENTIFIER, "Expect variable name.".to_string()) {
            Ok(identifier) => identifier,
            Err(()) => return Err(()),
        };

        let initializer = {
            if self.match_token(vec![EQUAL]) {
                match self.expression() {
                    Ok(expr) => expr,
                    Err(_expr) => return Err(()),
                }
            } else {
                Rc::new(Expr::Literal {
                    value: LoxValue::Nil,
                })
            }
        };

        if let Err(()) = self.consume(
            SEMICOLON,
            "Expect ';' after variable declaration.".to_string(),
        ) {
            Err(())
        } else {
            Ok(Rc::new(Stmt::Var { name, initializer }))
        }
    }

    pub fn while_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        self.consume(LEFT_PAREN, "Expect '(' after 'while'.".to_string())?;

        let condition = match self.expression() {
            Ok(expr) => expr,
            Err(_expr) => return Err(()),
        };

        self.consume(RIGHT_PAREN, "Expect ')' after condition.".to_string())?;

        let body = self.statement()?;

        Ok(Rc::new(Stmt::While { condition, body }))
    }

    pub fn statement(&mut self) -> Result<Rc<Stmt>, ()> {
        if self.match_token(vec![FOR]) {
            return self.for_statement();
        }
        if self.match_token(vec![IF]) {
            return self.if_statement();
        }
        if self.match_token(vec![PRINT]) {
            return self.print_statement();
        }
        if self.match_token(vec![RETURN]) {
            return self.return_statement();
        }
        if self.match_token(vec![WHILE]) {
            return self.while_statement();
        }
        if self.match_token(vec![LEFT_BRACE]) {
            match self.block() {
                Ok(statements) => return Ok(Rc::new(Stmt::Block { statements })),
                Err(()) => return Err(()),
            }
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        self.consume(LEFT_PAREN, "Expect '(' after 'for'.".to_string())?;

        let initializer = if self.match_token(vec![SEMICOLON]) {
            None
        } else if self.match_token(vec![VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(SEMICOLON) {
            match self.expression() {
                Ok(expr) => Some(expr),
                Err(_expr) => return Err(()),
            }
        } else {
            None
        };
        self.consume(SEMICOLON, "Expect ';' after loop condition.".to_string())?;

        let increment = if !self.check(RIGHT_PAREN) {
            match self.expression() {
                Ok(expr) => Some(expr),
                Err(_expr) => return Err(()),
            }
        } else {
            None
        };
        self.consume(RIGHT_PAREN, "Expect ')' after for clause.".to_string())?;

        let mut body = self.statement()?;

        body = match increment {
            Some(expr) => Rc::new(Stmt::Block {
                statements: vec![body, Rc::new(Stmt::Expression { expression: expr })],
            }),
            None => body,
        };

        let condition_expr = match condition {
            Some(expr) => expr,
            None => Rc::new(Expr::Literal {
                value: LoxValue::Boolean(true),
            }),
        };

        body = Rc::new(Stmt::While {
            condition: condition_expr,
            body,
        });

        body = match initializer {
            Some(stmt) => Rc::new(Stmt::Block {
                statements: vec![stmt, body],
            }),
            None => body,
        };

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        self.consume(LEFT_PAREN, "Expect '(' after 'if'.".to_string())?;

        let condition = match self.expression() {
            Ok(expr) => expr,
            Err(_expr) => return Err(()),
        };

        self.consume(RIGHT_PAREN, "Expect ')' after if condition.".to_string())?;

        let then_branch = self.statement()?;
        let else_branch = {
            if self.match_token(vec![ELSE]) {
                Some(self.statement()?)
            } else {
                None
            }
        };

        Ok(Rc::new(Stmt::If {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, ()> {
        let mut statements = vec![];

        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        if let Err(()) = self.consume(RIGHT_BRACE, "Expect '}' after block.".to_string()) {
            Err(())
        } else {
            Ok(statements)
        }
    }

    pub fn print_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        let expression = match self.expression() {
            Ok(expr) => expr,
            Err(_expr) => return Err(()),
        };

        if let Err(()) = self.consume(SEMICOLON, "Expect ';' after value".to_string()) {
            Err(())
        } else {
            Ok(Rc::new(Stmt::Print { expression }))
        }
    }

    pub fn expression_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        let expression = match self.expression() {
            Ok(expr) => expr,
            Err(_expr) => return Err(()),
        };

        if let Err(()) = self.consume(SEMICOLON, "Expect ';' after value".to_string()) {
            Err(())
        } else {
            Ok(Rc::new(Stmt::Expression { expression }))
        }
    }

    pub fn return_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        let keyword = self.previous();
        let value = {
            if !self.check(SEMICOLON) {
                self.expression()?
            } else {
                Rc::new(Expr::Literal {
                    value: LoxValue::Nil,
                })
            }
        };

        self.consume(SEMICOLON, "Expect ';' after return value.".to_string())?;
        Ok(Rc::new(Stmt::Return { keyword, value }))
    }


    pub fn function(&mut self, kind: String) -> Result<Rc<Stmt>, ()>{   
        let name = self.consume(IDENTIFIER, ["Expect ".to_string() , kind.clone(), " name.".to_string()].concat())?;
        self.consume(LEFT_PAREN, ["Expect '(' after ".to_string() , kind.clone(), " name.".to_string()].concat())?;
    
        let mut params = vec![];
        if !self.check(RIGHT_PAREN) {
            //This is a strange way to emulate a do-while loop from: https://gist.github.com/huonw/8435502
            //It works because the conditional of a while loop can be any expression, including a block expression
            while {
                if params.len() >= 255 {
                    crate::error_token(
                        &self.peek(),
                        "Can't have more than 255 parameters.".to_string(),
                    );
                    return Err(());
                }
                params.push(self.consume(IDENTIFIER, "Expect parameter name.".to_string())?);

                self.match_token(vec![COMMA])
            } {}
        }
        self.consume(RIGHT_PAREN, "Expect ')' after parameters.".to_string())?;
        self.consume(
            LEFT_BRACE,
            ["Expect '{' before ".to_string(), kind, " body.".to_string()].concat(),
        )?;
        let body = self.block()?;

        Ok(Rc::new(Stmt::Function { name, params, body }))
    }

    fn expression(&mut self) -> Result<Rc<Expr>, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Rc<Expr>, ()> {
        let expr = self.or()?;

        if self.match_token(vec![EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr.as_ref() {
                //Make sure left side is L-value
                Expr::Variable { name } => Ok(Rc::new(Expr::Assign {
                    name: name.clone(),
                    value,
                })),
                _ => {
                    crate::error_token(&equals, "Invalid assignment target.".to_string());
                    Err(())
                }
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.and()?;

        while self.match_token(vec![OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Rc::new(Expr::Logical {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.equality()?;

        while self.match_token(vec![AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Rc::new(Expr::Logical {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous().to_owned();
            let right = self.comparison()?;
            expr = Rc::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.term()?;

        while self.match_token(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Rc::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.factor()?;

        while self.match_token(vec![MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Rc::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.unary()?;

        while self.match_token(vec![SLASH, STAR, PERCENTAGE]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Rc::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Rc<Expr>, ()> {
        if self.match_token(vec![BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Rc::new(Expr::Unary { operator, right }));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Rc<Expr>, ()> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![LEFT_PAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Rc<Expr>) -> Result<Rc<Expr>, ()> {
        let mut arguments = vec![];
        if !self.check(RIGHT_PAREN) {
            //The following is a do-while loop
            while {
                arguments.push(self.expression()?);
                if arguments.len() >= 255 {
                    crate::error_token(
                        &self.peek(),
                        "Can't have more than 255 arguments.".to_string(),
                    );
                    return Err(());
                };
                self.match_token(vec![COMMA])
            } {}
        }

        let paren = match self.consume(RIGHT_PAREN, "Expect ')' after arguments.".to_string()) {
            Ok(token) => token,
            Err(()) => return Err(()),
        };

        Ok(Rc::new(Expr::Call {
            callee,
            paren,
            arguments,
        }))
    }

    fn primary(&mut self) -> Result<Rc<Expr>, ()> {
        if self.match_token(vec![FALSE]) {
            return Ok(Rc::new(Expr::Literal {
                value: LoxValue::Boolean(false),
            }));
        }
        if self.match_token(vec![TRUE]) {
            return Ok(Rc::new(Expr::Literal {
                value: LoxValue::Boolean(true),
            }));
        }
        if self.match_token(vec![NIL]) {
            return Ok(Rc::new(Expr::Literal {
                value: LoxValue::Nil,
            }));
        }

        if self.match_token(vec![NUMBER, STRING]) {
            return Ok(Rc::new(Expr::Literal {
                value: self.previous().literal.clone(),
            }));
        }

        if self.match_token(vec![IDENTIFIER]) {
            return Ok(Rc::new(Expr::Variable {
                name: self.previous(),
            }));
        }

        if self.match_token(vec![LEFT_PAREN]) {
            let expr = self.expression()?;
            if let Err(()) = self.consume(RIGHT_PAREN, "Expect ')' after expression".to_string()) {
                return Err(());
            } else {
                return Ok(Rc::new(Expr::Grouping { expression: expr }));
            };
        }

        //No expression matched
        crate::error_token(&self.peek(), "Expect Expression".to_string()); // report error
        Err(())
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().kind == EOF
    }

    fn peek(&mut self) -> Token {
        self.tokens.get(self.current as usize).unwrap().clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens
            .get((self.current - 1) as usize)
            .unwrap()
            .clone()
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ()> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            crate::error_token(&self.peek(), message);
            Err(())
        }
    }

    //See error reporting in readme for a discussion on issues with this function 
    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == SEMICOLON {
                return;
            }

            match self.peek().kind {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN =>  return,
                _ => (),
            }
            

            self.advance();
        }
    }
}
