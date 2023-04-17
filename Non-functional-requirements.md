## Non-functional requirements for the Bprog-language interpreter
- Performance
    - The interpreter should process and execute most operations within an acceptable response time (e.g., less than 300 milliseconds) for simple input expressions.
    - The interpreter should be able to handle large input expressions without noticably large execution times.

- Usability
    - The interpreter should provide clear and concise error messages when encountering incorrect syntax or other issues during execution.
    - The interpreter's manual should be easily understandable and comprehensive, providing users with the necessary information to effectively use the interpreter.

- Scalability
    - The interpreter should be designed to allow for future expansion, such as adding new data types, operations, or features.

- Maintainability
    - The source code should be well-documented and follow a consistent coding style to facilitate easy maintenance and updates.
    - The interpreter should be modular, with a clear separation of concerns between different components (e.g., parsing, execution, error handling).

- Portability
    - The interpreter should be platform-independent and capable of running on multiple operating systems (e.g., Windows, macOS, Linux) with minimal modification.

- Reliability
    - The interpreter should handle unexpected input gracefully, providing informative error messages and avoiding crashes or undefined behavior.

- Testability
    - The interpreter should include a comprehensive test suite covering all major functionality.
    - The test suite should be easily expandable, allowing for the addition of new test cases as needed.

- Security
    - The interpreter should not expose any security vulnerabilities, such as buffer overflows.

- Documentation
    - The interpreter's documentation should explain the program well to the user, and be easily accessible. It should also provide information about the requirements for the program and the development process. 