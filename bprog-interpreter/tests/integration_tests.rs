#[cfg(test)]
mod tests {
    use bprog_interpreter::parser::process_input;

    #[test]
    fn test_basic_operations() {
        assert_eq!(process_input("3 4 +"), "7");
        assert_eq!(process_input("3 4 *"), "12");
    }

    #[test]
    fn test_combined_operations() {
        assert_eq!(process_input("3 4 + 5 *"), "35");
        assert_eq!(process_input("5 3 4 + *"), "35");
    }

    #[test]
    fn test_pop_operation() {
        assert_eq!(process_input("3 4 + pop"), "");
        assert_eq!(process_input("3 4 * pop"), "");
    }

    #[test]
    fn test_comments() {
        assert_eq!(process_input("3 4 + \" this is a comment"), "7");
        assert_eq!(process_input("3 4 * \" this is another comment"), "12");
    }

    #[test]
    fn test_wrong_input() {
        assert_eq!(
            process_input("3 *"),
            "Not enough arguments for * 3"
        );
        assert_eq!(
            process_input("3 +"),
            "Not enough arguments for + 3"
        );
        assert_eq!(
            process_input("pop"),
            "Not enough arguments for pop"
        );
        assert_eq!(
            process_input("hello"),
            "Parsing error, expected a number, got: hello"
        );
    }
}