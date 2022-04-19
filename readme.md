There were a couple of times in this process where I was entirely stummped on how to translate some code. 
    Environment handling: I ended up mostly taking the exact format here. There were two key obstacles here:
        Multiple references: Solved via std::rc::Rc
        Interior mutability: Solved via std::cell::RefCell
    Resolving hashmap storage:
        I needed a unqiue identifier to reference each expresssion so I could look them up in the interpreter. Using the Expr itself
        was absolutley not viable as there are strict trait requirements for Hashmaps: (Eq and Hash. Both are difficult to implement)
        As it turns out I had a unique ID the whole time: The memory location of the expression. 
        This was inspired the source but not copied 


language extenstions: Added remainder operator for prime factorization