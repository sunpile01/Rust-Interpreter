
pub mod interpreter {
    use std::collections::HashMap;

    use super::parser::{self, process_tokens};

    use super::types::{Stack, WValue as V, OpBinary};

    /// Does the arithmetic operation sent as a parameter on the top two elements of the stack
    pub fn op_binary(stack: &mut Stack, op: OpBinary, tokens: &[&str],  var_and_fun: &mut HashMap<String, V>) {
        if stack.len() < 2 {
            println!("Error: The stack needs minimum two elements for binary operation!");
            return;
        } 
        let b = stack[0].clone();       // copy of the stack elements
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

            // Dividing by 0 is not allowed
            (V::VInt(a), V::VInt(b), OpBinary::IDivide) => {
                if b != 0 {
                    stack.insert(0, V::VInt(a / b))
                } else {
                    println!("Error: dividing by 0 is not allowed!");
                }
            }
            (V::VFloat(a), V::VFloat(b), OpBinary::IDivide) => {
                if b != 0.0 {
                    stack.insert(0, V::VInt(a as i64 / b as i64))
                } else {
                    println!("Error: dividing by 0 is not allowed!");
                }
            }
            (V::VFloat(a), V::VFloat(b), OpBinary::FDivide) => {
                if b != 0.0 {
                    stack.insert(0, V::VFloat(a / b))
                } else {
                    println!("Error: dividing by 0 is not allowed!");
                }
            }
            (V::VInt(a), V::VInt(b), OpBinary::FDivide) => {
                if b != 0 {
                    stack.insert(0, V::VFloat(a as f32 / b as f32))
                } else {
                    println!("Error: dividing by 0 is not allowed!");
                }
            }               
            (V::VInt(a), V::VInt(b), OpBinary::RGreater) => stack.insert(0, V::VBool(a < b)),
            (V::VFloat(a), V::VFloat(b), OpBinary::RGreater) => stack.insert(0, V::VBool(a < b)),

            (V::VInt(a), V::VInt(b), OpBinary::LGreater) => stack.insert(0, V::VBool(a > b)),
            (V::VFloat(a), V::VFloat(b), OpBinary::LGreater) => stack.insert(0, V::VBool(a > b)),

            (V::VInt(a), V::VInt(b), OpBinary::Equality) => stack.insert(0, V::VBool(a == b)),
            // Searched a bit online and found that comparing one float subtracted by another to epsilon is better
            // than using == which can cause problems
            (V::VFloat(a), V::VFloat(b), OpBinary::Equality) => stack.insert(0, V::VBool((a - b).abs() < f32::EPSILON)),
            (V::VString(a), V::VString(b), OpBinary::Equality) => stack.insert(0, V::VBool(a == b)),
            (V::VList(a), V::VList(b), OpBinary::Equality) => {stack.insert(0, V::VBool(a == b))}

            (V::VBool(a), V::VBool(b), OpBinary::Equality) => stack.insert(0, V::VBool(a == b)),
            (V::VBool(a), V::VBool(b), OpBinary::And) => stack.insert(0, V::VBool(a && b)),
            (V::VBool(a), V::VBool(b), OpBinary::Or) => stack.insert(0, V::VBool(a || b)),

            // Allowed operations where types do not fully match
            // Need both possible orders of the types, so double up everywhere
            (V::VInt(a), V::VFloat(b), OpBinary::Add) => stack.insert(0, V::VFloat(a as f32 + b)),
            (V::VFloat(a), V::VInt(b), OpBinary::Add) => stack.insert(0, V::VFloat(a + b as f32)),

            (V::VInt(a), V::VFloat(b), OpBinary::Multiply) => stack.insert(0, V::VFloat(a as f32 * b)),
            (V::VFloat(a), V::VInt(b), OpBinary::Multiply) => stack.insert(0, V::VFloat(a * b as f32)),

            (V::VInt(a), V::VFloat(b), OpBinary::IDivide) => stack.insert(0, V::VInt(a / b as i64)),
            (V::VFloat(a), V::VInt(b), OpBinary::IDivide) => stack.insert(0, V::VInt(a as i64 / b)),

            (V::VInt(a), V::VFloat(b), OpBinary::FDivide) => stack.insert(0, V::VFloat(a as f32 / b )),
            (V::VFloat(a), V::VInt(b), OpBinary::FDivide) => stack.insert(0, V::VFloat(a  / b as f32)),

            (V::VInt(a), V::VFloat(b), OpBinary::LGreater) => stack.insert(0, V::VBool(a as f32 > b )),
            (V::VFloat(a), V::VInt(b), OpBinary::LGreater) => stack.insert(0, V::VBool(a  > b as f32)),

            (V::VInt(a), V::VFloat(b), OpBinary::RGreater) => stack.insert(0, V::VBool((a as f32) < b  )),
            (V::VFloat(a), V::VInt(b), OpBinary::RGreater) => stack.insert(0, V::VBool(a < b as f32)),
            // For equality we take one minus the other and check if the result is bigger than a very small number
            (V::VInt(a), V::VFloat(b), OpBinary::Equality) => stack.insert(0, V::VBool((a as f32 - b).abs() < f32::EPSILON)),
            (V::VFloat(a), V::VInt(b), OpBinary::Equality) => stack.insert(0, V::VBool((a - b as f32).abs() < f32::EPSILON)),
            _ => {
                println!("The types of the top two elements are not compatible for {:?} operation.", op);
                succesfull = false;
            }
        }
        if succesfull{
            stack.remove(1);
            stack.remove(1);
        }
    
