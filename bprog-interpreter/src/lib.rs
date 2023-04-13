pub mod interpreter {
    use super::parser;

    use super::types::{Stack, WValue as V, OpBinary};
    use std::any::{Any, TypeId};

    fn print_type<T: Any>(_: &T) {
        let type_id = TypeId::of::<T>();
        println!("{:?}", type_id);
    }

    /// Does the arithmetic operation sent as a parameter on the top two elements of the stack
    pub fn op_binary(stack: &mut Stack, op: OpBinary, tokens: &[&str]) {
        if stack.len() < 2 {
            println!("Error: The stack needs minimum two elements for binary operation!");
        } else {
            let b = stack[0].clone();       // mutable copy
            let a = stack[1].clone();
            println!("The types of a and b: {:?} and {:?}", a, b);
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
                    println!("The types of the top two elements are not compatible for {:?} operation.", op);
                    succesfull = false;
                }
            }
            if succesfull{
                stack.remove(1);
                stack.remove(1);
            }
        }
        parser::process_tokens(&tokens[1..], stack);
    }   
    
    /// Popps an item of the stack
    pub fn op_pop(stack: &mut Stack){
        if !stack.is_empty() {
            stack.remove(0);
        } else {
            println!("Error: Pop operation not executed, stack was already empty");
        }
    }

    /// Turns the token into a WValue
    pub fn op_num(stack: &mut Stack, token: &str) {
        match V::from_string(token) {         
            value => {
                stack.insert(0, value);   
            }
        }
    }

    /// Duplicates the top element of the stack
    pub fn op_dup(stack: &mut Stack, tokens: &[&str]){
        if stack.len() >= 1 {
            let top_element = stack[0].clone();
            stack.push(top_element);
        }
        else {
            println!("Error: No elemets on the stack to duplicate!");
        }

        parser::process_tokens(&tokens[1..], stack);
    }

    /// Swaps the order of the top two elements on the stack
    pub fn op_swap(stack: &mut Stack, tokens: &[&str]) {
        if stack.len() >= 2 {
            stack.swap(0, 1);
        }
        else {
            println!("Error: Need atleast two elements to perform swap!");
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Prints the top element on the stack, works only for String types
    pub fn op_print(stack: &mut Stack, tokens: &[&str]){
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
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Reads user input and adds it to the stack as a VString
    pub fn op_read(stack: &mut Stack, tokens: &[&str]) {
        use std::io;                                            // Only function that uses IO
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap().to_string(); // Get the input and store it in the input variable
        input = input.trim_end().to_string();                   // Remove \n and turn to string
        stack.insert(0, V::VString(format!("\"{}\"", input)));  // Add it to stack with ""
        parser::process_tokens(&tokens[1..], stack);
    } 

    pub fn op_words(stack: &mut Stack, tokens: &[&str]) {
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
        parser::process_tokens(&tokens[1..], stack);

    }


    /// Changes the top element to the not of it
    pub fn op_not(stack: &mut Stack, tokens: &[&str]){
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
        parser::process_tokens(&tokens[1..], stack);
    }
    /// Returns the first element from a list if the first stack element is a list
    pub fn op_head(stack: &mut Stack, tokens: &[&str]){
        if let Some(V::VList(list)) = stack.get(0){ // if top element exists and is a VList
            if !list.is_empty() {
                
            } else {
                println!("Error: List is empty, cant return an element");
            }
        }
        else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Returns the last element from a list if the first stack element is a list
    pub fn op_tail(stack: &mut Stack, tokens: &[&str]) {
        if let Some(V::VList(list)) = stack.get(0) {    // if top element exists and is a VList
            if !list.is_empty() {
                
            } else {
                println!("Error: List is empty, cant return an element");
            }
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], stack);
    }
    /// Checks if the top element on the stack is a list. If it is a list and it is empty,
    ///  it inserts false into the stack, if it is not empty it inserts true.
    pub fn op_empty(stack: &mut Stack, tokens: &[&str]){
        if let Some(V::VList(list)) = stack.get(0){
            stack.insert(0, V::VBool(list.is_empty()));
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Checks if the top element on the stack is a list. If it is a list it inserts the length of the list
    pub fn op_length(stack: &mut Stack, tokens: &[&str]){
        if let Some(V::VList(list)) = stack.get(0){
            stack.insert(0, V::VInt(list.len() as i32));
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Append an element to the front of a list. Not sure about how the order should be done here
    pub fn op_cons (stack: &mut Stack, tokens: &[&str]) {
        if stack.len() >= 2 {
            let item = stack.remove(1);
            if let Some(V::VList(list)) = stack.get_mut(0) {
                list.insert(0, item);
            } else {
                println!("Error: Second element is not a list!");
                stack.insert(1, item);                          // Insert the item we removed.
            }
        } else {
            println!("Error: Not enough elements on the stack to perform cons");
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Appends one list to the end of another list. Not sure how the order should be done here
    pub fn op_append (stack: &mut Stack, tokens: &[&str]) {
        if stack.len() >= 2{
            if let Some(V::VList(list2)) = stack.get(0){    // Get the list to be appended to the other
                let list2_clone = list2.clone();
                stack.remove(0);                                    // Remove list 1
                if let Some(V::VList(list1)) = stack.get_mut(0) {   // Now we can get as mutable reference 
                    list1.extend(list2_clone);                      // Extends list1 with list2
                }
            } else {
                println!("Error: Top two elements are not lists");
            }
        } else {
            println!("Error: Not enough elements on the stack to perform append")
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Executes the tokens within a codeblock
    pub fn op_exec (stack: &mut Stack, tokens: &[&str]) {
        if let Some(V::VCodeBlock(code_block)) = stack.get(0).cloned(){ // Need to clone since we remove an element later
            stack.remove(0);
            
            let without_braces = &code_block[1..code_block.len()-1];    // Remove { }
            let tokens_code_block: Vec<&str> = without_braces.split_whitespace().collect(); 
            parser::process_tokens(&tokens_code_block, stack);  // Process the codeblock
        } else {
            println!("Error: Stack empty or top element not a codeblock");
        }
        parser::process_tokens(&tokens[1..], stack);
    }

    /// For each element in a list it executes a given code block
    pub fn op_each(stack: &mut Stack, tokens: &[&str]) {
        if stack.len() >= 2 {
            if let Some(V::VCodeBlock(code_block)) = stack.get(0) {
                // Clone the code block and split it into tokens without the {}
                let code_block_clone = code_block.clone(); 
                let code_block_no_braces = &code_block_clone[1..code_block_clone.len() - 1];
                let code_block_tokens: Vec<&str> = code_block_no_braces.split_whitespace().collect();

                if let V::VList(mut list) = stack[1].clone() {  // mutable reference to list
                    for i in 0..list.len() {
                        let mut dummy_stack: Vec<V> = vec![list[i].clone()]; // stack becomes the current list element
                        parser::process_tokens(&code_block_tokens, &mut dummy_stack); // codeblock executed for list element
                        if !dummy_stack.is_empty() {
                            list[i] = dummy_stack[0].clone();  // Insert the item where codeblock has been executed
                        }
                    }
                    stack.remove(0); // Remove the code block from the stack
                    stack[0] = V::VList(list); // Update the original list in the stack
                } else {
                    println!("Error: The first element on the stack is not a list!");
                }
            } else {
                println!("Error: The second element on the stack is not a codeblock!");
            }
        } else {
            println!("Error: Need at least two elements on the stack to perform each");
        }
        parser::process_tokens(tokens, stack);
    }

    /// Parse a string from stack to a integer or float and puts it back onto the stack
    pub fn op_parse_num(stack: &mut Stack, parse_float: bool, tokens: &[&str]) {
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
        parser::process_tokens(&tokens[1..], stack);
    }

    /// Adds a String, codeblock or list to the stack depending on the 'starting_symbol'
    pub fn op_enclosed(stack: &mut Stack, tokens: &[&str], starting_symbol: String, process_next: bool) {
        let mut new_string = String::new();            // {} and "" represented as a string
        let mut new_elements: Vec<V> = Vec::new();     // [] list represented as a vector 
        let mut index = 0;
        let initial_stack_len = stack.len();

        // Function only called for these three starting symbols, no need for handling exceptions
        let (start_char, end_char): (String, String) = match starting_symbol.as_str() {
            "\"" => ("\"".to_string(), "\"".to_string()),
            "[" => ("[".to_string(), "]".to_string()),
            "{" => ("{".to_string(), "}".to_string()),
            // This was just to satisfy the exhaustive pattern, function will never be called with any other symbol.
            _ => panic!("Invalid starting symbol"), 
        };
        
        let mut i = 1;
        while let Some(token) = tokens.get(i) {
            if *token == end_char {     // Token is the correct end_char                          
                index = i;              // Set the index to the last token
                break;
            } else {
                match start_char.as_str() {
                    "\"" => {                               // Just push token + space
                        new_string.push_str(token);
                        if i + 1 < tokens.len() && *tokens[i + 1] != end_char {     // Next token is not end_char
                            new_string.push(' ');
                        }
                        i += 1;
                    }
                    "{" => {
                        // TODO make the helper function work to remove duplicated code
                        if *token == "\"" || *token == "{" || *token == "[" {
                            let mut sub_tokens = Vec::new();   // Vec with tokens following initial ", { or [
                            sub_tokens.push(*token);
                            let mut j = i + 1;

                            let sub_end_char = match *token {   //Update new end char 
                                "\"" => "\"",
                                "{" => "}",
                                "[" => "]",
                                _ => panic!("Invalid starting symbol"), // will not reach this
                            };

                            // Goes through all the tokens untill it finds the closing " and adds them to the new stack
                            while let Some(token) = tokens.get(j) {
                                sub_tokens.push(token);
                                if token.contains(sub_end_char) {
                                    break;
                                }
                                j += 1;
                            }
                            let mut sub_stack: Stack = Vec::new();          // Dummy stack to send to op_enclosed
                            op_enclosed(&mut sub_stack, &sub_tokens, token.to_string(), true);
                            if let Some(value) = sub_stack.get(0) {         // Get the String, list or codeblock element from the sub stack
                                new_string.push_str(&format!("{} ", value.to_string()));
                            }
                            i = j + 1;
                        } else {
                            new_string.push_str(token);
                            if i + 1 < tokens.len() && *tokens[i + 1] != end_char {
                                new_string.push(' ');
                            }
                            i += 1;
                        }
                    }
                    "[" => {
                        // TODO make the helper function work to remove duplicated code
                        if *token == "\"" || *token == "{" || *token == "[" {
                            let mut sub_tokens = Vec::new();
                            sub_tokens.push(*token);
                            let mut j = i + 1;
                            let sub_end_char = match *token {   //Update new end char 
                                "\"" => "\"",
                                "{" => "}",
                                "[" => "]",
                                _ => panic!("Invalid starting symbol"), // will not reach this
                            };

                            while let Some(token) = tokens.get(j) {
                                sub_tokens.push(token);
                                if token.contains(sub_end_char) {
                                    break;
                                }
                                j += 1;
                            }
                            let mut sub_stack: Stack = Vec::new();
                            op_enclosed(&mut sub_stack, &sub_tokens, token.to_string(), true);
                            if let Some(value) = sub_stack.get(0) {
                                new_elements.push(value.clone());
                            }
                            i = j + 1;
                        } else {
                            match V::from_string(token) {
                                value => new_elements.push(value),
                            }
                            i += 1;
                        }
                    }
                    _ => {
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
        if process_next {
            parser::process_tokens(&tokens[index + 1..], stack);
        } 
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
        process_tokens(&tokens, stack);
    }
    
    /// Processes the tokens sent by process_input and handles the different type of tokens
    /// Calls itself recursively with the next token in the list until there are not tokens left
    pub fn process_tokens(tokens: &[&str], stack: &mut Stack) {
        if !tokens.is_empty() {
            match tokens[0] {
                "*" => interpreter::op_binary(stack, OpBinary::Multiply,  &tokens), 
                "+" => interpreter::op_binary(stack, OpBinary::Add,  &tokens), 
                "-" => interpreter::op_binary(stack, OpBinary::Subtract,  &tokens),
                "/" => interpreter::op_binary(stack, OpBinary::FDivide,  &tokens),
                "div" => interpreter::op_binary(stack, OpBinary::IDivide, &tokens),
                "<" => interpreter::op_binary(stack, OpBinary::RGreater, &tokens),
                ">" => interpreter::op_binary(stack, OpBinary::LGreater, &tokens),
                "==" => interpreter::op_binary(stack, OpBinary::Equality, &tokens),
                "&&" => interpreter::op_binary(stack, OpBinary::And, &tokens),
                "||" => interpreter::op_binary(stack, OpBinary::Or, &tokens),

                "not" => interpreter::op_not(stack, &tokens),
                "\"" => interpreter::op_enclosed(stack, &tokens, "\"".to_string(), true), 
                "[" => interpreter::op_enclosed(stack, &tokens, "[".to_string(), true), 
                "{" => interpreter::op_enclosed(stack, &tokens,"{".to_string(), true),

                "dup" => interpreter::op_dup(stack, &tokens),
                "swap" => interpreter::op_swap(stack, &tokens),

                "print" => interpreter::op_print(stack, &tokens),
                "read" => interpreter::op_read(stack, &tokens),
                
                "parseInteger" => interpreter::op_parse_num(stack, false, &tokens),
                "parseFloat" => interpreter::op_parse_num(stack, true, &tokens),
                "words" => interpreter::op_words(stack, &tokens),

                "head" => interpreter::op_head(stack, &tokens),
                "tail" => interpreter::op_tail(stack, &tokens),
                "empty" => interpreter::op_empty(stack, &tokens),
                "length" => interpreter::op_length(stack, &tokens),
                "cons" => interpreter::op_cons(stack, &tokens),
                "append" => interpreter::op_append(stack, &tokens),
                "each" => {
                    // Need to process codeblock first since syntax is [] each {}
                    if let Some(next_token) = tokens.get(1) {
                        if next_token.starts_with("{") {
                            // Adds the codeblock to the stack
                            interpreter::op_enclosed(stack, &tokens[1..], "{".to_string(), false);
                            // Finds the closing }, else the
                            if let Some(closing_brace_index) = tokens[1..].iter().position(|&x| x == "}") {
                                interpreter::op_each(stack, &tokens[closing_brace_index + 2..]);
                            } else {
                                // Should i let it process next tokens here?
                                println!("Error: Missing closing brace for the code block!");
                            }
                        } else {
                            // Should i let it process next tokens here? 
                            println!("Error: Next element is not a codeblock!");
                            process_tokens(&tokens[1..], stack);    // Processes the next token as usual
                        }
                    } else {
                        println!("Error: Needs to be a codeblock after each for it to work!");
                    }
                }
                "exec" => interpreter::op_exec(stack, &tokens),
                "pop" => {
                    interpreter::op_pop(stack);
                    process_tokens(&tokens[1..], stack);
                }
                _ => {
                     interpreter::op_num(stack, tokens[0]);
                     process_tokens(&tokens[1..], stack);
                    }
                _ => process_tokens(&tokens[1..], stack),
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