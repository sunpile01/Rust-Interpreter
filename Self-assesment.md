## The funcitonality of my program compared to the functional requirements:

My program allows for almost everything that was specified in the file "Functional-requirements.md". The only limitations my program currently has and that I am looking to improve before the final delivery are: 

- Does not support more than 2 layers of nesting for both lists and codeblocks.
- When not running in 'repl' mode and not utilizing filemode. The input does not allow for writing on several lines with using enter, since right now enter signals the end of input and the input will be processed after pressing enter. 

## The non-functional behaviour of my progra, compared to the non-functional requirements:

- Performance
    - The interpreter does execute most operations with the acceptable response time specified.
    - The interpreter also handles larger expressions without noticably large execution times. 

- Usability
    - The error messages are quite clear if you enter short input. However, it can be hard to find exactly where the error is caused if the input is large (specially for large files).
    - The manual is quite easy to understand and it covers all operations for the interpreter quite well, and should help the user understand how to use the interpreter. One improvement would be to add even more examples that explains different cases. 

- Scalability
    - The interpreter should be designed to allow for future expansion, such as adding new data types, operations, or features.
    - The interpreter allows for further expansions of new operations and data types quite easily. However, implementing a different way for handling all the operations and not passing around all the parameters to every single operation, would be a good improvement.

- Maintainability
    - The code is well documented and allows for easy updates and maintenance.
    - The project has decent modularization. 

- Portability
    - The interpreter can be run on different operating systems as long as the explained tools are present on the users computer.

- Reliability
    - The interpreter handles unexpected input well and does not crash when running. 

- Testability
    - The interpreter includes a comprehensive list of tests covering all major functionality.
    - The list of tests is easy to expand for new test cases and if new operations are implemented
- Security
    - The interpreter does not expose any security vulnerabilities

- Documentation
    - The documentation for the interpreter does explain the program well to the user and is easily accessible. It also provides information for the necessary requirements and the development process

## Reflection

I am overall very happy with the outcome of the project. The interpreter works well according to the functional requirements set for the project. The process to get to where the project is now was not easy though. 

I spent a lot of time working on this project (probably closer to 110 hours). I refactored the code multiple times, for example when I went from having a stack type being a vector of <WValue, err> to then only having <WValue> and printing errors manually. Then in the end I removed all the manual print lines in each function, for every error case and added an error called 'ParseError' type instead. Now every function returns an "Result<(), ParseError>" value and the error is now handled as it occurs which is much cleaner. 

Additionally I several times had to change the order of how the operation processed the stack elements. I also ended up with several very long functions at times for example the 'infix_op' function now utilizes 4 helper funcitons, but at one point, everything was in one single function. I have definetly learned that it is better to facilitate for adding the helper functions earlier on, rather than creating a mess and having to clean it up later.

I learned a lot about documentation from the first assignment and I believe I have used this knowledge well for this assignment. The documentation is cleaner and more comprehensive than what I did for the last assignment. Additionally, I was better at doing commits during the entire development process, and not just try to simulate commits in the end like I did last time (because I had forgotten it during development of the first assignment). 

One thing I that could be improved is the way I am passing the same arguments around all the time. Maybe just adding global variables would fix this and I will look into improving this for the final delivery of the portfolio. Some functions could probably be more conscise and shortened aswell. 