        parser::process_tokens(&tokens[1..], stack, var_and_fun);   // Process the remaining tokens 
    }   

    /// Popps an item of the stack
    pub fn op_pop(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if !stack.is_empty() {
            stack.remove(0);
        } else {
            panic!("Pop on empty stack not allowed!");  // Trying to pop on an empty stack should crash program
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Turns the token into a WValue using the from_string implemented on the WValue
    pub fn op_num(stack: &mut Stack, token: &str) {
        match V::from_string(token) {         
            value => {
                stack.insert(0, value);   
            }
        }
    }

    /// Duplicates the top element of the stack
    pub fn op_dup(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if stack.len() >= 1 {
            let top_element = stack[0].clone();     // Get a clone of the top element
            stack.insert(0, top_element);           // Insert it
        }
        else {
            println!("Error: No elemets on the stack to duplicate!");
        }

        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Swaps the order of the top two elements on the stack
    pub fn op_swap(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if stack.len() >= 2 {   
            stack.swap(0, 1);               
        }
        else {
            println!("Error: Need atleast two elements to perform swap!");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Prints the top element on the stack, works only for String types
    pub fn op_print(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if let Some(top_element) = stack.get(0).cloned() {  // Get top element, do not need to mutate it
            match top_element {
                V::VString(s) => {                      // Top element is of type String
                    println!("{}", s);
                    stack.remove(0);                    // Remove top element after printing it
                }
                _ => {
                    println!("Error: print not supported for other types than VString!");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to print!");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Reads user input and adds it to the stack as a VString
    pub fn op_read(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        use std::io;                                            // Only function that uses IO
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap().to_string(); // Get the input and store it in the input variable
        input = input.trim_end().to_string();                   // Remove \n and turn to string
        stack.insert(0, V::VString(format!("\"{}\"", input)));  // Add it to stack with ""
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    } 

    /// Gets a string element from the stack and seperates the words in the string into a vector
    /// and then inserts it into the stack as a WValue::VList (imported as V)
    pub fn op_words(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if let Some(top_element) = stack.get(0).cloned(){
            match top_element {
                V::VString(s) => {                      // Top element is of type String
                    let s_no_qoutes = &s[1..s.len() - 1];       // remove starting and ending " 
                    // Get all the words split by whitespace into a vector.
                    let string_tokens: Vec<&str> = s_no_qoutes.split_whitespace().collect();
                    let mut new_elements: Vec<V> = Vec::new();

                     // Go through all tokens and insert them into the vector, add " before and after
                    for token in string_tokens {
                        new_elements.push(V::VString(format!("\"{}\"", token)));
                    }
                    stack.remove(0);                            // Remove old string
                    stack.insert(0, V::VList(new_elements));    // Insert the list with the vector elements
                }
                _ => {
                    println!("Error: print not supported for other types than VString!");
                }
            }
        } else {
            println!("Error: Stack is empty, cant get the top element!");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);

    }

    /// Changes the top element to the not of it. For bools "true not" == "false" for numbers it negates them
    pub fn op_not(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if let Some(top_element) = stack.get_mut(0) {   // Get top element as mutable since we want to change it
            match top_element {
                V::VBool(b) => {       // Top element is a bool
                    *b = !*b;                 
                }
                V::VInt(i) => {       // Top element is an Int
                    *i = -*i;                 
                }
                V::VFloat(f) => {    // Top element is a float
                    *f = -*f;                 
                }
                _ => {
                    println!("Error: print not supported for other types than VString!");
                }
            }
        } else {
            println!("Error: stack is empty, no top element to perform logical not on!")
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Returns the first element from a list if the top stack element is a list
    pub fn op_head(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if let Some(V::VList(list)) = stack.get(0).cloned(){ // if top element exists and is a VList
            if !list.is_empty() {     
                stack.insert(0, list[0].clone()); // need to clone bcause WValue does not implement the Copy trait              
            } else {
                println!("Error: List is empty, cant return the head element");
            }
        }
        else {
            println!("Error: Stack empty or top element is not a list");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Returns the last element from a list if the first stack element is a list
    pub fn op_tail(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if let Some(V::VList(list)) = stack.get(0).cloned() {    // if top element exists and is a VList
            stack.remove (0); 
            if !list.is_empty() {
                let mut new_list: Vec<V> = Vec::new();

                for list_element in &list[1..]{                 // Push allt he tail elements to the new list
                    new_list.push(list_element.clone());        // Need to push cloned element since WValue (V)
                }                                               // Does not implement the Coopy trade so can not be moved
                stack.insert(0, V::VList(new_list));
            } else {
                println!("Error: List is empty, cant return an element");
            }
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Checks if the top element on the stack is a list. If it is a list and it is empty,
    ///  it inserts false into the stack, if it is not empty it inserts true.
    pub fn op_empty(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if let Some(V::VList(list)) = stack.get(0){
            stack.insert(0, V::VBool(list.is_empty()));
        } else {
            println!("Error: Stack empty or top element not a list");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Checks if the top element on the stack is a list, codeblock or string. 
    /// If it is, it inserts the length on the top of the stack and removes the list, codeblock or string.
    pub fn op_length(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if let Some (top_element) = stack.get(0).cloned(){  // Need cloned to avoid borrow issues below
            match top_element {                             // Find the type of the top element
                V::VList(list) => {
                    stack.remove(0);
                    stack.insert(0, V::VInt(list.len() as i64))
                }
                V::VCodeBlock(c) => {
                    stack.remove(0);
                    let code_block_no_braces =&c[1..c.len() - 1];   // Remove the { and }
                    // Need to get the tokens from the codeblock since it is a string
                    let code_block_tokens: Vec<&str> = code_block_no_braces.split_whitespace().collect();
                    stack.insert(0, V::VInt(code_block_tokens.len() as i64))
                }
                V::VString(s) =>{ 
                    stack.remove(0);
                    stack.insert(0, V::VInt(s.len() as i64 - 2))  // -2 for space before and after "
                } 
                _ => println!("Error: Type not allowed for operation length"),
            }
        } else {
            println!("Error: stack is empty cant get length of top element");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Append an element to the front of a list. Not sure about how the order should be done here
    /// Works in this order literal list cons
    pub fn op_cons (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if stack.len() < 2 {
            println!("Error: Not enough elements to perform cons operation!");
            return;
        } 
        let literal = stack.remove(1);                          // remove literal from stack      
        if let Some(V::VList(list)) = stack.get_mut(0) {        // top element is a list
            list.insert(0, literal);
        } else {
            println!("Error: Second element is not a list!");
            stack.insert(1, literal);                          // Reinsert the item we removed.
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Appends one list to the end of another list.
    /// [ 1 2 ] [ 3 4 ] append, results in [ 3 4 ] being appended to [ 1 2 ] result: [ 1 2 3 4 ]
    pub fn op_append (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if stack.len() < 2 {
            println!("Error: Not enough elements to perform append operation!");
            return;
        } 
        if let Some(V::VList(list2)) = stack.get(0){    // Get the list to be appended to the other
            let list2_clone = list2.clone();
            stack.remove(0);                                    // Remove list 1
            if let Some(V::VList(list1)) = stack.get_mut(0) {   // Now we can get as mutable reference 
                list1.extend(list2_clone);                      // Extends list1 with list2
            } else {
                println!("Error: the first element needs to be a list");
            }
        } else {
            println!("Error: Top two elements are not lists");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Executes the tokens within a codeblock
    pub fn op_exec (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if let Some(V::VCodeBlock(code_block)) = stack.get(0).cloned(){ 
            stack.remove(0);
            // Convert the codeblock to tokens that can be processed 
            let code_block_tokens: Vec<&str> = parse_code_block_tokens(&code_block);
            parser::process_tokens(&code_block_tokens, stack, var_and_fun);  // Process the codeblock
        } else {
            println!("Error: Stack empty or top element not a codeblock");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// For each element in a list it executes a given code block. if is_map is 1 the original list is updated 
    /// and we have done a map, if it is 0 then the original list is removed and we performed each.
    pub fn op_map_or_each(stack: &mut Stack, tokens: &[&str], is_map: i64, var_and_fun: &mut HashMap<String, V>) {
        let mut process_next = false;
        if stack.len() < 2 {
            println!("Error: Not enough elements to perform map or each operation!");
            return;
        } 
            
        let valid_or_cblock = &stack.get(0).cloned();      // allows { print } and print

        if let V::VList(mut list) = stack[1].clone() {
            stack.remove(0); // Remove the code block from the stack
            if is_map == 0 {
                stack.remove(0); // Remove the original list from the stack
            }
            // Goes through the elements in the list 
            for i in 0..list.len() {
                let mut dummy_stack: Vec<V> = Vec::new();
                dummy_stack = vec![list[i].clone()];    // stack becomes the current list element

                // Tried to make the match above the loop, but got some issues and did not have time to solve it
                match valid_or_cblock {
                    Some(V::VCodeBlock(code_block)) => {
                        let code_block_tokens: Vec<&str> = parse_code_block_tokens(code_block);
                        parser::process_tokens(&code_block_tokens, &mut dummy_stack, var_and_fun); // codeblock executed for list element
                    }
                    Some(operation) if  parser::is_valid_element_each_map(operation.to_string().as_str()) => { 
                        process_next = true;
                        parser::process_tokens(&[operation.to_string().as_str()], &mut dummy_stack, var_and_fun); // Valid symbol executed for list element
                    }
                    _ => {
                        if !process_next{   // Only needs to be printed once so can use process_next for this
                        println!("Element after map/each is not a codeblock or valid operation!
                                        Only unary operations are allowed for map and each");
                        }
                    }
                }
                if !dummy_stack.is_empty() {
                    if is_map == 0 {
                        stack.insert(0, dummy_stack[0].clone()); // Insert the result of the codeblock execution on the list element to the stack
                    } else {
                        list[i] = dummy_stack[0].clone();  // Insert the item where codeblock has been executed
                    }
                }
            }
            if is_map == 1 {
                stack[0] = V::VList(list); // Update the original list in the stack
            }
        } else {
            println!("Error: The first element on the stack is not a list!");
        }
        parser::process_tokens(&tokens[if process_next { 1 } else { 0 }..], stack, var_and_fun);
    }

    /// Uses a list, starting value and code block tofor example sum up a list:
    /// [1 5 9 20 ] 0 foldl { + }  on an empty stack will result in the stack: [35]
    pub fn op_foldl(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        let mut process_next = false;
        if stack.len() < 3 {
            println!("Error: Not enough codeblocks provided to perform a fodl!");
            return;
        } 
        if let (Some(V::VList(list)), Some(V::VInt(init_acc))) = (stack.get(2), stack.get(1)) {
            let mut acc = *init_acc;

            // Check if the top element is a valid operation or a code block
            match stack.get(0) {
                Some(V::VCodeBlock(code_block)) => {
                    acc = process_list_for_foldl(acc, list, var_and_fun, Some(code_block), None);
                }
                Some(operation) if parser::is_valid_element(operation.to_string().as_str()) => {
                    process_next = true;
                    acc = process_list_for_foldl(acc, list, var_and_fun, None, Some(operation));
                }
                _ => {
                    println!("Error: Top element on stack must be a code block or a valid operation");
                    return;
                }
            }
            if acc != 0 {        // The process_list_for_foldl did not fail
                stack.remove(0); // Remove the code block or valid operation from the stack
                stack.remove(0); // Remove the initial accumulator from the stack
                stack.remove(0); // Remove the list from the stack
                stack.insert(0, V::VInt(acc)); // Insert the accumulated value to the stack
            } else {
                stack.remove(0);
                stack.remove(0);
                stack.remove(0);
                println!("Error: There was an invalid element in the list!");
            }
        } else {
            // Should i remove elements here?
            println!("Error: Types entered for fold operation were not correct");
        }

        // If it was a codeblock, the index for the next token is already sent by op_infix,
        // so we dont have to send the index for the next token
        parser::process_tokens(&tokens[if process_next { 1 } else { 0 }..], stack, var_and_fun);
    }


    /// syntax: Condition if Then Else. Gets the condition and then else block from the stack.
    /// If condition is true executes the Then block, otherwise it executes the Else block
    pub fn op_if (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if stack.len() < 3 {
            println!("Error: Not enough codeblocks provided to perform if operation!");
            return;
        } 

        // Cant specify the types of the then_block and else_block since it needs to be matched later
        if let (Some(V::VBool(condition)), Some(then_block), Some(else_block))  = 
                                    (stack.get(2).cloned(), stack.get(1).cloned(), stack.get(0).cloned()) {                         
            stack.remove(0);
            stack.remove(0);        // remove elements on the stack used for if
            stack.remove(0);

            let block_to_use = if condition { then_block } else { else_block };
            
            // Check if it is a valid single operation or a codeblock
            match block_to_use {
                V::VCodeBlock(code_block) => {
                    let code_block_tokens = parse_code_block_tokens(&code_block);   // Parse the codeblock
                    parser::process_tokens(&code_block_tokens, stack, var_and_fun);
                }
                // Check if it is a valid element before processing the operation
                operation if parser::is_valid_element(operation.to_string().as_str()) => {
                    parser::process_tokens(&[operation.to_string().as_str()], stack, var_and_fun);
                }
                _ => {
                    println!("Error: Then and Else inputs must be a code block or a valid operation");
                }
            }
        } else {
            println!("Error: Wrong types provided for if statement!");
            stack.remove(0);
            stack.remove(0);
            stack.remove(0);
        }

        parser::process_tokens(&tokens, stack, var_and_fun);
        
    }

    /// Syntax : loop break block. Executes the block until break becomes true
    /// where both break and block must be codeblocks 
    pub fn op_loop(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if stack.len() < 2 {
            println!("Error: Not enough codeblocks provided to perform a loop!");
            return;
        } 
        // Make sure the top two elements are codeblocks
        if let (Some(V::VCodeBlock(break_cond)), Some (V::VCodeBlock(code_block))) = 
                                                                (stack.get(1).cloned(), stack.get(0).cloned()){
            stack.remove(0);
            stack.remove(0);

            loop {
                // Parse the break_cond into tokens and create a dumy stack
                let break_cond_tokens = parse_code_block_tokens(&break_cond);
                let mut break_stack = Stack::new();
                break_stack.push(stack[0].clone()); // Push the current value from the main stack onto the break_stack
                process_tokens(&break_cond_tokens, &mut break_stack, var_and_fun);

                // Check if the break condition is true
                if let Some(V::VBool(true)) = break_stack.get(0) {
                    break;
                }
    
                // Parse and execute the codeblock on the main stack
                let code_block_tokens = parse_code_block_tokens(&code_block);
                process_tokens(&code_block_tokens, stack, var_and_fun);
            }

        } else {
            println!("Error: Both types after loop must be codeblocks!");
        }
        process_tokens(&tokens, stack, var_and_fun);
    }

    /// Execute a code block x number of times
    /// For example 5 times { 10 10 + } will result in the stack [20, 20, 20, 20, 20]
    pub fn op_times (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        let mut process_next = false;
        if stack.len() < 2 {
            println!("Error: Not enough elements to perform times operation!");
            return;
        } 
        // Get the integer and valid operation (either a codeblock { + } or single operation + )
        if let (Some(V::VInt(num_times)), Some(valid_op)) = (stack.get(1).cloned(), stack.get(0).cloned()) {
            stack.remove(0);    // Remove elements used for times operation
            stack.remove(0);

            match valid_op {                        // Either codeblock or valid element
                V::VCodeBlock(code_block) => {
                    let code_block_tokens = parse_code_block_tokens(&code_block);
                    // Execute the codeblock num_times
                    for _ in 0..num_times {
                        parser::process_tokens(&code_block_tokens, stack, var_and_fun);
                    }
                }
                operation if parser::is_valid_element(operation.to_string().as_str()) => {
                    process_next = true;
                    // Execute the valid operation num_times
                    for _ in 0..num_times {
                        parser::process_tokens(&[operation.to_string().as_str()], stack, var_and_fun);
                    }
                }
                _ => {
                    println!("Error: The input after 'times' needs to be a codeblock or a valid operation!");
                }
            }      
        } else {
            println!("Error: Not correct types for the times operation! (Int, codeblock/valid operation)");
        }
        parser::process_tokens(&tokens[if process_next { 1 } else { 0 }..], stack, var_and_fun);
    }
    
    /// Assigne a name to a literal for example: myName " Aleksander " := . Assigns myName to Aleksander
    /// So when doing myName later Aleksander will be pushed to the stack.  
    pub fn op_assign_variable (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if stack.len() < 2 {
            println!("Error: Not enough elements to perform assignment operation!");
            return;
        } 
        let value = stack.remove(0);
        // Make sure it is not another valid type that has defined behavior, we use WValue::VOther for this
        if let Some(V::VOther(key)) = stack.get(0).cloned() {
            stack.remove(0);
            var_and_fun.insert(key, value);         // Insert the key with the value to the hashmap
        } else {
            println!("Error: The second element on the stack must be a VOther value for the key");
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Same as op_assign_variable but with functions instead and different arguments.  
    /// Syntax: VOther codeblock fun    
    pub fn op_assign_function (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>){
        if stack.len() < 2 {
            println!("Error: Not enough elements to perform assignment of functio!");
            return;
        } 
        if let Some(code_block) = stack.get(0).cloned(){
            stack.remove(0);
            if let Some(V::VOther(key)) = stack.get(0).cloned() {
                var_and_fun.insert(key, code_block);
                stack.remove(0);
            } else {
                println!("Error: The second element on the stack must be a VOther value for the key!");
            }
        } else {
            println!("Error: The first element on the stack must be a codeblock!")
        }
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }

    /// Parse a string from stack to a integer or float and puts it back onto the stack
    pub fn op_parse_num(stack: &mut Stack, parse_float: bool, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) {
        if let Some(top_element) = stack.get_mut(0) {
            match top_element {
                V::VString(s) => {
                    let s_no_qoutes = &s[1..s.len() - 1];   // Remove the surrounding qoutes
                    if parse_float {
                        // Try to parse to f32
                        if let Ok(f) = s_no_qoutes.parse::<f32>() {
                            *top_element = V::VFloat(f); // Replace the VString with the parsed VFloat
                        } else {
                            println!("Error: Unable to parse string to float!");
                        }
                    } else {
                        // Try to parse to i64
                        if let Ok(i) = s_no_qoutes.parse::<i64>() {
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
        parser::process_tokens(&tokens[1..], stack, var_and_fun);
    }


    /// Adds a String, codeblock or list to the stack depending on the 'starting_symbol'
    pub fn op_enclosed(stack: &mut Stack, tokens: &[&str], starting_symbol: String, process_next: bool, var_and_fun: &mut HashMap<String, V>) {
        let mut new_string = String::new();            // {} and "" represented as a string
        let mut new_elements: Vec<V> = Vec::new();     // [] list represented as a vector 
        let mut index = 0;
        let mut handle_next = true;

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
                            op_enclosed(&mut sub_stack, &sub_tokens, token.to_string(), true, var_and_fun);
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
                            op_enclosed(&mut sub_stack, &sub_tokens, token.to_string(), true, var_and_fun);
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
                "}"  => stack.insert(0, V::VCodeBlock(format!("{} {} {}", start_char, new_string, end_char))),
                _ => panic!("Invalid starting symbol"), // Just to satisfy exhaustive enforcement
            }
        } else {
        println!("Error: Missing closing quote");
            parser::process_tokens(&tokens[tokens.len()..], stack, var_and_fun); // Dont process next tokens
            handle_next = false;
        }
        if process_next && handle_next {
            parser::process_tokens(&tokens[index + 1..], stack, var_and_fun);
        } 
    }

    /// TODO DOES NOT HANDLE STRINGS INSIDE CODEBLOCKS CORRECTLY
    /// Helper function to parse the codeblock into a vector of tokens
    fn parse_code_block_tokens(code_block: &str) -> Vec<&str> {
        let code_block_no_braces = &code_block[1..code_block.len() - 1];
        let code_block_tokens: Vec<&str> = code_block_no_braces.split_whitespace().collect();
        code_block_tokens
    }
    /// Does foldl on the list sent as a parameter, returns the acummulated value to be placed on the stack
    fn process_list_for_foldl(acc: i64, list: &[V],
        var_and_fun: &mut HashMap<String, V>, code_block: Option<&str>, operation: Option<&V>) -> i64 {
        let mut accumulator = acc;
        
        // Go through each list element
        for elem in list {
            // Create a dummy stack with the list element and the accumulated value 
            let mut dummy_stack: Vec<V> = vec![elem.clone(), V::VInt(accumulator)];

            // Process either the code_block or operation on the dummy stack
            if let Some(code_block) = code_block {
                let code_block_tokens: Vec<&str> = parse_code_block_tokens(code_block);
                parser::process_tokens(&code_block_tokens, &mut dummy_stack, var_and_fun);
            } else if let Some(operation) = operation { // Uses else if to unwrapt he operation first
                parser::process_tokens(&[operation.to_string().as_str()], &mut dummy_stack, var_and_fun);
            }

            // Update the accumulator with the new accumulated value 
            if let Some(V::VInt(new_acc)) = dummy_stack.get(0) {
                accumulator = *new_acc;
            } else {
                println!("Error: An element in the list was not of integer type");
                return 0;
            }
        }
        accumulator
    }

}


pub mod parser {

    use std::collections::HashMap;

    use super::interpreter;
    use super::types::{Stack, OpBinary, WValue as V};
    
    /// Parses the string input into "tokens" for example *, +, then calls the process_tokens function to 
    /// execute the corresponding code depending on the tokens. 
    pub fn process_input(line: &str, stack: &mut Stack, var_and_fun: &mut HashMap<String, V>) {
        let tokens = line.split_whitespace().collect::<Vec<&str>>(); // Get all symbols separated by space
        // Process the token inputs
        process_tokens(&tokens, stack, var_and_fun);
    }
    
    /// Processes the tokens sent by process_input and handles the different type of tokens
    /// This function is called by each operation after executing the operation.
    pub fn process_tokens(tokens: &[&str], stack: &mut Stack, var_and_fun: &mut HashMap<String, V>) {
        if !tokens.is_empty() {
            // Do operation according to what the top token is. 
            match tokens[0] {
                "*" => interpreter::op_binary(stack, OpBinary::Multiply,  &tokens, var_and_fun), 
                "+" => interpreter::op_binary(stack, OpBinary::Add,  &tokens, var_and_fun), 
                "-" => interpreter::op_binary(stack, OpBinary::Subtract,  &tokens, var_and_fun),
                "/" => interpreter::op_binary(stack, OpBinary::FDivide,  &tokens, var_and_fun),
                "div" => interpreter::op_binary(stack, OpBinary::IDivide, &tokens, var_and_fun),
                "<" => interpreter::op_binary(stack, OpBinary::RGreater, &tokens, var_and_fun),
                ">" => interpreter::op_binary(stack, OpBinary::LGreater, &tokens, var_and_fun),
                "==" => interpreter::op_binary(stack, OpBinary::Equality, &tokens,var_and_fun),
                "&&" => interpreter::op_binary(stack, OpBinary::And, &tokens, var_and_fun),
                "||" => interpreter::op_binary(stack, OpBinary::Or, &tokens,var_and_fun),

                "not" => interpreter::op_not(stack, &tokens, var_and_fun),
                "\"" => interpreter::op_enclosed(stack, &tokens, "\"".to_string(), true, var_and_fun), 
                "[" => interpreter::op_enclosed(stack, &tokens, "[".to_string(), true, var_and_fun), 
                "{" => interpreter::op_enclosed(stack, &tokens,"{".to_string(), true, var_and_fun),

                "dup" => interpreter::op_dup(stack, &tokens, var_and_fun),
                "swap" => interpreter::op_swap(stack, &tokens, var_and_fun),

                "print" => interpreter::op_print(stack, &tokens, var_and_fun),
                "read" => interpreter::op_read(stack, &tokens, var_and_fun),
                
                "parseInteger" => interpreter::op_parse_num(stack, false, &tokens, var_and_fun),
                "parseFloat" => interpreter::op_parse_num(stack, true, &tokens, var_and_fun),
                "words" => interpreter::op_words(stack, &tokens, var_and_fun),

                "head" => interpreter::op_head(stack, &tokens, var_and_fun),
                "tail" => interpreter::op_tail(stack, &tokens, var_and_fun),
                "empty" => interpreter::op_empty(stack, &tokens, var_and_fun),
                "length" => interpreter::op_length(stack, &tokens, var_and_fun),
                "cons" => interpreter::op_cons(stack, &tokens, var_and_fun),
                "append" => interpreter::op_append(stack, &tokens, var_and_fun),
                "each" => infix_op(stack, &tokens, 0, var_and_fun),
                "map" => infix_op(stack, &tokens, 1, var_and_fun),
                "foldl" => infix_op(stack, &tokens, 2, var_and_fun),
                "if" => infix_op(stack, &tokens, 3, var_and_fun),
                "times" => infix_op(stack, &tokens, 4, var_and_fun),
                "loop" => infix_op(stack, &tokens, 5, var_and_fun),

                ":=" => interpreter::op_assign_variable(stack, &tokens, var_and_fun),
                "fun" => interpreter::op_assign_function(stack, &tokens, var_and_fun),
                "exec" => interpreter::op_exec(stack, &tokens, var_and_fun),
                "pop" => interpreter::op_pop(stack, &tokens, var_and_fun),
                _ => {
                    // Check if the token is already in the hashmap
                    if var_and_fun.contains_key(tokens[0]) {
                        // Retrieve the value of the variable and push it onto the stack
                        let value = var_and_fun.get(tokens[0]).unwrap().clone();
                        let value_clone = value.clone();    // Because we move value out of scope underneath
                        stack.insert(0, value);
                        if let V::VCodeBlock(_) = value_clone {
                            // Call process_tokens with "exec" as the token to execute the codeblock that was 
                            // Put onto the stack
                            process_tokens(&["exec"], stack, var_and_fun);
                        }
                        process_tokens(&tokens[1..], stack, var_and_fun)
                    }
                    else{
                        interpreter::op_num(stack, tokens[0]);
                        process_tokens(&tokens[1..], stack, var_and_fun);
                    }
                }
            };
        }
    }
    
    /// NEED TO FIX THE NESTING LVELS HERE REALLY UGLY AS OF NOW, BUT WORKS
    fn infix_op (stack: &mut Stack, tokens: &[&str], operation_type: i64, var_and_fun: &mut HashMap<String, V>){
        // Need to process codeblock first since syntax is [] each {}
        if let Some(next_token) = tokens.get(1) {

            if next_token.starts_with("{") {
                // Adds the codeblock to the stack
                interpreter::op_enclosed(stack, &tokens[1..], "{".to_string(), false, var_and_fun);
                // Finds the index for the closing } in the tokens array
                if let Some(closing_brace_index) = find_matching_brace(&tokens[1..]) {
                    // Calls the correct function depending on what operation is supposed to be exectued
                    // Check function above for reference
                    match operation_type {
                        0 => interpreter::op_map_or_each(stack, &tokens[closing_brace_index + 2..], 0, var_and_fun),
                        1 => interpreter::op_map_or_each(stack, &tokens[closing_brace_index + 2..], 1, var_and_fun),
                        2 => interpreter::op_foldl(stack, &tokens[closing_brace_index + 2..], var_and_fun),
                        3 | 5=> {
                            // Need to take the second codeblock into consideration aswell for if statement
                            if let Some(second_token) = tokens.get(closing_brace_index + 2) {
                                //Checks
                                if second_token.starts_with("{"){
                                    interpreter::op_enclosed(stack, &tokens[closing_brace_index + 2..], "{".to_string(), false, var_and_fun);
                                    if let Some (second_closing_brace_index) = find_matching_brace(&tokens[closing_brace_index +2..]){
                                        if operation_type == 3 {
                                            interpreter::op_if(stack, 
                                                &tokens[closing_brace_index + second_closing_brace_index + 3..], var_and_fun);
                                        } else {
                                            interpreter::op_loop(stack, 
                                                &tokens[closing_brace_index + second_closing_brace_index + 3..], var_and_fun);
                                        }
                                    } else {
                                        println!("Error: Missing closing brace for the last codeblock following if ");
                                    }
                                } else if is_valid_element(second_token){
                                    stack.insert(0, V::VString(second_token.clone().to_string()));
                                    interpreter::op_if(stack, 
                                        &tokens[closing_brace_index + 3..], var_and_fun);
                                } else {
                                    println!("Error: Second element after if is not a codeblock or valid operation");
                                }
                            }  else {
                                println!("Error: Needs to be two elements after if!")
                            }
                        }
                        4 => interpreter::op_times(stack, &tokens[closing_brace_index + 2..], var_and_fun),
                        _ => {} // Should never be called with anything other than 0,1,2,3
                    }
                } else {
                    // Should i let it process next tokens here?
                    println!("Error: Missing closing brace for the first code block!");
                }
            } else if is_valid_element(next_token) {
                stack.insert(0, V::VString(next_token.clone().to_string()));
                match operation_type {
                    0 => interpreter::op_map_or_each(stack, &tokens[1..], 0, var_and_fun),
                    1 => interpreter::op_map_or_each(stack, &tokens[1..], 1, var_and_fun),
                    2 => interpreter::op_foldl(stack, &tokens[1..], var_and_fun),
                    3 => {
                        if let Some(second_token) = tokens.get(2) {
                            if second_token.starts_with("{"){
                                // Adds the codeblock to the stack
                                println!("tokens first time: {:?}", tokens);
                                interpreter::op_enclosed(stack, &tokens[2..], "{".to_string(), false, var_and_fun);
                                // Finds the index for the closing } in the tokens array
                                if let Some (second_closing_brace_index) = find_matching_brace(&tokens[2..]){
                                    interpreter::op_if(stack, 
                                                    &tokens[second_closing_brace_index + 3..], var_and_fun);
                                } else {
                                    println!("Error: Missing closing brace for the last codeblock following if ");
                                }
                            } else if is_valid_element(second_token) {
                                stack.insert(0, V::VString(second_token.clone().to_string()));
                                    interpreter::op_if(stack, &tokens[3..], var_and_fun);
                            }
                            else {
                                println!("Error: Second element not vlalid for if statement!");
                            }
                        }
                    }
                    4 => interpreter::op_times(stack, &tokens[1..], var_and_fun),
                    _ => {} // Should never be called with anything other than 0,1,2,3
                }
            } else {
                println!("does not enter here");
                // Should i let it process next tokens here? 
                println!("Error: Next element is not a codeblock or valid operation!");
                process_tokens(&tokens[1..], stack, var_and_fun);    // Processes the next token as usual
            }
        } else {
            println!("Error: Needs to be a token after ");
        }
    }

    // Helper function to find the mathing end brace for the brace index it is sent as a parameter
    fn find_matching_brace(tokens: &[&str]) -> Option<usize> {
        let mut open_braces = 0;
        // Go throught the remaining tokens
        for (i, token) in tokens.iter().enumerate() {
            if *token == "{" {
                open_braces += 1;       // Now needs to find and extra closing brace
            } else if *token == "}" {
                if open_braces == 1 {   
                    return Some(i);    // Found the corresponding closing brace, return from the function
                } else {
                    open_braces -= 1;
                }
            }
        }
        // Did not find a corresponding closing brace
        None   // Always called with "let Some (..) = find_matching_brace()" so the Some will handle the error
    }
    
    /// Checks if a element is valid for use instead of a { } for operations that require { }
    pub fn is_valid_element(element: &str) -> bool {
        match element {
            "head" | "tail" | "empty" | "length" | "cons" | "append" | "+" | "div" | "dup" |
            "swap" | "pop" | "parseInteger" | "print" | "read" | "parseFloat" | "words" | "-" | "*" |
            "/" | "<" | ">" | "==" => true,
            _ => {
                // Check if the string can be parsed as an integer or a float
                element.parse::<i64>().is_ok() || element.parse::<f32>().is_ok()
            }
        }
    }
    // Each and map is on a single element of the list only so some single operations can not be done
    // Such as binary operations. 
    pub fn is_valid_element_each_map(element: &str) -> bool {
        match element {
            "head" | "tail" | "empty" | "length" | "dup" | "pop" | "parseInteger" | "print" | "read" | 
            "parseFloat" | "words" => true,
            _ => {
                // Check if the string can be parsed as an integer or a float
                element.parse::<i64>().is_ok() || element.parse::<f32>().is_ok()
            }
        }
    }

}


pub mod types {
    use core::fmt;
    use std::str::FromStr;
    use ryu;


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
    #[derive(Clone, Debug, PartialEq)]  // PartialEq so we can compare to lists, debug for printing 
    pub enum WValue {
        VInt (i64),
        VFloat (f32),
        VBool (bool),
        VString (String),
        VList (Vec<WValue>),
        VCodeBlock (String),    // Maybe should have been a vector but hard to change this late
        VOther (String)
    }
    
    // To display the wrapped types as strings
    impl fmt::Display for WValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                WValue::VInt(n) => write!(f, "{}", n),
                WValue::VFloat(fl) => {
                    
                    let mut buf = ryu::Buffer::new(); // Searched online and found the crate ryu converting to
                    write!(f, "{}", buf.format(*fl)) // converting to floating points with correct Decimal values
                }
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
            if let Ok(num) = i64::from_str(s) {
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

// integration testing. Function is called in the integration_tests.rs file located in the test folder
pub fn t(input: &str) -> String {
    use parser::process_input;
    use std::collections::HashMap;
    use types::{WValue as V};

    // Create the new stack and hashmap
    let mut stack = types::Stack::new();                        
    let mut var_and_fun: HashMap<String, V> = HashMap::new();  

    // process the input with the input sent as a parameter on the new stack and hashmap
    process_input(input, &mut stack, &mut var_and_fun);

    // get and return the output
    let output: String = stack[0].to_string();  
    output
}
