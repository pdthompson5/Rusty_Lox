
use rusty_lox::Lox;
use rusty_lox::interpreter::Interpreter;
use std::io::{BufWriter};
use std::rc::Rc;
use std::fs;

fn run_test_file(test_name: &str) -> String{
    let mut buf = BufWriter::new(Vec::new());
    let mut lox = Lox{
        had_error: false,
        had_runtime_error: false,
        interpreter: Rc::new(Interpreter::new()),
        output_buffer: &mut buf,
    };

    let absolute_filename = ["tests/resources/", test_name, "/input.lox"].concat();
    lox.run_file(&absolute_filename);

    let bytes = buf.into_inner().expect("Error reading from test buffer");
    String::from_utf8(bytes).expect("Error reading from test buffer")
}

fn read_expected_output(test_name: &str) -> String{
    fs::read_to_string(["tests/resources/", test_name, "/expected_output.lox"].concat().as_str()).expect("Error reading from expected output file")
}

fn run_and_assert(test_name: &str){
    assert_eq!(run_test_file(test_name), read_expected_output(test_name))
}



#[test]
fn test_literals(){
    run_and_assert("literals");
}
#[test]
fn test_comparisons(){
    run_and_assert("comparisons");
}
#[test]
fn test_arithmetic(){
    run_and_assert("arithmetic");
}
#[test]
fn test_global_variables(){
    run_and_assert("global_variables");
}
#[test]
fn test_colinked_recursion(){
    run_and_assert("colinked_recursion");
}
#[test]
fn test_scope(){
    run_and_assert("scope");
}
#[test]
fn test_if_control_flow(){
    run_and_assert("if_control_flow");
}
#[test]
fn test_loops(){
    run_and_assert("loops");
}
#[test]
fn test_fibonocci(){
    run_and_assert("fibonocci");
}
// #[test]
// fn test_(){
//     run_and_assert("");
// }






