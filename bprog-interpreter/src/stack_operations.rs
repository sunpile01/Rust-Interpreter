
use std::collections::HashMap;

use crate::parser::{self, process_tokens};

use crate::types::{Stack, WValue as V, OpBinary, ParseError};

/// Does the arithmetic operation sent as a parameter on the top two elements of the stack
pub fn op_binary(stack: &mut Stack, op: OpBinary, tokens: &[&str],  var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements);
    } 
    let b = stack[0].clone();       // copy of the stack elements
    let a = stack[1].clone();    
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
                return Err(ParseError::DivisionByZero)
            }
        }
        (V::VFloat(a), V::VFloat(b), OpBinary::IDivide) => {
            if b != 0.0 {
                stack.insert(0, V::VInt(a as i64 / b as i64))
            } else {
                return Err(ParseError::DivisionByZero)
            }
        }
        (V::VFloat(a), V::VFloat(b), OpBinary::FDivide) => {
            if b != 0.0 {
                stack.insert(0, V::VFloat(a / b))
            } else {
                return Err(ParseError::DivisionByZero)
            }
        }
        (V::VInt(a), V::VInt(b), OpBinary::FDivide) => {
            if b != 0 {
                stack.insert(0, V::VFloat(a as f32 / b as f32))
            } else {
                return Err(ParseError::DivisionByZero)
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
            return Err(ParseError::NonCompatibleTypes)
        }
    }
    stack.remove(1);
    stack.remove(1);

    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())  // Process the remaining tokens 
}   

