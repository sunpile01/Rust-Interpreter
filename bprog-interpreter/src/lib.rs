pub mod interpreter {
    use super::parser;

    use super::types::{Stack, WValue as V, OpBinary};
    
    /// Does the arithmetic operation sent as a parameter on the top two elements of the stack
    pub fn op_binary(stack: &mut Stack, op: OpBinary, ignore: bool, tokens: &[&str]) {
        if stack.len() < 2 {
            println!("Error: The stack needs minimum two elements for binary operation!");
        } else {
            let b = stack[0].clone();       // mutable copy
            let a = stack[1].clone();
            let mut succesfull = true;
            // Mathces the types of the top two elements on the stack with the opperation
            match (a, b, op) {
                // Here type a matches type b
                (V::VInt(a), V::VInt(b), OpBinary::Add) => stack.insert(0, V::VInt(a + b)),
                (V::VFloat(a), V::VFloat(b), OpBinary::Add) => stack.insert(0, V::VFloat(a + b)),

                (V::VInt(a), V::VInt(b), OpBinary::Subtract) => stack.insert(0, V::VInt(a - b)),
                (V::VFloat(a), V::VFloat(b), OpBinary::Subtract) => stack.insert(0, V::VFloat(a - b)),

                (V::VInt(a), V::VInt(b), OpBinary::Multiply) => stack.insert(0, V::VInt(a * b)),
                (V::VFloat(a), V::VFloat(b), OpBinary::Multiply) => stack.insert(0, V::VFloat(a * b)),

                (V::VInt(a), V::VInt(b), OpBinary::IDivide) => stack.insert(0, V::VInt(a / b)),
                (V::VFloat(a), V::VFloat(b), OpBinary::FDivide) => stack.insert(0, V::VFloat(a / b)),

                (V::VInt(a), V::VInt(b), OpBinary::RGreater) => stack.insert(0, V::VBool(a < b)),
                (V::VFloat(a), V::VFloat(b), OpBinary::RGreater) => stack.insert(0, V::VBool(a < b)),

                (V::VInt(a), V::VInt(b), OpBinary::LGreater) => stack.insert(0, V::VBool(a > b)),
                (V::VFloat(a), V::VFloat(b), OpBinary::LGreater) => stack.insert(0, V::VBool(a > b)),

                (V::VInt(a), V::VInt(b), OpBinary::Equality) => stack.insert(0, V::VBool(a == b)),
                // Searched a bit online and found that comparing one float subtracted by another to epsilon is better
                // than using == which can cause problems
                (V::VFloat(a), V::VFloat(b), OpBinary::Equality) => stack.insert(0, V::VBool((a - b).abs() < f32::EPSILON)),
                
                (V::VBool(a), V::VBool(b), OpBinary::Equality) => stack.insert(0, V::VBool(a == b)),
                (V::VBool(a), V::VBool(b), OpBinary::And) => stack.insert(0, V::VBool(a && b)),
                (V::VBool(a), V::VBool(b), OpBinary::Or) => stack.insert(0, V::VBool(a || b)),

                // Allowed operations where types do not fully match
                // TODO
                _ => {
                    println!("The types of the top two elements are not compatible for {:?} operation", op);
                    succesfull = false;
                }
            }
            if succesfull{
                stack.remove(1);
                stack.remove(1);
            }
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }   
    
    /// Popps an item of the stack
    pub fn op_pop(stack: &mut Stack){
        if !stack.is_empty() {
            stack.remove(0);
        } else {
            println!("Error: Pop operation not executed, stack was already empty");
        }
    }

    /// Turns the token into a WValue, if the token is not a WValue it is an error
    pub fn op_num(stack: &mut Stack, token: &str) {
        match V::from_string(token) {            // Pattern matches the token 
            value => {
                stack.insert(0, value);   // if it is a valid type insert it to the stack
            }
            _ => {              // Insert error to the stack
                println!("Error: not supported type!");
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
                V::VString(s) => {                  // Top element is of type String
                    println!("{}", s);
                    // We check that there is an element above so do not need to use the Result return type
                    stack.remove(0);                    // Remove top element after printing it
                }
                _ => {
                    println!("Error: print not supported for other types than VString!");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to print!");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Reads user input and adds it to the stack as a VString
    pub fn op_read(stack: &mut Stack, ignore: bool, tokens: &[&str]) {
        use std::io;                                            // Only function that uses IO
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap().to_string(); // Get the input and store it in the input variable
        input = input.trim_end().to_string();                   // Remove \n and turn to string
        stack.insert(0, V::VString(format!("\"{}\"", input)));  // Add it to stack with ""
        parser::process_tokens(&tokens[1..], ignore, stack);
    } 

    pub fn op_words(stack: &mut Stack, ignore: bool, tokens: &[&str]) {
        if let Some(top_element) = stack.get(0){
            match top_element {
                V::VString(s) => {                      // Top element is of type String
                    let s_no_qoutes = &s[1..s.len() - 1];       // remove ""
                    let string_tokens = s_no_qoutes.split_whitespace().collect::<Vec<&str>>();
                    let mut new_elements: Vec<V> = Vec::new();

                    for token in string_tokens { // Go through all tokens and insert them into the vector, add " before and after
                        new_elements.push(V::VString(format!("\"{}\"", token)));
                }
                    stack.remove(0);
                    stack.insert(0, V::VList(new_elements));    // Insert the list with the vector elements
                }
                _ => {
                    println!("Error: print not supported for other types than VString!");
                }
            }
        } else {
            println!("Error: Stack is empty, cant get the top element!");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);

    }


    /// Changes the top element to the not of it
    pub fn op_not(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if let Some(top_element) = stack.get_mut(0) {
            match top_element {
                V::VBool(b) => {       
                    *b = !*b;                 
                }
                V::VInt(i) => {       
                    *i = -*i;                 
                }
                V::VFloat(f) => {       
                    *f = -*f;                 
                }
                _ => {
                    println!("Error: print not supported for other types than VString!");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to perform logical not on!")
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }
    /// Returns the first element from a list if the first stack element is a list
    pub fn op_head(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if let Some(V::VList(list)) = stack.get(0){ // if top element exists and is a VList
            if !list.is_empty() {
                
            } else {
                println!("Error: List is empty, cant return an element");
            }
        }
        else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Returns the last element from a list if the first stack element is a list
    pub fn op_tail(stack: &mut Stack, ignore: bool, tokens: &[&str]) {
        if let Some(V::VList(list)) = stack.get(0) {    // if top element exists and is a VList
            if !list.is_empty() {
                
            } else {
                println!("Error: List is empty, cant return an element");
            }
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }
    /// Checks if the top element on the stack is a list. If it is a list and it is empty,
    ///  it inserts false into the stack, if it is not empty it inserts true.
    pub fn op_empty(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if let Some(V::VList(list)) = stack.get(0){
            stack.insert(0, V::VBool(list.is_empty()));
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Checks if the top element on the stack is a list. If it is a list it inserts the length of the list
    pub fn op_length(stack: &mut Stack, ignore: bool, tokens: &[&str]){
        if let Some(V::VList(list)) = stack.get(0){
            stack.insert(0, V::VInt(list.len() as i32));
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    pub fn op_cons (stack: &mut Stack, ignore: bool, tokens: &[&str]) {
        if stack.len() >= 2 {
            let item = stack.remove(1);
            if let Some(V::VList(list)) = stack.get_mut(0) {
                list.insert(0, item);
            } else {
                println!("Error: Second element is not a list!");
                stack.insert(1, item);                          // Insert the item we removed.
            }
        } else {
            println!("Error: Not enough elements on the stack to perform cons.");
        }
        parser::process_tokens(&tokens[1..], ignore, stack);
    }

    /// Parse a string from stack to a integer and puts it back onto the stack
    pub fn op_parse_num(stack: &mut Stack, ignore: bool, parse_float: bool, tokens: &[&str]) {
        if let Some(top_element) = stack.get_mut(0) {
            match top_element {
                V::VString(s) => {
                    let s_no_qoutes = &s[1..s.len() - 1];   // Remove the surrounding qoutes
                    if parse_float {
                        if let Ok(f) = s_no_qoutes.parse::<f32>() {
                            *top_element = V::VFloat(f); // Replace the VString with the parsed VFloat
                        } else {
                            println!("Error: Unable to parse string to float!");
                        }
                    } else {
                        if let Ok(i) = s_no_qoutes.parse::<i32>() {
                            *top_element = V::VInt(i); // Replace the VString with the parsed VInt
                        } else {
                            println!("Error: Unable to parse string to integer!");
                        }
                    }
                }
                _ => {
                    println!("Error: Tried to parse a non supported type, only works for VString!");
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
                        if *token == "\"" {
                            let mut sub_tokens = Vec::new(); // Vec with tokens following initial "
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
                            let mut sub_stack: Stack = Vec::new(); // Dummy stack to send to op_enclosed
                            op_enclosed(&mut sub_stack, ignore, &sub_tokens, "\"".to_string());
                            if let Some(value) = sub_stack.get(0) { // Get the String element from the sub stack
                                new_elements.push(value.clone());
                            }
                            i = j + 1;
                            // TODO make the helper function work to remove duplicated code
                        } else if *token == "{" {
                            let mut sub_tokens = Vec::new();
                            sub_tokens.push("{");
                            let mut j = i + 1;
                            while let Some(token) = tokens.get(j) {
                                sub_tokens.push(token);
                                if token.ends_with("}") {
                                    break;
                                }
                                j += 1;
                            }
                            let mut sub_stack: Stack = Vec::new();
                            op_enclosed(&mut sub_stack, ignore, &sub_tokens, "{".to_string());
                            if let Some(value) = sub_stack.get(0) {
                                new_elements.push(value.clone());
                            }
                            i = j + 1;
                            // REALLY BADLY NEED THIS HELPER FUNCTION
                        } else if *token == "[" {
                            let mut sub_tokens = Vec::new();
                            sub_tokens.push("[");
                            let mut j = i + 1;
                            while let Some(token) = tokens.get(j) {
                                sub_tokens.push(token);
                                if token.ends_with("]") {
                                    break;
                                }
                                j += 1;
                            }
                            let mut sub_stack: Stack = Vec::new();
                            op_enclosed(&mut sub_stack, ignore, &sub_tokens, "[".to_string());
                            if let Some(value) = sub_stack.get(0) {
                                 new_elements.push(value.clone());
                            }
                            i = j + 1;
                        } else {
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
                "\"" => stack.insert(0, V::VString(format!("{}{}{}", start_char, new_string, end_char))),
                "]"  => stack.insert(0, V::VList(new_elements)),
                "}"  => stack.insert(0, V::VCodeBlock(format!("{}{}{}", start_char, new_string, end_char))),
                _ => panic!("Invalid starting symbol"), // Just to satisfy exhaustive enforcement
            }
        } else {
        println!("Error: Missing closing quote");
        stack.truncate(initial_stack_len); // Restore the stack to its initial length
        }
        parser::process_tokens(&tokens[index + 1..], ignore, stack);
    }
    /* 
    Tried to make a helper function here to remove the duplicated code in op_enclosed, but could not make it work
    Needs to be fixed later.
    /// Processes a String or CodeBlock item that will be added to the list vector.
    /// Many parameters because i send what needs to be updated as reference, so I dont have to duplicate code
    /// in the op_enclosed function
    fn string_or_codeblock(token: &str, i: usize, tokens: &[&str], starting_symbol: &str) -> (usize, Option<V>) {
        let mut sub_tokens = Vec::new();    // Vec with tokens following initial "
        sub_tokens.push(starting_symbol);  
        let mut j = i + 1;
        // Goes through all the tokens untill it finds the closing " and adds them to the new stack
        while let Some(token) = tokens.get(j) {
            sub_tokens.push(token);
            if token.ends_with(starting_symbol) {
                break;
            }
            j += 1;
        }
        let mut sub_stack: Stack = Vec::new();  // Dummy stack to send to op_enclosed
        op_enclosed(&mut sub_stack, false, &sub_tokens, starting_symbol.to_string());
        let value = sub_stack.get(0).and_then(|v| v.clone().ok());   // Get the String element from the sub stack
        (j, value)
    }
    */
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
                "*" if !ignore => interpreter::op_binary(stack, OpBinary::Multiply, ignore, &tokens), 
                "+" if !ignore => interpreter::op_binary(stack, OpBinary::Add, ignore, &tokens), 
                "-" if !ignore => interpreter::op_binary(stack, OpBinary::Subtract, ignore, &tokens),
                "/" if !ignore => interpreter::op_binary(stack, OpBinary::FDivide, ignore, &tokens),
                "div" if !ignore => interpreter::op_binary(stack, OpBinary::IDivide, ignore, &tokens),
                "<" if !ignore => interpreter::op_binary(stack, OpBinary::RGreater, ignore, &tokens),
                ">" if !ignore => interpreter::op_binary(stack, OpBinary::LGreater, ignore, &tokens),
                "==" if !ignore => interpreter::op_binary(stack, OpBinary::Equality, ignore, &tokens),
                "&&" if !ignore => interpreter::op_binary(stack, OpBinary::And, ignore, &tokens),
                "||" if !ignore => interpreter::op_binary(stack, OpBinary::Or, ignore, &tokens),
                "not" if !ignore => interpreter::op_not(stack, ignore, &tokens),

                "\"" if !ignore => interpreter::op_enclosed(stack, ignore, &tokens, "\"".to_string()), 
                "[" if !ignore => interpreter::op_enclosed(stack, ignore, &tokens, "[".to_string()), 
                "{" if !ignore => interpreter::op_enclosed(stack, ignore, &tokens,"{".to_string()),

                "dup" if !ignore => interpreter::op_dup(stack, ignore, &tokens),
                "swap" if !ignore => interpreter::op_swap(stack, ignore, &tokens),

                "print" if !ignore => interpreter::op_print(stack, ignore, &tokens),
                "read" if !ignore => interpreter::op_read(stack, ignore, &tokens),
                
                "parseInteger" if !ignore => interpreter::op_parse_num(stack, ignore, false, &tokens),
                "parseFloat" if !ignore => interpreter::op_parse_num(stack, ignore, true, &tokens),
                "words" if !ignore => interpreter::op_words(stack, ignore, &tokens),

                "head" if !ignore => interpreter::op_head(stack, ignore, &tokens),
                "tail" if !ignore => interpreter::op_tail(stack, ignore, &tokens),
                "empty" if !ignore => interpreter::op_empty(stack, ignore, &tokens),
                "length" if !ignore => interpreter::op_length(stack, ignore, &tokens),
                "cons" if !ignore => interpreter::op_cons(stack, ignore, &tokens),
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
}


pub mod types {
    use core::fmt;
    use std::str::FromStr;

    // Represents the program stack
    pub type Stack = Vec<WValue>;

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
        pub fn from_string(s: &str) -> WValue {
            if let Ok(num) = i32::from_str(s) {
                WValue::VInt(num)
            } else if let Ok(num) = f32::from_str(s) {
                WValue::VFloat(num)
            } else if let Ok(b) = bool::from_str(s) {
                WValue::VBool(b)
            } else if s.starts_with("\"") && s.ends_with("\"") {
                WValue::VString(s.to_string())
            } else {
                WValue::VOther(s.to_string())
            }
        }
    }

}