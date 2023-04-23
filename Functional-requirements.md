Functional requirements for the Simple-language interpreter

### Functional requirements from the user (teacher).
### Bprog must: 
- accept instructions form standard input and execute them following the semantincs of the language.
- provide the option to be used in REPL(Read Evaluate Print Loop) mode, or can take a file with instrucitons and execute them. 

- Implement the stack operations:
    - dup:  duplicates the top element on the stack.
    - swap: swaps the two top elements on the stack.
    - pop: removes the top element from the stack.

- Have the following functionality for parsing strings: 
    - parseInteger: takes a string from stack and converts it to Integer and puts it onto the stack.
    - parseFloat: Same as above but for floats.
    - words: takes a string from the stack, splits is with Rust split_whitespace command and puts a list of tokens onto the stack.

- Support the following literals that will be pushed onto the stack:
    - Integers.
    - Floats.
    - Bools.
    - Strings (delimeted by double quotes for example: " this is bprog interpreter " ).
    - Lists delimited the same way as string but with [ ].

- Support arithmetic (binary) operations: 
    - '+': addition.
    - '-': subtraction.
    - '*': multiplication.
    - '/': floating point division.
    - 'div': integer division.
    - '<': checks if x < y, and puts true or false on the stack.
    - '>': checks if x > y, and puts true or false on the stack.
    - '==': checks if x == y and puts true or false on the stack.

- Support logical opertations:
    - true: is a literal.
    - false: is a literal.
    - &&: logical AND.
    - ||: logical OR. 
    - not: logical NOT, can work for floats and integers aswell by negating them.

- Support lists: 
    - They are delimeted by square brackets as mentioned before
    - Need to support these list operaitons:
        - head: Takes a list and returns its head.
        - tail: Takes a list and returns the tail.
        - empty: Takes a list and returns true if the list is empty.
        - length: Puts the length of a given list onto the stack.
        - cons: Appends the item in front of the list.
        - append: Concatenates both lists.
        - each quotation: Takes a list an a code block, and executes the code block on each of the elements of the list
        - map quotation: Takes a list, and a block, and executes the block on each of the elements of the list, forming a new list that is put on the stack.
        - foldl quotation: folds the list from left to right. 

### Bprog should: 

- Have simple IO: 
    - print: takes the top element from the stack and prints it to the standard output.
    - read: reads a line from standard input and puts it into the stack as string.

- Support code blocks: 
    - Needs to support blocks of code.
    - The code blocks are delimited by { literals and operations here }.
    - Need to implement 'exec' to trigger a code block. 
    - Code blocks are not required for single instruction qoutations for example: 
        - 3 times { 10 } can be written as 3 times 10.
        - true if { + } { - } can be writeen as: true if + -.

- Support simple control flow: 
    - if then_block else_block: Ff expression takes a boolean value from the stack, and executes the then_code_block if true, or else_code_block if false. The executed block operates in the context of the global stack.
    - loop break block: Execute the block until break becomes True. break and block are expected to be quotations. break evaluating to True or False does not leave that value on the stack (it is consumed by the loop).
    - times block: Repeat the block num times.
    
- Support assignment to a symbol (variable)
    - Needs to support assignment denoted by ':='. Assignment takes two arguments, left hand side must be a symbol (aka variable), and right hand side can be any value different from a symbol, eg. number, bool, list or code_block.
    - Needs to support function definition denoted by 'fun'. Function definition takes two arguments, left hand side must be a symbol (aka function name), and the right hand side must be quotation (code block).
    - Once a symbol is bound to a function or variable, using that symbol will either run the function or evaluate the variable to a value.

### Functional requirements assumed by the developer: 

- When using the single instructions together with map and each for example [ " hello " " I " ] each print , it is not allowed with binary operations such as +, - .. since map and each works on the list provided.
- Using the integer division operation 'div' on two floats will convert them to an integer and then divide them
- Same goes for using float division '/' with integers, they will be converted to floats and then divided. 
- True or False does not have the standard 1 and 0 values but are rather just true or false. 
- The interpreter does not process the next tokens after an error is encountered. 
- Doing the operations: head, length, empty and tail should not remove the original list

      