/// Popps an item of the stack
pub fn op_pop(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if !stack.is_empty() {
        stack.remove(0);
    } else {
        panic!("Pop on empty stack not allowed!");  // Trying to pop on an empty stack should crash program
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
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
pub fn op_dup(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if stack.len() >= 1 {
        let top_element = stack[0].clone();     // Get a clone of the top element
        stack.insert(0, top_element);           // Insert it
    }
    else {
        return Err(ParseError::StackEmpty)
    }

    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Swaps the order of the top two elements on the stack
pub fn op_swap(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if stack.len() >= 2 {   
        stack.swap(0, 1);               
    }
    else {
        return Err(ParseError::NotEnoughElements)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Prints the top element on the stack, works only for String types
pub fn op_print(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if let Some(top_element) = stack.get(0).cloned() {  // Get top element, do not need to mutate it
        match top_element {
            V::VString(s) => {                      // Top element is of type String
                stack.remove(0);                    // Remove top element after printing it
            }
            _ => {
                return Err(ParseError::ExpectedString)
            }
        }
    } else {
        return Err(ParseError::StackEmpty)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Reads user input and adds it to the stack as a VString
pub fn op_read(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    use std::io;                                            // Only function that uses IO
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap().to_string(); // Get the input and store it in the input variable
    input = input.trim_end().to_string();                   // Remove \n and turn to string
    stack.insert(0, V::VString(format!("\"{}\"", input)));  // Add it to stack with ""
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
} 

/// Gets a string element from the stack and seperates the words in the string into a vector
/// and then inserts it into the stack as a WValue::VList (imported as V)
pub fn op_words(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
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
                return Err(ParseError::ExpectedString)
            }
        }
    } else {
        return Err(ParseError::StackEmpty)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())

}

/// Changes the top element to the not of it. For bools "true not" == "false" for numbers it negates them
pub fn op_not(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
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
                return Err(ParseError::ExpectedBoolOrNumber)
            }
        }
    } else {
        return Err(ParseError::StackEmpty)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Returns the first element from a list if the top stack element is a list
pub fn op_head(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if let Some(V::VList(list)) = stack.get(0).cloned(){ // if top element exists and is a VList
        if !list.is_empty() {     
            stack.insert(0, list[0].clone()); // need to clone bcause WValue does not implement the Copy trait              
        } else {
            return Err(ParseError::ListEmpty)
        }
    }
    else {
        return Err(ParseError::ExpectedList)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Returns the last element from a list if the first stack element is a list
pub fn op_tail(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if let Some(V::VList(list)) = stack.get(0).cloned() {    // if top element exists and is a VList
        if !list.is_empty() {
            let mut new_list: Vec<V> = Vec::new();

            for list_element in &list[1..]{                 // Push allt he tail elements to the new list
                new_list.push(list_element.clone());        // Need to push cloned element since WValue (V)
            }                                               // Does not implement the Coopy trade so can not be moved
            stack.insert(0, V::VList(new_list));
        } else {
            return Err(ParseError::ListEmpty)
        }
    } else {
        return Err(ParseError::ExpectedList)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Checks if the top element on the stack is a list. If it is a list and it is empty,
///  it inserts false into the stack, if it is not empty it inserts true.
pub fn op_empty(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if let Some(V::VList(list)) = stack.get(0){
        stack.insert(0, V::VBool(list.is_empty()));
    } else {
        return Err(ParseError::ExpectedList)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Checks if the top element on the stack is a list, codeblock or string. 
/// If it is, it inserts the length on the top of the stack and removes the list, codeblock or string.
pub fn op_length(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if let Some (top_element) = stack.get(0).cloned(){  // Need cloned to avoid borrow issues below
        match top_element {                             // Find the type of the top element
            V::VList(list) => {
                stack.insert(0, V::VInt(list.len() as i64))
            }
            V::VCodeBlock(c) => {
                let code_block_no_braces =&c[1..c.len() - 1];   // Remove the { and }
                // Need to get the tokens from the codeblock since it is a string
                let code_block_tokens: Vec<&str> = code_block_no_braces.split_whitespace().collect();
                stack.insert(0, V::VInt(code_block_tokens.len() as i64))
            }
            V::VString(s) =>{ 
                stack.insert(0, V::VInt(s.len() as i64 - 2))  // -2 for space before and after "
            } 
            _ => return Err(ParseError::ExpectedQuotation)
        }
    } else {
        return Err(ParseError::StackEmpty)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Append an element to the front of a list. 
pub fn op_cons (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements)
    } 
    let literal = stack.remove(1);                          // remove literal from stack      
    if let Some(V::VList(list)) = stack.get_mut(0) {        // top element is a list
        list.insert(0, literal);
    } else {
        return Err(ParseError::ExpectedList)     
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Appends one list to the end of another list.
/// [ 1 2 ] [ 3 4 ] append, results in [ 3 4 ] being appended to [ 1 2 ] result: [ 1 2 3 4 ]
pub fn op_append (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements)
    } 
    if let Some(V::VList(list2)) = stack.get(0){    // Get the list to be appended to the other
        let list2_clone = list2.clone();
        stack.remove(0);                                    // Remove list 1
        if let Some(V::VList(list1)) = stack.get_mut(0) {   // Now we can get as mutable reference 
            list1.extend(list2_clone);                      // Extends list1 with list2
        } else {
            return Err(ParseError::ExpectedList)
        }
    } else {
        return Err(ParseError::ExpectedList)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Executes the tokens within a codeblock
pub fn op_exec (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if let Some(V::VCodeBlock(code_block)) = stack.get(0).cloned(){ 
        stack.remove(0);
        // Convert the codeblock to tokens that can be processed 
        let code_block_tokens: Vec<&str> = parse_code_block_tokens(&code_block);
        parser::process_tokens(&code_block_tokens, stack, var_and_fun)?;  // Process the codeblock
    } else {
        return Err(ParseError::EmptyOrNotCorrectType)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// For each element in a list it executes a given code block. if is_map is 1 the original list is updated 
/// and we have done a map, if it is 0 then the original list is removed and we performed each.
pub fn op_map_or_each(stack: &mut Stack, tokens: &[&str], is_map: i64, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    let mut process_next = false;
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements)
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
                    parser::process_tokens(&code_block_tokens, &mut dummy_stack, var_and_fun)?; // codeblock executed for list element
                }
                Some(operation) if  parser::is_valid_element_each_map(operation.to_string().as_str()) => { 
                    process_next = true;
                    parser::process_tokens(&[operation.to_string().as_str()], &mut dummy_stack, var_and_fun)?; // Valid symbol executed for list element
                }
                _ => {
                    if !process_next{   // Only needs to be printed once so can use process_next for this
                        return Err(ParseError::ExpectedQuotation)
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
        return Err(ParseError::ExpectedList)
    }
    parser::process_tokens(&tokens[if process_next { 1 } else { 0 }..], stack, var_and_fun)?; Ok(())
}

/// Uses a list, starting value and code block tofor example sum up a list:
/// [1 5 9 20 ] 0 foldl { + }  on an empty stack will result in the stack: [35]
pub fn op_foldl(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    let mut process_next = false;
    if stack.len() < 3 {
        return Err(ParseError::NotEnoughElements)
    } 
    if let (Some(V::VList(list)), Some(V::VInt(init_acc))) = (stack.get(2), stack.get(1)) {
        let mut acc = *init_acc;

        // Check if the top element is a valid operation or a code block
        match stack.get(0) {
            Some(V::VCodeBlock(code_block)) => {
                acc = process_list_for_foldl(acc, list, var_and_fun, Some(code_block), None)?;
            }
            Some(operation) if parser::is_valid_element(operation.to_string().as_str()) => {
                process_next = true;
                acc = process_list_for_foldl(acc, list, var_and_fun, None, Some(operation))?;
            }
            _ => {
                return Err(ParseError::ExpectedList)
            }
        }
        stack.remove(0); // Remove the code block or valid operation from the stack
        stack.remove(0); // Remove the initial accumulator from the stack
        stack.remove(0); // Remove the list from the stack
        stack.insert(0, V::VInt(acc)); // Insert the accumulated value to the stack

    } else {
        return Err(ParseError::NonCompatibleTypes)
    }

    // If it was a codeblock, the index for the next token is already sent by op_infix,
    // so we dont have to send the index for the next token
    parser::process_tokens(&tokens[if process_next { 1 } else { 0 }..], stack, var_and_fun)?; Ok(())
}


/// syntax: Condition if Then Else. Gets the condition and then else block from the stack.
/// If condition is true executes the Then block, otherwise it executes the Else block
pub fn op_if (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if stack.len() < 3 {
        return Err(ParseError::NotEnoughElements)
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
                parser::process_tokens(&code_block_tokens, stack, var_and_fun)?;
            }
            // Check if it is a valid element before processing the operation
            operation if parser::is_valid_element(operation.to_string().as_str()) => {
                parser::process_tokens(&[operation.to_string().as_str()], stack, var_and_fun)?;
            }
            _ => {
                return Err(ParseError::ExpectecCodeBlockOrValidOperation)
            }
        }
    } else {
        return Err(ParseError::ExpectecCodeBlockOrValidOperation)
    }

    parser::process_tokens(&tokens, stack, var_and_fun)?; Ok(())
    
}

/// Syntax : loop break block. Executes the block until break becomes true
/// where both break and block must be codeblocks 
pub fn op_loop(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements);
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
            process_tokens(&break_cond_tokens, &mut break_stack, var_and_fun)?;

            // Check if the break condition is true
            if let Some(V::VBool(true)) = break_stack.get(0) {
                break;
            }

            // Parse and execute the codeblock on the main stack
            let code_block_tokens = parse_code_block_tokens(&code_block);
            process_tokens(&code_block_tokens, stack, var_and_fun)?;
        }

    } else {
        return Err(ParseError::ExpectedCodeblock)
    }
    process_tokens(&tokens, stack, var_and_fun)?; Ok(())
}

/// Execute a code block x number of times
/// For example 5 times { 10 10 + } will result in the stack [20, 20, 20, 20, 20]
pub fn op_times (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    let mut process_next = false;
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements)
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
                    parser::process_tokens(&code_block_tokens, stack, var_and_fun)?;
                }
            }
            operation if parser::is_valid_element(operation.to_string().as_str()) => {
                process_next = true;
                // Execute the valid operation num_times
                for _ in 0..num_times {
                    parser::process_tokens(&[operation.to_string().as_str()], stack, var_and_fun)?;
                }
            }
            _ => {
                return Err(ParseError::ExpectecCodeBlockOrValidOperation)
            }
        }      
    } else {
        return Err(ParseError::NonCompatibleTypes)
    }
    parser::process_tokens(&tokens[if process_next { 1 } else { 0 }..], stack, var_and_fun)?; Ok(())
}

/// Assigne a name to a literal for example: myName " Aleksander " := . Assigns myName to Aleksander
/// So when doing myName later Aleksander will be pushed to the stack.  
pub fn op_assign_variable (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements);
    } 
    let value = stack.remove(0);
    // Make sure it is not another valid type that has defined behavior, we use WValue::VOther for this
    if let Some(V::VOther(key)) = stack.get(0).cloned() {
        stack.remove(0);
        var_and_fun.insert(key, value);         // Insert the key with the value to the hashmap
    } else {
        return Err(ParseError::ExpectedVOther)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Same as op_assign_variable but with functions instead and different arguments.  
/// Syntax: VOther codeblock fun    
pub fn op_assign_function (stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{
    if stack.len() < 2 {
        return Err(ParseError::NotEnoughElements)
    } 
    if let Some(code_block) = stack.get(0).cloned(){
        stack.remove(0);
        if let Some(V::VOther(key)) = stack.get(0).cloned() {
            var_and_fun.insert(key, code_block);
            stack.remove(0);
        } else {
            return Err(ParseError::ExpectedVOther)
        }
    } else {
        return Err(ParseError::ExpectedCodeblock)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}

/// Parse a string from stack to a integer or float and puts it back onto the stack
pub fn op_parse_num(stack: &mut Stack, parse_float: bool, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if let Some(top_element) = stack.get_mut(0) {
        match top_element {
            V::VString(s) => {
                let s_no_qoutes = &s[1..s.len() - 1];   // Remove the surrounding qoutes
                if parse_float {
                    // Try to parse to f32
                    if let Ok(f) = s_no_qoutes.parse::<f32>() {
                        *top_element = V::VFloat(f); // Replace the VString with the parsed VFloat
                    } else {
                        return Err(ParseError::CouldNotParse)
                    }
                } else {
                    // Try to parse to i64
                    if let Ok(i) = s_no_qoutes.parse::<i64>() {
                        *top_element = V::VInt(i); // Replace the VString with the parsed VInt
                    } else {
                        return Err(ParseError::CouldNotParse)
                    }
                }
            }
            _ => {
                return Err(ParseError::ExpectedString)
            }
        }
    } else {
        return Err(ParseError::StackEmpty)
    }
    parser::process_tokens(&tokens[1..], stack, var_and_fun)?; Ok(())
}


/// Adds a String, codeblock or list to the stack depending on the 'starting_symbol'
/// Only supports 2 layers of nesting at this point and is very smelly as Mariusz would say
pub fn op_enclosed(stack: &mut Stack, tokens: &[&str], starting_symbol: String, process_next: bool, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    let mut new_string = String::new();            // {} and "" represented as a string
    let mut new_elements: Vec<V> = Vec::new();     // [] list represented as a vector 
    let mut index = 0;

    // Function only called for these three starting symbols, no need for handling exceptions
    let (start_char, end_char): (String, String) = match starting_symbol.as_str() {
        "\"" => ("\"".to_string(), "\"".to_string()),
        "[" => ("[".to_string(), "]".to_string()),
        "{" => ("{".to_string(), "}".to_string()),
        _ => return Err(ParseError::FirstElemNotValid), 
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
                            _ => return Err(ParseError::FirstElemNotValid), // will not reach this
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
                        op_enclosed(&mut sub_stack, &sub_tokens, token.to_string(), true, var_and_fun)?;
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
                    if *token == "\"" || *token == "{" || *token == "[" {
                        let mut sub_tokens = Vec::new();
                        sub_tokens.push(token.to_string());
                        let mut j = i + 1;
                        let sub_end_char = match *token {   //Update new end char 
                            "\"" => "\"",
                            "{" => "}",
                            "[" => "]",
                            _ => return Err(ParseError::FirstElemNotValid), // will not reach this
                        };
                
                        while let Some(token) = tokens.get(j) {
                            // If list contains a assigned variable insert the value of that variable
                            if var_and_fun.contains_key(token.clone()) {
                                let value = var_and_fun.get(token.clone()).unwrap().clone();
                                sub_tokens.push(value.to_string());
                            } else {
                                sub_tokens.push(token.to_string());
                            }
                            if token.contains(sub_end_char) {
                                break;
                            }
                            j += 1;
                        }
                        let sub_tokens_ref = sub_tokens.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
                        let mut sub_stack: Stack = Vec::new();
                        op_enclosed(&mut sub_stack, &sub_tokens_ref, token.to_string(), true, var_and_fun)?;
                        if let Some(value) = sub_stack.get(0) {
                            new_elements.push(value.clone());
                        }
                        i = j + 1;
                    } else {
                        // If list contains a assigned variable insert the value of that variable
                        if var_and_fun.contains_key(token.clone()) {
                            let value = var_and_fun.get(token.clone()).unwrap().clone();
                            new_elements.push(value);
                        } else {
                            match V::from_string(token) {
                                value => new_elements.push(value),
                            }
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
            _ => return Err(ParseError::FirstElemNotValid), 
        }
    } else {
        return Err(ParseError::MissingClosingQuote)
    }
    if process_next {
        parser::process_tokens(&tokens[index + 1..], stack, var_and_fun)?;
    } 
    Ok(())
}

/// TODO DOES NOT HANDLE STRINGS AND LISTS INSIDE CODEBLOCKS CORRECTLY
/// Helper function to parse the codeblock into a vector of tokens
fn parse_code_block_tokens(code_block: &str) -> Vec<&str> {
    
    let code_block_no_braces = &code_block[1..code_block.len() - 1];
    let code_block_tokens: Vec<&str> = code_block_no_braces.split_whitespace().collect();
    let mut processed_tokens = Vec::new();
    // The first part here does not correctly split the lists and strings within codeblocks 
    // Since they are stored like this { [1, 2, 3, 4] }, which then turns [1 into one token
    // same goes for strings, therefore we need to split it further such that it becomes [ 1
    
    for token in code_block_tokens {
        let first_char = token.chars().next().unwrap(); // Get the first and last character if it is only
        let last_char = token.chars().last().unwrap();  // one they become the same which is fine 

        if first_char == '[' || first_char == '"' {         // Push [ or " first then variable
            processed_tokens.push(&token[..1]);      
            processed_tokens.push(&token[1..]);
        } else if last_char == ']' || last_char == '"' {    // Opposite here variable first then ] or "
            processed_tokens.push(&token[..token.len() - 1]);
            processed_tokens.push(&token[token.len() - 1..]);
        } else {
            processed_tokens.push(token);       // No [ or "
        }
    }
    processed_tokens
}

/// Does foldl on the list sent as a parameter, returns the acummulated value to be placed on the stack
fn process_list_for_foldl(acc: i64, list: &[V],
    var_and_fun: &mut HashMap<String, V>, code_block: Option<&str>, operation: Option<&V>) -> Result<i64, ParseError> {
    let mut accumulator = acc;
    
    // Go through each list element
    for elem in list {
        // Create a dummy stack with the list element and the accumulated value 
        let mut dummy_stack: Vec<V> = vec![elem.clone(), V::VInt(accumulator)];

        // Process either the code_block or operation on the dummy stack
        if let Some(code_block) = code_block {
            let code_block_tokens: Vec<&str> = parse_code_block_tokens(code_block);
            parser::process_tokens(&code_block_tokens, &mut dummy_stack, var_and_fun)?;
        } else if let Some(operation) = operation { // Uses else if to unwrapt he operation first
            parser::process_tokens(&[operation.to_string().as_str()], &mut dummy_stack, var_and_fun)?;
        }

        // Update the accumulator with the new accumulated value 
        if let Some(V::VInt(new_acc)) = dummy_stack.get(0) {
            accumulator = *new_acc;
        } else {
            return Err(ParseError::InvalidListElement);
        }
    }
    Ok(accumulator)
}

