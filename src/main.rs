use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use crate::ast_printer::AstPrinter;
use crate::token::{Token, TokenType};
use crate::parser::Parser;
mod scanner;
mod token;
mod lox_type;
mod expr;
mod interpreter;
mod ast_printer;
mod parser;



use crate::scanner::Scanner;

static mut HAD_ERROR:bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: cargo run -- [script]");
        std::process::exit(65);
    } else if args.len() == 2 {
        run_file(args.get(1).unwrap());
    } else {
        run_prompt();
    }
}



fn run_file(path: &String){
    let source: String = fs::read_to_string(path).expect("Cannot find file");
    run(source);
}

fn run_prompt(){
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
        run(line);
        unsafe{
            HAD_ERROR = false;
        }
    }
}

fn run(source: String){
    let mut scanner = Scanner::new(source);

    let tokens = match scanner.scan_tokens(){
        Ok(tokens) => tokens,
        Err(tokens) => {unsafe {HAD_ERROR = true;} tokens}
    };
    
    // for token in tokens{
    //     println!("{}", token.to_string());
    // }
    
    //TODO: Now that I am setting had_error in report I think I don't need these matches
    //perhaps I can make had_error a local varible if don't set it in report

    let mut parser = Parser::new(tokens);
    let expression = match parser.parse(){
        Ok(expr) => expr,
        Err(expr) => {unsafe {HAD_ERROR = true;} expr}
    };

    unsafe{
        if HAD_ERROR{
            return;
        }
    }

    let mut printer = AstPrinter{};
    println!("{}", printer.print(expression));

    
    
    
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
    unsafe {
        HAD_ERROR = true;
    }
}

