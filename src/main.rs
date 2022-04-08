use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use crate::token::{Token, TokenType};
use crate::parser::Parser;
use crate::interpreter::Interpreter;
mod scanner;
mod token;
mod lox_type;
mod expr;
mod interpreter;
mod ast_printer;
mod parser;
mod stmt;
mod environment;



use crate::scanner::Scanner;
//TODO: Current status: Everything is looking good, I think I'm ready to keep going 


fn main(){
    let mut lox = Lox{
        had_error: false,
        had_runtime_error: false,
        interpreter: Interpreter::new()
    };
    lox.main();
}
pub struct Lox{
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter
}

fn error(line: u32, message: &String){
    report(line, &String::from(""), message);
}

fn error_token(token : &Token, message : String){
    if token.kind == TokenType::EOF{
        report(token.line, &"at end".to_string(), &message);
    } else{
        let location = ["at '", token.lexeme.as_str(), "'"].concat();
        report(token.line, &location, &message);
    }
}

fn report(line: u32, location: &String, message: &String){
    println!("[line {}] Error {} : {}", line, location, message);
}


impl Lox{
    fn main(&mut self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 2 {
            println!("Usage: cargo run -- [script]");
            std::process::exit(65);
        } else if args.len() == 2 {
            self.run_file(args.get(1).unwrap());
        } else {
            self.run_prompt();
        }
    }
    fn error_exit(&mut self){
        if self.had_runtime_error{
            std::process::exit(70);
        }
        if self.had_error{
            std::process::exit(65);
        }
    }
    
    fn run_file(&mut self, path: &String){
        let source: String = fs::read_to_string(path).expect("Cannot find file");
        match self.run(source){
            Ok(()) => return,
            Err(()) => self.error_exit()
        }
    }
    
    fn run_prompt(&mut self){
        loop {
            print!(">>> ");
            io::stdout().flush().unwrap();
    
            let stdin = io::stdin();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line){
                Ok(_a)=> (), 
                Err(_e) => break,
            }
             
    
            if line.eq("\n"){
                break;
            }

            //do not exit repl due to error
            match self.run(line){
                Ok(()) =>(),
                Err(()) =>(),
            } 
            


            self.had_error = false;
        }
    }
    
    fn run(&mut self, source: String) -> Result<(), ()>{
        let mut scanner = Scanner::new(source);
    
        let tokens = match scanner.scan_tokens(){
            Ok(tokens) => tokens,
            Err(tokens) => {self.had_error = true; tokens}
        };
        
        // for token in tokens{
        //     println!("{}", token.to_string());
        // }

    
        let mut parser = Parser::new(tokens);
        let statements = match parser.parse(){
            Ok(expr) => expr,
            Err(()) => {self.had_error = true; return Err(())}
        };
    

        // let mut printer = AstPrinter{};
        // println!("{}", printer.print(expression));
        
        
        match self.interpreter.interpret(statements){
            Ok(()) => Ok(()),
            Err(error) => {
                self.had_runtime_error = true; 
                crate::error(error.line, &error.message);
                Err(())
            }
        }
    }
}


