mod lib;

use crate::lib::parser;
use crate::lib::types;
use std::io::{self, Write};
fn main() {
    let mut stack = types::Stack::new();

    loop {
        print!("Bprog-Interpreter> ");
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Get the input and store it in the input variable

        if input.trim() == "exit" {                 // Exit the program
            break;
        }
        parser::process_input(&input, &mut stack);

        let output: String = stack
        .iter() // Iterator for the stack items
        .map(|v| match v { // For every item in the stack pattern match on value 
            types::WValue::VString(s) => format!("{}", s),
            num => format!("{}", num),  
        })
        .collect::<Vec<String>>()     // Collects the transformed values into a vector
        .join(", ");                  // Joins the vector of string into one string with commas separating

    println!("Stack: [{}]", output);   // Print the formatted output
    }
}
