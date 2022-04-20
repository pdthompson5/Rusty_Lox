



## Basic information
### Project scope 
This project implements the Lox interpreter as described in [Crafting Intepreters](https://craftinginterpreters.com/) by Robert Nystrom. This project implements the AST intepreter in chapters 4 through 11. A major objective of the project was to learn Rust as I did not previously know it.
### Code locations
All project source code can be found in `src/`.
All test code can be found in `tests`.

### Executing the project
This project requires only Rust(tested on rustc v1.59.0) and the cargo package manager.

Development Run Instructions:
REPL: Simply execute `cargo run`.
Run file: `cargo run -- {filename}` replacing `filename` with the appropriate file in the root directory.

Compilation: Execute `cargo build --release`
* This constucts the optomized executable located at `target/release/rusty_lox` 
* Execute the executable using `./rusty_lox {filename}` or just `./rusty_lox` for the REPL

## Testing
Execute the test code via `cargo test`
All of the Rust code for the tests is located in `tests/all_tests.rs`. 
The code loads and executes the lox code located at `tests/resources/{test_name}/input.lox` and asserts that it is equal to `tests/resources/{test_name}/expected_output.lox`.

Most tests determine that a specific language feature works. The tests `fibonacci`, `prime_factorization` and `stack` treat the interpreter holistically.


## Error reporting 
I have attempted to directly translate the error reporting in the textbook. I have not thouroughly tested this aspect of the interpreter, but it seems to work well. 

## Lanaguage Extensions
I added the the remainder operator("%") to the Lox language. It is identical to the remainder operator in Rust. I added this operation to enable the `prime_factorization` test.   

## Citations
This project is intended to be a direct translation from Crafting Interpreters so most code is similar to code found in the textbook.

There were a couple of times in this process where I was entirely stummped on how to translate some code. At these times I consulted another Rust implementation of this Intepreter found at [UncleScientist/lox-ast](https://github.com/UncleScientist/lox-ast). 

The following is a brief description of the issues
    Environment handling: I ended up mostly taking the exact format. There were two key obstacles that I found quite challenging:
        With the enclosed environment I needed to have two mutable reference the enclosing environment. This is not allowed in Rust without reaching out to some built-in data structures.
        Multiple references: Solved via std::rc::Rc
        Mutability: Solved via std::cell::RefCell

    Resolved expression hashmap storage:
        I needed a unqiue identifier to reference each expresssion so I could look up their resolved distance in the Interpreter. Using the Expr itself, as done in the textbook, was not viable as there are strict trait requirements for Hashmaps: (Eq and Hash. Both are difficult to implement)
        As it turns out I had a unique ID the whole time: The memory location of the expression. 
        This was inspired by the [UncleScientist/lox-ast](https://github.com/UncleScientist/lox-ast) but not copied. 


