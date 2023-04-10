mod lib;

use crate::lib::parser;
use crate::lib::types;
use std::io::{self, Write};
fn main() {
    let mut stack = types::Stack::new();

    loop {
        print!("Bprog-Interpreter> ");
        io::stdout().flush().unwrap(); // Flush the stdout buffer to display the prompt immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        stack = parser::process_input(&input, stack);

        
        let output: String = stack
        .iter() // Iterator for the stack items
        .map(|v| match v { // For every item in the stack pattern match on value 
            Ok(types::WValue::VString(s)) => format!("\"{}\"", s),
            Ok(num) => format!("{}", num),  // Transforms the value to a string
            Err(err) => format!("Error: {}", err),
        })
        .collect::<Vec<String>>()     // Collects the transformed values into a vector
        .join(", ");                  // Joins the vector of string into one string with commas separating

    println!("Output: [{}]", output);
    }
}
