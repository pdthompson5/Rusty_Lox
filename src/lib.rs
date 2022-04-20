//This is file is essentially the main file
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::token::{Token, TokenType};
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::rc::Rc;
mod environment;
mod expr;
pub mod interpreter;
mod lox_callable;
mod lox_function;
mod lox_type;
mod native_function;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;

use crate::scanner::Scanner;

pub fn entry_point() {
    let mut lox = Lox {
        had_error: false,
        had_runtime_error: false,
        interpreter: Rc::new(Interpreter::new()),
        output_buffer: &mut io::stdout(),
    };
    lox.main();
}
pub struct Lox<'a> {
    pub had_error: bool,
    pub had_runtime_error: bool,
    pub interpreter: Rc<Interpreter>,
    pub output_buffer: &'a mut dyn Write,
}

fn error(line: u32, message: &String) {
    report(line, &String::from(""), message);
}

fn error_token(token: &Token, message: String) {
    if token.kind == TokenType::EOF {
        report(token.line, &" at end".to_string(), &message);
    } else {
        let location = [" at '", token.lexeme.as_str(), "'"].concat();
        report(token.line, &location, &message);
    }
}

fn report(line: u32, location: &String, message: &String) {
    println!("[line {}] Error {} : {}", line, location, message);
}

impl<'a> Lox<'a> {
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
    fn error_exit(&mut self) {
        if self.had_runtime_error {
            std::process::exit(70);
        }
        if self.had_error {
            std::process::exit(65);
        }
    }

    pub fn run_file(&mut self, path: &String) {
        let source: String =
            fs::read_to_string(path).expect(["Cannot find file: ", path].concat().as_str());
        match self.run(source) {
            Ok(()) => return,
            Err(()) => self.error_exit(),
        }
    }

    fn run_prompt(&mut self) {
        loop {
            write!(self.output_buffer, ">>> ").expect("Could not write to provided output buffer");
            io::stdout().flush().unwrap();

            let stdin = io::stdin();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(_a) => (),
                Err(_e) => break,
            }

            if line.eq("\n") {
                break;
            }

            //do not exit repl due to error
            match self.run(line) {
                Ok(()) => (),
                Err(()) => (),
            }

            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) -> Result<(), ()> {
        let mut scanner = Scanner::new(source);

        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(tokens) => {
                self.had_error = true;
                tokens
            }
        };

        let mut parser = Parser::new(tokens);
        let statements = match parser.parse() {
            Ok(expr) => expr,
            Err(()) => {
                self.had_error = true;
                return Err(());
            }
        };

        //No need to keep the scanner or parser in memory
        drop(parser);
        drop(scanner);

        let resolver = Resolver::new(self.interpreter.clone());
        match resolver.resolve_vec(&statements) {
            Ok(()) => (),
            Err(error) => {
                self.had_error = true;
                crate::error(error.line, &error.message);
                self.error_exit()
            }
        }

        drop(resolver);

        match self.interpreter.interpret(statements, self.output_buffer) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.had_runtime_error = true;
                crate::error(error.line, &error.message);
                Err(())
            }
        }
    }
}
