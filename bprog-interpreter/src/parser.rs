
use std::collections::HashMap;

use crate::stack_operations as operations;
use crate::types::{Stack, OpBinary, WValue as V, ParseError};


/// Parses the string input into "tokens" for example *, +, then calls the process_tokens function to 
/// execute the corresponding code depending on the tokens. 
pub fn process_input(line: &str, stack: &mut Stack, var_and_fun: &mut HashMap<String, V>) {
    let tokens = line.split_whitespace().collect::<Vec<&str>>(); // Get all symbols separated by space
    // Process the token inputs
    if let Err(error) = process_tokens(&tokens, stack, var_and_fun) {
        // Handle the error
        match error {
            ParseError::StackEmpty => println!("Error: Stack empty"),
            ParseError::ExpectedList => println!("Error: Top element not a list"),
            ParseError::ListEmpty => println!("Error: List is empty, can't return an element"),
            ParseError::ExpectedVOther => println!("Error: Expected VOther type for this operation!"),
            ParseError::CouldNotParse => println!("Error: Could not parse the entered string!"),
            ParseError::NotEnoughElements => println!("Error: Not Enough Elements provided for the operation"),
            ParseError::MissingClosingQuote => println!("Error: Missing closing quote for either list, codeblock or string"),
            ParseError::ExpectedCodeblock => println!("Error: The operation expected a codeblock!"),
            ParseError::ExpectedQuotation => println!("Error: The operation expected either a codeblock or a list!"),
            ParseError::ExpectecCodeBlockOrValidOperation => println!("Error: The operation expected a codeblock or a valid operation!"),
            ParseError::ExpectedBoolOrNumber => println!("Error: The operation expected either a bool or a number!"),
            ParseError::ExpectedString => println!("Error: The operation expected a string!"),
            ParseError::DivisionByZero => println!("Error: Division by 0 is not allowed!"),
            ParseError::NonCompatibleTypes => println!("Error: The types were not compatible for the given operation!"),
            ParseError::FirstElemNotValid => println!("Error: The first element was not valid for the given operation!"),
            ParseError::InvalidListElement => println!("Error: There was an invalid element in the list for the given operation!"),
            ParseError::ExpectedVariable => println!("Error: The operation ' expected a self defined variable or function!"),
            ParseError::EmptyOrNotCorrectType => println!("Error: Either wrong type provided for the operation or missing type/types!"),
        }
    }
}

