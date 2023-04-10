pub mod interpreter {
    use std::string;
    use super::parser;

    use super::types::{Stack, WValue as V, OpArithmetic};
    
    /// Does the arithmetic operation sent as a parameter on the top two elements of the stack
    pub fn op_arithmetic(stack: &mut Stack, op: OpArithmetic) -> Result<V, String> {
        if stack.len() < 2 {
            Err(format!("Not enough arguments for {:?}", op))
        } else {
            let b = stack[0].clone();       // mutable copy
            let a = stack[1].clone();
            // Mathces the types of the top two elements on the stack with the opperation
            let result = a.and_then(|a_val| b.and_then(|b_val| {
                match (a_val, b_val, op) {
                    (V::VInt(a), V::VInt(b), OpArithmetic::Add) => Ok(V::VInt(a + b)),
                    (V::VFloat(a), V::VFloat(b), OpArithmetic::Add) => Ok(V::VFloat(a + b)),
                    (V::VInt(a), V::VInt(b), OpArithmetic::Subtract) => Ok(V::VInt(a - b)),
                    (V::VFloat(a), V::VFloat(b), OpArithmetic::Subtract) => Ok(V::VFloat(a - b)),
                    (V::VInt(a), V::VInt(b), OpArithmetic::Multiply) => Ok(V::VInt(a * b)),
                    (V::VFloat(a), V::VFloat(b), OpArithmetic::Multiply) => Ok(V::VFloat(a * b)),
                    (V::VInt(a), V::VInt(b), OpArithmetic::Divide) => Ok(V::VInt(a / b)),
                    (V::VFloat(a), V::VFloat(b), OpArithmetic::Divide) => Ok(V::VFloat(a / b)),
                    _ => Err(format!("The type is not supported for {:?} operation", op)),
                }
            }));
            result
        }
    }   

    /// Popps an item of the stack
    pub fn op_pop(stack: &mut Stack){
        if !stack.is_empty() {
            stack.remove(0);
        } else {
            println!("Pop operation not executed, stack was already empty");
        }
    }

    /// Turns the token into a WValue, if the token is not a WValue it is an error
    pub fn op_num(stack: &mut Stack, token: &str) {
        match V::from_string(token) {            // Pattern matches the token 
            Ok(value) => {
                stack.insert(0, Ok(value));   // if it is a valid type insert it to the stack
            }
            Err(_) => {         
                stack.insert(                              // Insert error to the stack
                    0,
                    Err(format!("Error when parsing, expected a Value,  got: {}", token)),
                );
            }
        }
    }
    /// Duplicates the top element of the stack
    pub fn op_dup(stack: &mut Stack){
        if (stack.len() >= 1){
            let top_element = stack[0].clone();
            stack.push(top_element);
        }
        else {
            println!("Error: No elemets on the stack to duplicate!");
        }
    }

    pub fn op_swap(stack: &mut Stack) {
        if (stack.len() >= 2){
            stack.swap(0, 1);
        }
        else {
            println!("Error: Need atleast two elements to perform swap!");
        }
    }

    /// Adds a String to the stack string is denoted by " <input> "
    pub fn op_quotes(stack: &mut Stack, ignore: bool, tokens: &[&str]) {
        let mut new_string = String::new();
        let mut index = 0;
        let initial_stack_len = stack.len();
        
        // enumerate returns a tuple with the index and token
        for (i, token) in tokens.iter().enumerate().skip(1) {
            if *token == "\"" {                  // Token ends with "
                new_string.push_str(&token[..token.len() - 1]);     
                index = i;                                      // Set the index to the last token
                break;
            } else {
                new_string.push_str(token);     // Push token to the new string
                if !tokens.get(i + 1).map_or(false, |next_token| next_token.ends_with("\"")) {
                    new_string.push(' ');
                }
            }
        }
        if index > 0 && tokens[index] == "\"" {
            stack.insert(0, Ok(V::VString(format!("\"{}\"", new_string))));
            parser::process_tokens(&tokens[index + 1..], ignore, stack);
        } else {
        println!("Error: Missing closing quote");
        stack.truncate(initial_stack_len); // Restore the stack to its initial length
        // Skip processing all the tokens after the opening quote
        parser::process_tokens(&tokens[tokens.len()..], ignore, stack);
    }
    }
}

 
pub mod parser {
    use super::interpreter;
    use super::types::{Stack, WValue, OpArithmetic};
    
