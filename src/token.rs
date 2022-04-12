#![allow(non_camel_case_types)]

use crate::lox_type::LoxValue;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}



pub struct Token  {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: LoxValue,
    pub line: u32,
}



impl Token{
    // pub fn to_string(&self) -> String {
    //     return format!("{:?} {} {}", self.kind, self.lexeme, self.literal);
    // }

    pub fn new(kind : TokenType, lexeme : String, literal : LoxValue, line : u32) -> Self{
        Token{
            kind,
            lexeme,
            literal,
            line
        }
    }
}