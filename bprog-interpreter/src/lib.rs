pub mod interpreter {
    use super::types::{Stack, WValue as V, OpArithmetic};
    
    /// Does the arithmetic operation sent as a parameter on the top two elements of the stack
    pub fn op_arithmetic(mut stack: Stack, op: OpArithmetic) -> Stack {
        if stack.len() < 2 {
            stack.insert(0, Err(format!("Not enough arguments for {:?}", op)));
        } else {
            let b = stack.remove(0);
            let a = stack.remove(0);
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
            stack.insert(0, result);
        }
        stack
    }   
    
    /// Popps an item of the stack
    pub fn op_pop(mut stack: Stack) -> Stack {
        if stack.is_empty() {
            stack.insert(0, Err("Not enough arguments for pop".to_string()));
        } else {
            stack.remove(0);
        }
        stack
    }
    /// Turns the token into a float, if the token is not a float it is an error
    pub fn op_num(mut stack: Stack, token: &str) -> Stack {
        match V::from_string(token) {            // Pattern matches the token 
            Ok(value) => {
                stack.insert(0, Ok(value));   // if it is a valid type insert it to the stack
                stack
            }
            Err(_) => {         
                stack.insert(                              // Insert error to the stack
                    0,
                    Err(format!("Error when parsing, expected a Value,  got: {}", token)),
                );
                stack
            }
        }
    }
}

/// Parses the string input into "tokens" for example *, +, then calls the process_tokens function to 
/// To execute 
pub mod parser {
    use super::interpreter;
    use super::types::{Stack, WValue, OpArithmetic};

    pub fn process_input(line: &str) -> String {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();    // Get all symbols seperated by space
        // Process the token inputs and stack contains Result values, either f32 variable or error
        let stack = process_tokens(&tokens, false, vec![]);
        stack       
            .iter() // Iterator for the stack items
            .map(|v| match v {  // For every item in the stack pattern match on value 
                Ok(WValue::VString(s)) => format!("\"{}\"", s),
                Ok(num) => format!("{}", num),  // Transforms the value to a string
                Err(err) => format!("Error: {}", err),
            })
            .collect::<Vec<String>>()     // Collects the transformed values into a vector
            .join(" ")                    // Joins the vector of string into one string with spaces seperating
    }

    /// Processes the tokens sent by process_input and handles the different type of tokens
    /// Calls itself recursively with the next token in the list until there are not tokens left
    pub fn process_tokens(tokens: &[&str], ignore: bool, stack: Stack) -> Stack {
        if tokens.is_empty() {
            stack
        } else {
            let token = &tokens[0];
            // Pattern matches the tokens and exectues operation if pattern matches
            let (new_ignore, new_stack) = match tokens[0] {
                "\"" => (true, stack),
                "*" if !ignore => {
                    let new_stack = interpreter::op_arithmetic(stack, OpArithmetic::Multiply);
                    (ignore, new_stack)
                }
                "+" if !ignore => {
                    let new_stack = interpreter::op_arithmetic(stack, OpArithmetic::Add);
                    (ignore, new_stack)
                }
                "-" if !ignore => {
                    let new_stack = interpreter::op_arithmetic(stack, OpArithmetic::Subtract);
                    (ignore, new_stack)
                }
                "/" if !ignore => {
                    let new_stack = interpreter::op_arithmetic(stack, OpArithmetic::Divide);
                    (ignore, new_stack)
                }
                "pop" if !ignore => {
                    let new_stack = interpreter::op_pop(stack);
                    (ignore, new_stack)
                }
                _ if !ignore => {
                    let new_stack = interpreter::op_num(stack, token);
                    (ignore, new_stack)
                }
                _ => (ignore, stack),
            };
    
            process_tokens(&tokens[1..], new_ignore, new_stack)
        }
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