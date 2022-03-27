use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
mod scanner;
mod token;
mod lox_type;

use crate::scanner::Scanner;


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
    }
}

fn run(source: String){
    let mut had_error = false;
    let mut scanner = Scanner::new(source);

    let tokens = match scanner.scan_tokens(){
        Ok(tokens) => tokens,
        Err(tokens) => {had_error = true; tokens}
    };
    
    for token in tokens{
        println!("{}", token.to_string());
    }
}


fn error(line: u32, message: &String){
    report(line, &String::from(""), message);
}

fn report(line: u32, location: &String, message: &String){
    println!("[line {}] Error {} : {}", line, location, message);
    std::process::exit(65);
}