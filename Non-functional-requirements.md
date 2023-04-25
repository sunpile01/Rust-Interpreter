## Non-functional requirements for the Bprog-language interpreter
- Performance
    - The interpreter should process and execute most operations within an acceptable response time ( for ecample less than 300 milliseconds) for simple input expressions.
    - The interpreter should be able to handle large input expressions without noticably large execution times.

- Usability
    - The interpreter should provide clear and concise error messages when encountering incorrect syntax or other issues during execution.
    - The interpreter's manual should be easily understandable and comprehensive, providing users with the necessary information to effectively use the interpreter.

- Scalability
    - The interpreter should be designed to allow for future expansion, such as adding new data types, operations, or features.

- Maintainability
    - The source code should be well documented to allow for easy maintenance and updates.
    - The interpreter should have good modularization and seperate the different main functionalities of the program into seperate files.

- Portability
    - The interpreter should be capable of running on multiple operating systems as long as the necessary rust installations are present on the users computer.

- Reliability
    - The interpreter should handle unexpected input gracefully, providing informative error messages and avoiding crashes or undefined behavior.

- Testability
    - The interpreter should include a comprehensive list of tests covering all major functionality.
    - The list of tests should be easily expandable, allowing for the addition of new test cases as needed.

- Security
    - The interpreter should not expose any security vulnerabilities.

- Documentation
    - The interpreter's documentation should explain the program well to the user, and be easily accessible. It should also provide information about the requirements for the program and the development process. 