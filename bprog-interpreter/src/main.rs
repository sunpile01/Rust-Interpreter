mod lib;

use crate::lib::parser;
use crate::lib::types::{WValue as V, Stack};
use std::collections::HashMap;
use std::io::{self, Write};
fn main() {
    let mut stack = Stack::new();
    let mut symbol_table: HashMap<String, V> = HashMap::new();

    loop {
        print!("Bprog-Interpreter> ");
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Get the input and store it in the input variable

        if input.trim() == "exit" {                 // Exit the program
            break;
        }
        parser::process_input(&input, &mut stack, &mut symbol_table);

        let output: String = stack
        .iter() // Iterator for the stack items
        // Can maybe be removed?? 
        .map(|v| match v { // For every item in the stack pattern match on value 
            V::VString(s) => format!("{}", s),
            num => format!("{}", num),  
        })
        .collect::<Vec<String>>()     // Collects the transformed values into a vector
        .join(", ");                  // Joins the vector of string into one string with commas separating

    println!("Stack: [{}]", output);   // Print the formatted output
    }
}
