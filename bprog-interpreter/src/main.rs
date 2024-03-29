mod parser;
mod types;
mod stack_operations;

use crate::parser::process_input;
use crate::types::{WValue as V, Stack};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
fn main() {
    let mut stack = Stack::new();
    let mut symbol_table: HashMap<String, V> = HashMap::new();
    let mut replmode = false;
    println!("You can now enter the input for the interpreter if you wish!");
    println!("Available commands:");
    println!("h or H - Show this help message");
    println!("r or R - Enter REPL mode");
    println!("f or F - Enter file mode");
    println!("q or Q - Quit the program");

    loop {
        print!("Bprog-Interpreter> ");
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Get the input and store it in the input variable
        let trimmed_input = input.trim();

        // Match on the user input 
        match trimmed_input.to_lowercase().as_str() {
            "q" => break, 
            "h" => {
                println!("Available commands:");
                println!("h or H - Show this help message");
                println!("r or R - Enter REPL mode (q or Q to quit repl mode)");
                println!("f or F - Enter file mode");
                println!("q or Q - Quit the program");
            }
            "r" => {
                // REPL mode
                loop {
                    print!("Bprog-Interpreter (REPL)> ");   
                    io::stdout().flush().unwrap();
                    
                    // Get new user input each time
                    let mut repl_input = String::new();                 
                    io::stdin().read_line(&mut repl_input).unwrap();

                    if repl_input.trim().to_lowercase() == "q" {
                        break;
                    }
                    replmode = true;
                    // Process the input and print the output 
                    process_input(&repl_input, &mut stack, &mut symbol_table);
                    print_stack(&stack, replmode);
                }
                stack.clear();
            }
            "f" => {
                // File mode
                print!("Enter filepath: ");
                io::stdout().flush().unwrap();

                // Get the filepath from the user
                let mut filepath = String::new();
                io::stdin().read_line(&mut filepath).unwrap();
                let filepath = filepath.trim();

                // Get the contents of the file specified by filepath
                match fs::read_to_string(filepath) {
                    Ok(contents) => {
                        // Process the file and print the resulting stack
                        replmode = false;
                        process_input(&contents, &mut stack, &mut symbol_table);
                        print_stack(&stack, replmode);
                        stack.clear();
                    },
                    Err(_) => {
                        println!("Filepath not found! File must be in the folder 'bprog-interpreter'");
                    },
                }
            }
            _ => {
                // Default mode is processing the user input and printing the resulting stack
                process_input(&trimmed_input, &mut stack, &mut symbol_table);
                print_stack(&stack, replmode);
                stack.clear();
            }
        }
    }
}

/// Would have been in a file called utilities.rs, but there are no other functions than this I would have there
/// So left it in the main as I felt it was kinda pointless with a file for 1 small function.
/// Prints the content of the stack 
fn print_stack(stack: &Stack, replmode: bool) {
    let output: String = stack
        .iter() // Iterator for the stack items
        .map(|v| match v {   // For every item in the stack pattern match on value 
            V::VString(s) => format!("{}", s),
            num => format!("{}", num),
        })
        .collect::<Vec<String>>()   // Collects the transformed values into a vector
        .join(", ");                        // Joins the vector of string into one string with commas separating
    
    println!("[{}]", output);               // Print the formatted output
    if stack.len() > 1 {
        if !replmode { 
            println!("Error: Stack Should Only Have 1 Element After Processing Tokens"); 
        }  else {
            println!("\"--\"");
        }
    }
}




