
use crate::{token::{TokenType::{self, *}, Token}, lox_type::LoxValue};
use crate::expr::Expr;
//The rampant lifetime annotations in the file are a result of a vicious wrestling match the the borrow checker. 
//I am not sure how many are needed by this works
//What they mean to me is that the expression returned by parse is only valid as long as the tokens vector is in scope.
//That previous statement is true as the expression include pointers to tokens in that vector
pub struct Parser<'a>{
    tokens: &'a Vec<Token>,
    current: u32,
    had_error: bool
}

impl<'a> Parser<'a>{
    pub fn new(tokens: &'a Vec<Token>) -> Self{
        Parser { 
            tokens: tokens, 
            current: 0,
            had_error: false
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, Box<Expr>>{
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<Expr<'a>>, Box<Expr<'a>>>{
        self.equality()
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
        
        if self.match_token(vec![LEFT_PAREN]){
            let expr = self.expression()?;
            self.consume(RIGHT_PAREN, "Expect ')' after expression".to_string())?;
            return Ok(Box::new(Expr::Grouping { expression: expr }));
        }
        
        //No expression matched
        crate::error_token(self.peek(), "Expect Expression".to_string()); // report errror
        return Err(Box::new(Expr::Literal { value: LoxValue::Nil})); //Returning this Nil expression signals an error. The value of the expression should never be used.
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

    fn advance(&mut self) -> &Token{
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
    //Consume should be called using the '?' operator to propagate the error
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<(), Box<Expr<'a>>>{
        if self.check(token_type){
            self.advance();
            return Ok(());
        } else{
            self.had_error = true;
            crate::error_token(self.peek(), message);
            return Err(Box::new(Expr::Literal { value: LoxValue::Nil}));
        }        
    }





}