pub mod interpreter {
    use std::string;

    use super::parser;

    use super::types::{Stack, WValue as V, OpBinary};
    
    /// Does the arithmetic operation sent as a parameter on the top two elements of the stack
    pub fn op_binary(stack: &mut Stack, op: OpBinary) -> Result<V, String> {
        if stack.len() < 2 {
            Err(format!("Not enough arguments for {:?}", op))
        } else {
            let b = stack[0].clone();       // mutable copy
            let a = stack[1].clone();
            // Mathces the types of the top two elements on the stack with the opperation
            let result = a.and_then(|a_val| b.and_then(|b_val| {
                match (a_val, b_val, op) {
                    // Here type a matches type b
                    (V::VInt(a), V::VInt(b), OpBinary::Add) => Ok(V::VInt(a + b)),
                    (V::VFloat(a), V::VFloat(b), OpBinary::Add) => Ok(V::VFloat(a + b)),

                    (V::VInt(a), V::VInt(b), OpBinary::Subtract) => Ok(V::VInt(a - b)),
                    (V::VFloat(a), V::VFloat(b), OpBinary::Subtract) => Ok(V::VFloat(a - b)),

                    (V::VInt(a), V::VInt(b), OpBinary::Multiply) => Ok(V::VInt(a * b)),
                    (V::VFloat(a), V::VFloat(b), OpBinary::Multiply) => Ok(V::VFloat(a * b)),

                    (V::VInt(a), V::VInt(b), OpBinary::IDivide) => Ok(V::VInt(a / b)),
                    (V::VFloat(a), V::VFloat(b), OpBinary::FDivide) => Ok(V::VFloat(a / b)),

                    (V::VInt(a), V::VInt(b), OpBinary::RGreater) => Ok(V::VBool(a < b)),
                    (V::VFloat(a), V::VFloat(b), OpBinary::RGreater) => Ok(V::VBool(a < b)),

                    (V::VInt(a), V::VInt(b), OpBinary::LGreater) => Ok(V::VBool(a > b)),
                    (V::VFloat(a), V::VFloat(b), OpBinary::LGreater) => Ok(V::VBool(a > b)),

                    (V::VInt(a), V::VInt(b), OpBinary::Equality) => Ok(V::VBool(a == b)),
                    // Searched a bit online and found that comparing one float subtracted by another to epsilon is better
                    // than using == which can cause problems
                    (V::VFloat(a), V::VFloat(b), OpBinary::Equality) => Ok(V::VBool((a - b).abs() < f32::EPSILON)),
                    
                    (V::VBool(a), V::VBool(b), OpBinary::Equality) => Ok(V::VBool(a == b)),
                    (V::VBool(a), V::VBool(b), OpBinary::And) => Ok(V::VBool(a && b)),
                    (V::VBool(a), V::VBool(b), OpBinary::Or) => Ok(V::VBool(a || b)),

                    // Allowed operations where types do not fully match
                    // TODO
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
    pub fn op_dup(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if stack.len() >= 1 {
            let top_element = stack[0].clone();
            stack.push(top_element);
        }
        else {
            println!("Error: No elemets on the stack to duplicate!");
        }

        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Swaps the order of the top two elements on the stack
    pub fn op_swap(stack: &mut Stack, ignore: bool, tokens: &[&str]) {
        if stack.len() >= 2 {
            stack.swap(0, 1);
        }
        else {
            println!("Error: Need atleast two elements to perform swap!");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Prints the top element on the stack, works only for String types
    pub fn op_print(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if let Some(top_element) = stack.get(0) {       // Get top element if there is one 
            match top_element {
                Ok(V::VString(s)) => {                  // Top element is of type String
                    println!("{}", s);
                    // We check that there is an element above so do not need to use the Result return type
                    stack.remove(0);                    // Remove top element after printing it
                }
                Ok(_) => {
                    println!("Error: print not supported for other types than VString!");
                }
                Err(_) => {
                    println!("Error: there is an error value at the top of the stack.");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to print!");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Changes the top element to the not of it
    pub fn op_not(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if let Some(top_element) = stack.get_mut(0) {
            match top_element {
                Ok(V::VBool(b)) => {       
                    *b = !*b;                 
                }
                Ok(V::VInt(i)) => {       
                    *i = -*i;                 
                }
                Ok(V::VFloat(f)) => {       
                    *f = -*f;                 
                }
                Ok(_) => {
                    println!("Error: print not supported for other types than VString!");
                }
                Err(_) => {
                    println!("Error: there is an error value at the top of the stack!");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to perform logical not on!")
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Parse a string from stack to a integer and puts it back onto the stack
    pub fn op_parse_num(stack: &mut Stack, ignore: bool, parse_float: bool, tokens: &[&str]) {
        if let Some(top_element) = stack.get_mut(0) {
            match top_element {
                Ok(V::VString(s)) => {
                    let s_no_qoutes = &s[1..s.len() - 1];   // Remove the surrounding qoutes
                    if parse_float {
                        if let Ok(f) = s_no_qoutes.parse::<f32>() {
                            *top_element = Ok(V::VFloat(f)); // Replace the VString with the parsed VFloat
                        } else {
                            println!("Error: Unable to parse string to float!");
                        }
                    } else {
                        if let Ok(i) = s_no_qoutes.parse::<i32>() {
                            *top_element = Ok(V::VInt(i)); // Replace the VString with the parsed VInt
                        } else {
                            println!("Error: Unable to parse string to integer!");
                        }
                    }
                }
                Ok(_) => {
                    println!("Error: Tried to parse a non supported type, only works for VString!");
                }
                Err(_) => {
                    println!("Error: there is an error value at the top of the stack!");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to parse!");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Adds a String, codeblock or list to the stack depending on the 'starting_symbol'
    pub fn op_enclosed(stack: &mut Stack, ignore: bool, tokens: &[&str], starting_symbol: String) {
        let mut new_string = String::new();            // {} and "" represented as a string
        let mut new_elements: Vec<V> = Vec::new();     // [] list represented as a vector 
        let mut index = 0;
        let initial_stack_len = stack.len();

        // Function only called for these three starting symbols, no need for error handling
        let (start_char, end_char): (String, String) = match starting_symbol.as_str() {
            "\"" => ("\"".to_string(), "\"".to_string()),
            "[" => ("[".to_string(), "]".to_string()),
            "{" => ("{".to_string(), "}".to_string()),
            // This was just to satisfy the exhaustive pattern, function will never be called with any other symbol.
            _ => panic!("Invalid starting symbol"), 
        };
        
        let mut i = 1;
        // enumerate returns a tuple with the index and token skips the first ", {, [
        while let Some(token) = tokens.get(i) {
        if *token == end_char {                  // Token ends with correct end_char  
            index = i;                                      // Set the index to the last token
            break;
        } else {
            match start_char.as_str() {
                "\"" | "{" => {                     // Just push token + space 
                    new_string.push_str(token);
                    if tokens.get(i + 1).map_or(false, |t| *t != end_char) {
                        new_string.push(' ');
                    }
                    i += 1;
                }
                "[" => {
                    if *token == "\"" {        // Process a string within the list
                        let mut sub_tokens = Vec::new();    // Vec with tokens following initial "
                        sub_tokens.push("\"");
                        let mut j = i + 1;                 
                        // Goes through all the tokens untill it finds the closing " and adds them to the new stack
                        while let Some(token) = tokens.get(j) {
                            sub_tokens.push(token);
                            if token.ends_with("\"") {
                                break;
                            }
                            j += 1;
                        }
                        let mut sub_stack: Stack = Vec::new();          // Dummy stack to send to op_enclosed
                        op_enclosed(&mut sub_stack, ignore, &sub_tokens, "\"".to_string());
                        if let Some(value) = sub_stack.get(0) { // Get the String element from the sub stack
                            if let Ok(v) = value {              // clone the value and push it to the vector
                                new_elements.push(v.clone());
                            }
                        }
                        i = j + 1;
                    } else {                                               // Not a " token
                        new_elements.push(V::VString(token.to_string()));
                        i += 1;   
                    }
                }
                _ => {              // To satisfy exhaustive pattern
                    i += 1;
                }                             
                }
            }
        }
        if index > 0 && tokens[index] == end_char {
            match end_char.as_str() {
                "\"" => stack.insert(0, Ok(V::VString(format!("{}{}{}", start_char, new_string, end_char)))),
                "]"  => stack.insert(0, Ok(V::VList(new_elements))),
                "}"  => stack.insert(0, Ok(V::VCodeBlock(format!("{}{}{}", start_char, new_string, end_char)))),
                _ => panic!("Invalid starting symbol"), // Just to satisfy exhaustive enforcement
            }
        } else {
        println!("Error: Missing closing quote");
        stack.truncate(initial_stack_len); // Restore the stack to its initial length
        }
        parser::process_tokens(&tokens[index + 1..], ignore, stack);
    }

}

 

pub mod parser {

    use super::interpreter;
    use super::types::{Stack, OpBinary};
    
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
                "*" if !ignore => exectue_binary_op(stack, OpBinary::Multiply, ignore, &tokens), 
                "+" if !ignore => exectue_binary_op(stack, OpBinary::Add, ignore, &tokens), 
                "-" if !ignore => exectue_binary_op(stack, OpBinary::Subtract, ignore, &tokens),
                "/" if !ignore => exectue_binary_op(stack, OpBinary::FDivide, ignore, &tokens),
                "div" if !ignore => exectue_binary_op(stack, OpBinary::IDivide, ignore, &tokens),
                "<" if !ignore => exectue_binary_op(stack, OpBinary::RGreater, ignore, &tokens),
                ">" if !ignore => exectue_binary_op(stack, OpBinary::LGreater, ignore, &tokens),
                "==" if !ignore => exectue_binary_op(stack, OpBinary::Equality, ignore, &tokens),
                "&&" if !ignore => exectue_binary_op(stack, OpBinary::And, ignore, &tokens),
                "||" if !ignore => exectue_binary_op(stack, OpBinary::Or, ignore, &tokens),
                "not" if !ignore => interpreter::op_not(stack, ignore, &tokens),
                "\"" if !ignore => interpreter::op_enclosed(stack, ignore, &tokens, "\"".to_string()), 
                "[" if !ignore => interpreter::op_enclosed(stack, ignore, &tokens, "[".to_string()), 
                "{" if !ignore => interpreter::op_enclosed(stack, ignore, &tokens,"{".to_string()), 
                "dup" if !ignore => interpreter::op_dup(stack, ignore, &tokens),
                "swap" if !ignore => interpreter::op_swap(stack, ignore, &tokens),
                "print" if !ignore => interpreter::op_print(stack, ignore, &tokens),
                "parseInteger" if !ignore => interpreter::op_parse_num(stack, ignore, false, &tokens),
                "parseFloat" if !ignore => interpreter::op_parse_num(stack, ignore, true, &tokens),
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

    fn exectue_binary_op(stack: &mut Stack, op: OpBinary, ignore: bool, tokens: &[&str]) {
        if !ignore {
            match interpreter::op_binary(stack, op) {
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

    #[derive(Debug, Copy, Clone)]            // For printing out the OpBinary, for example Add.
    pub enum OpBinary {
        Add,
        Subtract,
        Multiply,
        FDivide,
        IDivide,
        RGreater,
        LGreater,
        Equality,
        And,
        Or 
    }
    #[derive(Clone, Debug)]
    pub enum WValue {
        VInt (i32),
        VFloat (f32),
        VBool (bool),
        VString (String),
        VList (Vec<WValue>),
        VCodeBlock (String),
        VOther (String)
    }
    // To display the wrapped types as strings
    impl fmt::Display for WValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                WValue::VInt(n) => write!(f, "{}", n),
                WValue::VFloat(n) => write!(f, "{}", n),
                WValue::VBool(b) => write!(f, "{}", b),
                WValue::VString(s) => write!(f, "{}", s),
                WValue::VList(list) => {   // Vec<WValue> does not implement fmt::Display so need to do it customly
                    write!(f, "[")?;
                    for (i, value) in list.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", value)?;
                    }
                    write!(f, "]")
                }
                WValue::VCodeBlock(cb) => write!(f, "{}", cb),    
                WValue::VOther(o) => write!(f, "{}", o),
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
            } else if s.starts_with("\"") && s.ends_with("\"") {
                Ok(WValue::VString(s.to_string()))
            } else {
                Ok(WValue::VOther(s.to_string()))
            }
        }
    }

}