/// Processes the tokens sent by process_input and handles the different type of tokens
/// This function is called by each operation after executing the operation.
pub fn process_tokens(tokens: &[&str], stack: &mut Stack, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if !tokens.is_empty() {
        // Do operation according to what the top token is. 
        match tokens[0] {
            "*" => operations::op_binary(stack, OpBinary::Multiply,  &tokens, var_and_fun)?, 
            "+" => operations::op_binary(stack, OpBinary::Add,  &tokens, var_and_fun)?, 
            "-" => operations::op_binary(stack, OpBinary::Subtract,  &tokens, var_and_fun)?,
            "/" => operations::op_binary(stack, OpBinary::FDivide,  &tokens, var_and_fun)?,
            "div" => operations::op_binary(stack, OpBinary::IDivide, &tokens, var_and_fun)?,
            "<" => operations::op_binary(stack, OpBinary::RGreater, &tokens, var_and_fun)?,
            ">" => operations::op_binary(stack, OpBinary::LGreater, &tokens, var_and_fun)?,
            "==" => operations::op_binary(stack, OpBinary::Equality, &tokens,var_and_fun)?,
            "&&" => operations::op_binary(stack, OpBinary::And, &tokens, var_and_fun)?,
            "||" => operations::op_binary(stack, OpBinary::Or, &tokens, var_and_fun)?,

            "not" => operations::op_not(stack, &tokens, var_and_fun)?,
            "\"" => operations::op_enclosed(stack, &tokens, "\"".to_string(), true, var_and_fun)?, 
            "[" => operations::op_enclosed(stack, &tokens, "[".to_string(), true, var_and_fun)?, 
            "{" => operations::op_enclosed(stack, &tokens,"{".to_string(), true, var_and_fun)?,

            "dup" => operations::op_dup(stack, &tokens, var_and_fun)?,
            "swap" => operations::op_swap(stack, &tokens, var_and_fun)?,

            "print" => operations::op_print(stack, &tokens, var_and_fun)?,
            "read" => operations::op_read(stack, &tokens, var_and_fun)?,
            
            "parseInteger" => operations::op_parse_num(stack, false, &tokens, var_and_fun)?,
            "parseFloat" => operations::op_parse_num(stack, true, &tokens, var_and_fun)?,
            "words" => operations::op_words(stack, &tokens, var_and_fun)?,

            "head" => operations::op_head(stack, &tokens, var_and_fun)?,
            "tail" => operations::op_tail(stack, &tokens, var_and_fun)?,
            "empty" => operations::op_empty(stack, &tokens, var_and_fun)?,
            "length" => operations::op_length(stack, &tokens, var_and_fun)?,
            "cons" => operations::op_cons(stack, &tokens, var_and_fun)?,
            "append" => operations::op_append(stack, &tokens, var_and_fun)?,
            "each" => infix_op(stack, &tokens, 0, var_and_fun)?,
            "map" => infix_op(stack, &tokens, 1, var_and_fun)?,
            "foldl" => infix_op(stack, &tokens, 2, var_and_fun)?,
            "if" => infix_op(stack, &tokens, 3, var_and_fun)?,
            "times" => infix_op(stack, &tokens, 4, var_and_fun)?,
            "loop" => infix_op(stack, &tokens, 5, var_and_fun)?,

            ":=" => operations::op_assign_variable(stack, &tokens, var_and_fun)?,
            "fun" => operations::op_assign_function(stack, &tokens, var_and_fun)?,
            "exec" => operations::op_exec(stack, &tokens, var_and_fun)?,
            "pop" => operations::op_pop(stack, &tokens, var_and_fun)?,
            ":b" => print_bindings(stack, &tokens, var_and_fun)?,
            "'" => operations::op_tick(stack, &tokens, var_and_fun)?,
            "eval" => operations::op_eval(stack, &tokens, var_and_fun)?,
            ":s" => {
                println!("{:?}", stack); process_tokens(&tokens[1..], stack, var_and_fun)?;
            }
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
                        process_tokens(&["exec"], stack, var_and_fun)?;
                    }
                    process_tokens(&tokens[1..], stack, var_and_fun)?;
                }
                else {               // If not a symbol put it on the stack as a VOther type done by op_num
                    operations::op_num(stack, tokens[0]);
                    process_tokens(&tokens[1..], stack, var_and_fun)?;
                }
            }
        }
    } 
    Ok (())
}

/// Handles all the cases where the provided operation needs to use the arguments that comes after it.
/// This includes: each, map, if, foldl, loop and times. This and the next 4 functions are all for 
/// correctly handling these cases. 
fn infix_op(stack: &mut Stack, tokens: &[&str], operation_type: i64, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    if let Some(next_token) = tokens.get(1) {       // Get the first token after the operation
        match next_token {
            token if token.starts_with("{") => {    // If it is a codeblock
                handle_code_block_case(stack, tokens, operation_type, var_and_fun)?;
            }
            token if is_valid_element(token) => {   // If it is a valid element 
                handle_valid_element_case(stack, tokens, operation_type, var_and_fun)?;
            }
            _ => {
                return Err(ParseError::ExpectecCodeBlockOrValidOperation);
            }
        }
    } else {
        return Err(ParseError::ExpectecCodeBlockOrValidOperation);
    }
    Ok(())
}

/// Handles the case where the first element provided after the operation is a codeblock 
/// and calls the corresponding function to handle it depending on the operation_type.
fn handle_code_block_case(stack: &mut Stack, tokens: &[&str], operation_type: i64, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    // Get the entire codeblock that begins with the { 
    operations::op_enclosed(stack, &tokens[1..], "{".to_string(), false, var_and_fun)?;

    // Here we find the index of the closing brace so we can send the index to the next function
    if let Some(closing_brace_index) = find_matching_brace(&tokens[1..]) {
        // Call correct function
        match operation_type {
            0 => operations::op_map_or_each(stack, &tokens[closing_brace_index + 2..], 0, var_and_fun)?,
            1 => operations::op_map_or_each(stack, &tokens[closing_brace_index + 2..], 1, var_and_fun)?,
            2 => operations::op_foldl(stack, &tokens[closing_brace_index + 2..], var_and_fun)?,
            // If and loop both require 2 codeblocks or valid operations so we need to process the next
            3 | 5 => handle_if_or_loop_case(stack, tokens, operation_type, closing_brace_index, var_and_fun)?,
            4 => operations::op_times(stack, &tokens[closing_brace_index + 2..], var_and_fun)?,
            _ => {}
        }
    } else {
        return Err(ParseError::MissingClosingQuote);
    }
    Ok(())
}

