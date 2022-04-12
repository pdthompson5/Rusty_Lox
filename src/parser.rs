
use crate::{token::{TokenType::{self, *}, Token}, lox_type::LoxValue};
use crate::expr::Expr;
use crate::stmt::Stmt;
//The rampant lifetime annotations in the file are a result of a vicious wrestling match the the borrow checker. 
//I am not sure how many are needed by this works
//What they mean to me is that the expression returned by parse is only valid as long as the tokens vector is in scope.
//That previous statement is true as the expression include pointers to tokens in that vector
pub struct Parser<'a>{
    tokens: &'a Vec<Token>,
    current: u32,
}

impl<'a> Parser<'a>{
    pub fn new(tokens: &'a Vec<Token>) -> Self{
        Parser { 
            tokens: tokens, 
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<Stmt>>, ()>{
        let mut statements : Vec<Box<Stmt>> = vec![];
        let mut had_error = false;
        while !self.is_at_end(){
            match self.declaration(){
                Ok(statement) => statements.push(statement),
                Err(()) => had_error = true,
            };
        }

        if had_error {
            Err(())
        } else{
            Ok(statements)
        }
    }

    pub fn declaration(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        let statement = {
            if self.match_token(vec![VAR]){
                self.var_declaration()
            } else{
                self.statement()
            }
        };

        match statement{
            Ok(stmt) => Ok(stmt),
            Err(()) => {
                self.synchronize();
                Err(())
            }
        }
    } 


    
    pub fn var_declaration(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        let name = match self.consume(IDENTIFIER, "Expect variable name.".to_string()){
            Ok(identifier) => identifier,
            Err(()) => return Err(()) 
        };

        
        let initializer = {
            if self.match_token(vec![EQUAL]){
                match self.expression(){
                    Ok(expr) => expr,
                    Err(_expr) => return Err(())
                }
            } else{
                Box::new(Expr::Literal { value: LoxValue::Nil})
            }
        };

        if let Err(()) = self.consume(SEMICOLON, "Expect ';' after variable declaration.".to_string()){
            Err(())
        } else{
            Ok(Box::new(Stmt::Var { name, initializer }))
        }

    }

    pub fn while_statement(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        self.consume(LEFT_PAREN, "Expect '(' after 'while'.".to_string())?;

        let condition = match self.expression(){
            Ok(expr) => expr,
            Err(_expr) => return Err(())
        };

        self.consume(RIGHT_PAREN, "Expect ')' after condition.".to_string())?;

        let body = self.statement()?;

        Ok(Box::new(Stmt::While { condition, body }))
    }

    pub fn statement(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        if self.match_token(vec![FOR]){
            return self.for_statement()
        }
        if self.match_token(vec![IF]){
            return self.if_statement()
        }
        if self.match_token(vec![PRINT]){
            return self.print_statement()
        }
        if self.match_token(vec![WHILE]){
            return self.while_statement()
        }
        if self.match_token(vec![LEFT_BRACE]){
           match self.block(){
               Ok(statements) => return Ok(Box::new(Stmt::Block { statements })),
               Err(()) => return Err(())
           }
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        self.consume(LEFT_PAREN, "Expect '(' after 'for'.".to_string())?;

        let initializer = 
            if self.match_token(vec![SEMICOLON]){
                None
            } else if self.match_token(vec![VAR]){
                Some(self.var_declaration()?)
            } else{
                Some(self.expression_statement()?)
            };
        
        let condition = 
            if !self.check(SEMICOLON){
                match self.expression(){
                    Ok(expr) => Some(expr),
                    Err(_expr) => return Err(())
                }
            } else{
                None
            };
        self.consume(SEMICOLON, "Expect ';' after loop condition.".to_string())?;
        
        let increment = 
            if !self.check(RIGHT_PAREN){
                match self.expression(){
                    Ok(expr) => Some(expr),
                    Err(_expr) => return Err(())
                }
            } else{
                None
            };
        self.consume(RIGHT_PAREN, "Expect ')' after for clause.".to_string())?;
        
        let mut body = self.statement()?;

        body = match increment {
            Some(expr) => Box::new(Stmt::Block { statements: vec![body, Box::new(Stmt::Expression { expression: expr })]}),
            None => body
        };

        let condition_expr = match condition{
            Some(expr) => expr,
            None => Box::new(Expr::Literal { value: LoxValue::Boolean(true) })
        };

        body = Box::new(Stmt::While{condition: condition_expr, body});

        body = match initializer{
            Some(stmt) => Box::new(Stmt::Block { statements: vec![stmt, body] }),
            None => body
        };

        Ok(body)

    }

    fn if_statement(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        self.consume(LEFT_PAREN, "Expect '(' after 'if'.".to_string())?;

        let condition = match self.expression(){
            Ok(expr) => expr,
            Err(_expr) => return Err(())
        };

        self.consume(RIGHT_PAREN, "Expect ')' after if condition.".to_string())?;
        
        let then_branch = self.statement()?;
        let else_branch = {
            if self.match_token(vec![ELSE]){
                Some(self.statement()?)
            } else{
                None
            }
        };

        Ok(Box::new(Stmt::If { condition, then_branch, else_branch}))
    }


    fn block(&mut self) -> Result<Vec<Box<Stmt<'a>>>, ()> {
        let mut statements = vec![];
        
        while !self.check(RIGHT_BRACE) && !self.is_at_end(){
            statements.push(self.declaration()?)
        }
        if let Err(()) = self.consume(RIGHT_BRACE, "Expect '}' after block.".to_string()){
            Err(())
        } else{
            Ok(statements)
        }
        
    }

    pub fn print_statement(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        let expression = match self.expression(){
            Ok(expr) => expr,
            Err(_expr) => return Err(()) 
        };

        if let Err(()) = self.consume(SEMICOLON, "Expect ';' after value".to_string()){
            Err(())
        } else{
            Ok(Box::new(Stmt::Print { expression }))
        }
    }

    pub fn expression_statement(&mut self) -> Result<Box<Stmt<'a>>, ()>{
        let expression = match self.expression(){
            Ok(expr) => expr,
            Err(_expr) => return Err(()) 
        };

        if let Err(()) = self.consume(SEMICOLON, "Expect ';' after value".to_string()){
            Err(())
        } else{
            Ok(Box::new(Stmt::Expression { expression }))
        }        
    }


    fn expression(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        self.assignment()   
    }

    fn assignment(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let expr  = self.or()?;

        if self.match_token(vec![EQUAL]){
            let equals = self.previous();
            let value = self.assignment()?;

            match expr.as_ref(){
                //Make sure left side is L-value
                Expr::Variable{name} => {
                    Ok(Box::new(Expr::Assign { name , value}))
                },
                _ => {
                    crate::error_token(equals, "Invalid assignment target.".to_string());
                    Err(Box::new(Expr::Literal { value: LoxValue::Nil}))
                }
            }

            //need to be a varible expression
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let mut expr = self.and()?;

        while self.match_token(vec![OR]){
            let operator = self.previous();
            let right = self.and()?;
            expr = Box::new(Expr::Logical { left: expr, operator, right});
        }   

        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let mut expr = self.equality()?;

        while self.match_token(vec![AND]){
            let operator = self.previous();
            let right = self.equality()?;
            expr = Box::new(Expr::Logical { left: expr, operator, right});
        }

        Ok(expr)
    }

    

    //I think that I need to show that the return value won't have the mutable self refernce in it 
    fn equality(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let mut expr = self.comparison()?;

        while self.match_token(vec![BANG_EQUAL, EQUAL_EQUAL]){
            let operator = self.previous().to_owned();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary { left: expr, operator, right});
        }
       Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let mut expr = self.term()?;

        while self.match_token(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]){
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Expr::Binary{ left: expr, operator, right});
        };
        Ok(expr)
    }   


    fn term(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let mut expr = self.factor()?;
        
        while self.match_token(vec![MINUS, PLUS]){
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary { left: expr, operator, right});
        };

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        let mut expr = self.unary()?;

        while self.match_token(vec![SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary { left:expr, operator, right});
        };
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        if self.match_token(vec![BANG, MINUS]){
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary { operator, right}));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        if self.match_token(vec![FALSE]){
            return Ok(Box::new(Expr::Literal { value: LoxValue::Boolean(false)}));
        }
        if self.match_token(vec![TRUE]){
            return Ok(Box::new(Expr::Literal { value: LoxValue::Boolean(true)}));
        }
        if self.match_token(vec![NIL]){
            return Ok(Box::new(Expr::Literal { value: LoxValue::Nil}));
        }

        if self.match_token(vec![NUMBER, STRING]){
            return Ok(Box::new(Expr::Literal { value: self.previous().literal.clone() }));
        }

        if self.match_token(vec![IDENTIFIER]){
            return Ok(Box::new(Expr::Variable { name: self.previous() }))
        }
        
        if self.match_token(vec![LEFT_PAREN]){
            let expr = self.expression()?;
            if let Err(()) = self.consume(RIGHT_PAREN, "Expect ')' after expression".to_string()){
                return Err(Box::new(Expr::Literal { value: LoxValue::Nil}));
            } else{
                return Ok(Box::new(Expr::Grouping { expression: expr }));
            };
        }
        
        //No expression matched
        crate::error_token(self.peek(), "Expect Expression".to_string()); // report error
        //An expression must be returned so just return Nil. The value of the expression should never be used.
        //TODO: Determine if this is true
        Err(Box::new(Expr::Literal { value: LoxValue::Nil}))
    }


    fn match_token(&mut self, types : Vec<TokenType>) -> bool{
        for token_type in types{
            if self.check(token_type){
                self.advance();
                return true;
            }
        }
        false
    }


    fn check(&mut self, token_type : TokenType) -> bool{
        if self.is_at_end(){
            return false
        }
        self.peek().kind == token_type
    }

    fn advance(&mut self) -> &'a Token{
        if !self.is_at_end(){
            self.current += 1;
        }
        return self.previous()
    }


    fn is_at_end(&mut self) -> bool{
        self.peek().kind == EOF
    }

    fn peek(&mut self) -> &'a Token{
        self.tokens.get(self.current as usize).unwrap()
    }

    fn previous(&mut self) -> &'a Token{
        self.tokens.get((self.current-1) as usize).unwrap()
    }

    //My consume function differs from the author's because Rust does not include exceptions 
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&'a Token, ()>{
        if self.check(token_type){
            Ok(self.advance())
        } else{
            crate::error_token(self.peek(), message);
            Err(())
        }        
    }

    pub fn synchronize(&mut self){
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == SEMICOLON {
                return;
            }
            
            match self.peek().kind{
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => ()
            }

            self.advance();
        }
    }





}