pub mod interpreter {
    use super::types::Stack;
    use std::str::FromStr;
    /// Processes the tokens sent by process_input and handles the different type of tokens
    /// Calls itself recursively with the next token in the list until there are not tokens left
    pub fn process_tokens(tokens: &[&str], ignore: bool, stack: Stack) -> Stack {
        if tokens.is_empty() {
            stack
        } else {
            let (new_ignore, new_stack) = match tokens[0] {
                "\"" => (true, stack),
                "*" if !ignore => {
                    let new_stack = op_mult(stack);
                    (ignore, new_stack)
                }
                "+" if !ignore => {
                    let new_stack = op_add(stack);
                    (ignore, new_stack)
                }
                "pop" if !ignore => {
                    let new_stack = op_pop(stack);
                    (ignore, new_stack)
                }
                _ if !ignore => {
                    let new_stack = op_num(stack, tokens[0]);
                    (ignore, new_stack)
                }
                _ => (ignore, stack),
            };
    
            process_tokens(&tokens[1..], new_ignore, new_stack)
        }
    }
    /// Removes the top two elements from the stack, multiplies them and inserts the sum of the two numbers
    fn op_mult(mut stack: Stack) -> Stack {
        if stack.len() < 2 {
            stack.insert(0, Err("Not enough arguments for *".to_string()));
        } else {
            let b = stack.remove(0);
            let a = stack.remove(0);
            stack.insert(0, a.and_then(|a_val| { b.map(|b_val| a_val * b_val)} ));
        }
        stack
    }
    /// Removes the top two elements from the stack, adds them and inserts the sum of the two numbers
    fn op_add(mut stack: Stack) -> Stack {
        if stack.len() < 2 {
            stack.insert(0, Err("Not enough arguments for +".to_string()));
        } else {
            let b = stack.remove(0);
            let a = stack.remove(0);
            stack.insert(0, a.and_then(|a_val| { b.map(|b_val| a_val + b_val)} ));
        }
        stack
    }
    /// Popps an item of the stack
    fn op_pop(mut stack: Stack) -> Stack {
        if stack.is_empty() {
            stack.insert(0, Err("Not enough arguments for pop".to_string()));
        } else {
            stack.remove(0);
        }
        stack
    }
    /// Turns the token into a float, if the token is not a float it is an error
    fn op_num(mut stack: Stack, token: &str) -> Stack {
        match f32::from_str(token) {            // Pattern matches the token 
            Ok(num) => {
                stack.insert(0, Ok(num));   // if it is a float insert it to the stack
                stack
            }
            Err(_) => {
                stack.insert(                              // Insert error to the stack
                    0,
                    Err(format!("Parsing error, expected a number, got: {}", token)),
                );
                stack
            }
        }
    }
}

/// Parses the string input into "tokens" for example *, +, then calls the process_tokens function to 
/// To execute 
pub mod parser {
    use super::interpreter::process_tokens; 

    pub fn process_input(line: &str) -> String {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();    // Get all words seperated by space
        // Process the token inputs and stack contains Result values, either f32 variable or error
        let stack = process_tokens(&tokens, false, vec![]);
        stack       
            .iter() // Iterator for the stack items
            .map(|v| match v {  // For every item in the stack pattern match on value 
                Ok(num) => format!("{}", num),  // Transforms the value to a string
                Err(err) => format!("Error: {}", err),
            })
            .collect::<Vec<String>>()     // Collects the transformed values into a vector
            .join(" ")                    // Joins the vector of string into one string with spaces seperating
    }


}

pub mod types {
    // Represents the program stack
    pub type Stack = Vec<Result<f32, String>>;
}