/// This function is for handling the last codeblock or valid operation that the if and loop functions need
/// For example if the input is "if { 10 + } + " then this function handles the last +
/// for "if + { 10 + }" we handle the { 10 + }, same for loop 
fn handle_if_or_loop_case(stack: &mut Stack, tokens: &[&str], operation_type: i64, closing_brace_index: usize, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    let second_token = tokens.get(closing_brace_index + 2).unwrap();

    if second_token.starts_with("{") {
        // Same as before need to get the entire codeblock inserted to the stack
        operations::op_enclosed(stack, &tokens[closing_brace_index + 2..], "{".to_string(), false, var_and_fun)?;
        // Make sure there is a closing brace and get its index
        if let Some(second_closing_brace_index) = find_matching_brace(&tokens[closing_brace_index + 2..]) {
            if operation_type == 3 {
                operations::op_if(stack, &tokens[closing_brace_index + second_closing_brace_index + 3..], var_and_fun)?;
            } else {
                operations::op_loop(stack, &tokens[closing_brace_index + second_closing_brace_index + 3..], var_and_fun)?;
            }
        } else {
            return Err(ParseError::MissingClosingQuote);
        }
    // Token was not a codeblock check if it is a valid element
    } else if is_valid_element(second_token) {
        // If it was valid insert it and call function for handling the operation
        stack.insert(0, V::VString(second_token.clone().to_string()));
        operations::op_if(stack, &tokens[closing_brace_index + 3..], var_and_fun)?;
    } else {
        return Err(ParseError::ExpectecCodeBlockOrValidOperation);
    }
    Ok(())
}

/// This is when the first element was a valid element and not a codeblock
/// Simply inserts it into the stack and calls the function to process the operation
fn handle_valid_element_case(stack: &mut Stack, tokens: &[&str], operation_type: i64, var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    stack.insert(0, V::VString(tokens[1].clone().to_string()));
    match operation_type {
        0 => operations::op_map_or_each(stack, &tokens[1..], 0, var_and_fun)?,
        1 => operations::op_map_or_each(stack, &tokens[1..], 1, var_and_fun)?,
        2 => operations::op_foldl(stack, &tokens[1..], var_and_fun)?,
        3 => handle_if_case(stack, tokens, var_and_fun)?,           // Loop can not be called with valid element
        4 => operations::op_times(stack, &tokens[1..], var_and_fun)?,
        _ => {} // Should never be called with anything other than 0,1,2,3,4
    }
    Ok(())
}

/// Handles the if case this is called when the first element was a valid operation and not a codeblock
/// Inserts the second codeblock or valid element onto the stack and calls the op_if function
fn handle_if_case(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError> {
    
    if let Some(second_token) = tokens.get(2) {          // Get the valid symbol or starting brace {
        if second_token.starts_with("{") {
            // This functions inserts the codeblock to the stack
            operations::op_enclosed(stack, &tokens[2..], "{".to_string(), false, var_and_fun)?;
            // Checks if there is a closing brace and retuns its index
            if let Some(second_closing_brace_index) = find_matching_brace(&tokens[2..]) {
                operations::op_if(stack, &tokens[second_closing_brace_index + 3..], var_and_fun)?;
            } else {
                return Err(ParseError::MissingClosingQuote);
            }
        } else if is_valid_element(second_token) {                         // Make sure it is a valid element 
            stack.insert(0, V::VString(second_token.clone().to_string())); // Insert it to the stack
            operations::op_if(stack, &tokens[3..], var_and_fun)?;           // Process the if operation 
        } else {
            return Err(ParseError::ExpectecCodeBlockOrValidOperation);
        }
    }
    Ok(())
}

fn print_bindings(stack: &mut Stack, tokens: &[&str], var_and_fun: &mut HashMap<String, V>) -> Result<(), ParseError>{

    for (key, value) in var_and_fun.clone() {
        println!("Binding name: {}, Value: {:?}", key, value);
    }

    process_tokens(&tokens[1..], stack, var_and_fun)?;
    Ok(())
}
/// Helper function to find the mathing end brace for the brace index it is sent as a parameter
pub fn find_matching_brace(tokens: &[&str]) -> Option<usize> {
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
        "/" | "<" | ">" | "==" | "true" | "false" => true,
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

