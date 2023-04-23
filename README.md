# Bprog-Language Interpreter

## Requirements and how to run the bprog-interpreter:
Make sure you have rust installed on your pc, you can do the command 'rustc --version'. if it is not installed, you can follow the instructions on the offical rust download site: https://www.rust-lang.org/tools/install

The next step is to clone this gir repository onto your own machine. Click the big blue button named "Clone" on the right of your screen, then use either https or ssh and copy the link. Then run "git clone 'link' 'optional name for project'". The last step is to navigate to the folder with the projcet and do the command "cargo run". Now the program should start!

## Bprog-Language Interpreter Manual V1

1. Introduction

The Bprog-Language Interpreter is a stack-based language interpreter designed to process Bprog operations and expressions. This manual will provide an overview of the Bprog-Language syntax and the available functions within the program.

2. Syntax

The Bprog-Language is whitespace-delimited and relies on a stack for handling operations. To perform an operation, enter the arguments you want the operation to be performed on for example: "3 2", followed by the operation itself, for exampel '+'. This will first push the 3 onto the stack and then 2, when the '+' operation is encountered, it is consumed and the resulting stack will be "5". The syntax utilizes reverse polish notation for most of the operations, which means that you first enter the values and the the operation for example:
" [ 1 2 3 4 ] head ". However, there are control flow expressions control flow operations that uses a infix notation for example: " 1 1 == if { 10 10 + } { 10 10 * } " where 'if' is the operation, this input would produce the stack " 20 ", check the "Operation-manual.md" if you are unsure about the syntax for each operation.

3. Data Types

The Bprog-Language supports the following data types:

- Integers (e.g., 42)
- Floats (e.g., 3.14)
- Booleans (true or false)
- Strings (e.g., "hello")
- Lists (e.g., [1, 2, 3])
- Code Blocks (e.g., { + })

4. Operations

The Bprog-Language supports various operations, such as:

- Arithmetic operations: +, -, *, /, div
- Comparison operations: >, <, ==
- Logical operations: &&, ||, not
- List operations: head, tail, empty, length, cons, append, map, each, foldl
- Control flow operations: if, loop, times
- Assignment operations: :=, fun 

For explanation on how each of these works see the operation manual in the file "Operation-manual.md"

5. Using Bprog

To use the Bprog-Language Interpreter, enter a series of tokens separated by whitespace. Tokens can be data types or operations.

Examples:

"8 10 >" This will push 8 and 10 onto the stack and then check if 8 is larger than 10 and the resulting stack will be "false"
"5 5 == if { 10 + } { 10 * }" Here we push to 5's onto the stack and then check if they are equal, which they are and the resulting stack is now "true". If checks the value on the stack, and if it is true performs the first codeblock and in the case of false performs the second codeblcok. So here the resulting stack will be "15 5"

6. Advanced Features

The Bprog-Language Interpreter also supports advanced features, such as:

- Defining and using variables
- Defining and using functions
- Conditional execution using if, else, and elif
- Support for nested lists and codeblocks

7. Troubleshooting

If you encounter issues or errors while using the Bprog-Language Interpreter, there should be an error message explaining what went wrong. If you still have issues please make sure that:

- Your input is properly formatted, with whitespace separating each token.
- There are no mismatched parentheses, brackets, or braces.
- The stack contains the necessary elements for the desired operation.

You can also check the manual for operations located in the file "Operation-manual.md" for trubleshooting if there are issues with the operations. 

8. Have fun with the Bprog-language!


## Other documentation files: 
- Non-functional-requirements.md
- Functional-requirements.md
- Self-assesment.md
- Operation-manual.md