    /// Parses the string input into "tokens" for example *, +, then calls the process_tokens function to 
    /// execute the corresponding code depending on the tokens. 
    pub fn process_input(line: &str, stack: &mut Stack) {
        let tokens = line.split_whitespace().collect::<Vec<&str>>(); // Get all symbols separated by space
        // Process the token inputs
        process_tokens(&tokens, false, stack);
    }
    
    /// Processes the tokens sent by process_input and handles the different type of tokens
    /// Calls itself recursively with the next token in the list until there are not tokens left
    pub fn process_tokens(tokens: &[&str], ignore: bool, stack: &mut Stack) {
        if !tokens.is_empty() {
            match tokens[0] {
                "*" if !ignore => exectue_arithmetic_op(stack, OpArithmetic::Multiply, ignore, &tokens), 
                "+" if !ignore => exectue_arithmetic_op(stack, OpArithmetic::Add, ignore, &tokens), 
                "-" if !ignore => exectue_arithmetic_op(stack, OpArithmetic::Subtract, ignore, &tokens),
                "/" if !ignore => exectue_arithmetic_op(stack, OpArithmetic::Divide, ignore, &tokens),
                "\"" if !ignore => interpreter::op_quotes(stack, ignore, &tokens), 
                "dup" if !ignore => interpreter::op_dup(stack),
                "swap" if !ignore => interpreter::op_swap(stack),
                "pop" if !ignore => {
                    interpreter::op_pop(stack);
                    process_tokens(&tokens[1..], ignore, stack);
                }
                _ if !ignore => {
                     interpreter::op_num(stack, tokens[0]);
                     process_tokens(&tokens[1..], ignore, stack);
                    }
                _ => process_tokens(&tokens[1..], ignore, stack),
            };
        }
    }

    fn exectue_arithmetic_op(stack: &mut Stack, op: OpArithmetic, ignore: bool, tokens: &[&str]) {
        if !ignore {
            match interpreter::op_arithmetic(stack, op) {
                Ok(val) => {                // The arithemetic operation was possible and allowed for types
                    stack.remove(0);        // remove top two elements that computed the value
                    stack.remove(0);
                    stack.insert(0, Ok(val))
                }
                Err(err) => println!("Error: {}", err), 
            }
        }
        process_tokens(&tokens[1..], ignore, stack);
    }

}

pub mod types {
    use core::fmt;
    use std::str::FromStr;

    // Represents the program stack
    pub type Stack = Vec<Result<WValue, String>>;

    #[derive(Debug, Copy, Clone)]            // For printing out the OpArithmetic, for example Add.
    pub enum OpArithmetic {
        Add,
        Subtract,
        Multiply,
        Divide,
    }
    #[derive(Clone)]
    pub enum WValue {
        VInt (i32),
        VFloat (f32),
        VBool (bool),
        VString (String),
    }
    // To display the wrapped types as strings
    impl fmt::Display for WValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                WValue::VInt(n) => write!(f, "{}", n),
                WValue::VFloat(n) => write!(f, "{}", n),
                WValue::VBool(b) => write!(f, "{}", b),
                WValue::VString(s) => write!(f, "{}", s),
            }
        }
    }
    // To convert from string to the enum type
    impl WValue {
        pub fn from_string(s: &str) -> Result<Self, String> {
            if let Ok(num) = i32::from_str(s) {
                Ok(WValue::VInt(num))
            } else if let Ok(num) = f32::from_str(s) {
                Ok(WValue::VFloat(num))
            } else if let Ok(b) = bool::from_str(s) {
                Ok(WValue::VBool(b))
            } else {
                Ok(WValue::VString(s.to_string()))
            }
        }
    }

}