use crate::{
    lox_type::LoxValue,
    token::TokenType::*,
    token::{Token, TokenType},
};
use std::collections::HashMap;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), AND);
        keywords.insert("class".to_string(), CLASS);
        keywords.insert("else".to_string(), ELSE);
        keywords.insert("false".to_string(), FALSE);
        keywords.insert("for".to_string(), FOR);
        keywords.insert("fun".to_string(), FUN);
        keywords.insert("if".to_string(), IF);
        keywords.insert("nil".to_string(), NIL);
        keywords.insert("or".to_string(), OR);
        keywords.insert("print".to_string(), PRINT);
        keywords.insert("return".to_string(), RETURN);
        keywords.insert("super".to_string(), SUPER);
        keywords.insert("this".to_string(), THIS);
        keywords.insert("true".to_string(), TRUE);
        keywords.insert("var".to_string(), VAR);
        keywords.insert("while".to_string(), WHILE);

        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, &Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(EOF, String::from(""), LoxValue::Nil, self.line));
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_null(LEFT_PAREN),
            ')' => self.add_token_null(RIGHT_PAREN),
            '{' => self.add_token_null(LEFT_BRACE),
            '}' => self.add_token_null(RIGHT_BRACE),
            ',' => self.add_token_null(COMMA),
            '.' => self.add_token_null(DOT),
            '-' => self.add_token_null(MINUS),
            '+' => self.add_token_null(PLUS),
            ';' => self.add_token_null(SEMICOLON),
            '*' => self.add_token_null(STAR),
            '%' => self.add_token_null(PERCENTAGE),
            //Multi character options
            '!' => {
                if self.match_char('=') {
                    self.add_token_null(BANG_EQUAL)
                } else {
                    self.add_token_null(BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token_null(EQUAL_EQUAL)
                } else {
                    self.add_token_null(EQUAL)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token_null(LESS_EQUAL)
                } else {
                    self.add_token_null(LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token_null(GREATER_EQUAL)
                } else {
                    self.add_token_null(GREATER)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_null(SLASH)
                }
            }

            //Ignore most whitespace
            ' ' => (),
            '\r' => (),
            '\t' => (),

            '\n' => {
                self.line += 1;
            }

            '"' => self.add_string(),

            _ => {
                if is_digit(c) {
                    self.add_number();
                } else if is_alpha(c) {
                    self.add_identifier();
                } else {
                    crate::error(self.line, &"Unexpected character.".to_string());
                }
            }
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.get_current_char();
        self.current = self.current + 1;
        c
    }

    //Add token if literal doesn't matter
    fn add_token_null(&mut self, kind: TokenType) {
        self.add_token(kind, LoxValue::Nil);
    }

    fn add_token(&mut self, kind: TokenType, literal: LoxValue) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(kind, text, literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        }
        else if self.get_current_char() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.get_current_char()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        
        self.source.as_bytes()[(self.current + 1) as usize] as char
    }

    fn get_current_char(&self) -> char {
        //Due to this access only single byte encoded characters can be used 
        self.source.as_bytes()[self.current as usize] as char
    }

    fn add_string(&mut self) {
        //Read until closing " is found 
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::error(self.line, &"Unterminated string.".to_string());
            return;
        }

        //consume terminating "
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(STRING, LoxValue::LoxString(value));
    }

    fn add_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        //Match fractional part
        if self.peek() == '.' && is_digit(self.peek_next()) {
            //consume '.'
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            NUMBER,
            LoxValue::Number(self.source[self.start..self.current].parse().unwrap()),
        );
    }

    fn add_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let mut type_to_add = IDENTIFIER;

        match self
            .keywords
            .get(&self.source[self.start..self.current].to_string())
        {
            Some(v) => type_to_add = *v,
            None => (),
        }

        self.add_token_null(type_to_add);
    }
}
