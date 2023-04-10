mod lib;

use crate::lib::parser;
fn main() {
    let input = "True False +";
    let output = parser::process_input(input);
    println!("Output: {}", output);
}
