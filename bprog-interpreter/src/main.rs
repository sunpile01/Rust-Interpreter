mod lib;

use crate::lib::parser;
fn main() {
    let input = "5 3 + 7 *";
    let output = parser::process_input(input);
    println!("Output: {}", output);
}
