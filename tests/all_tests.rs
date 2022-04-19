

use std::fs;
fn read_file(filename: &str) -> String{
    fs::read_to_string(filename).expect("Failed to find test file")
}

#[cfg(test)]
fn test_test(){
    print!("{}", read_file("tests/resources/all_tests.lox"));
    assert!(false)
}
