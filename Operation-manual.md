## Rundwon of each operation with explanation and examples 

### Arithmetic operations
- '+': is the addition operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Integers and floats, you can also add one of each type together. 
    - Examples:
        - "1 2 +": pushes 1 and 2 together then adds them together, pushes the new 3 onto the stack and removes the elements included in the addition 
        - "4 5.5 +": Same as above but here the integer 4 is converted to a float and 9.5 is added to the stack.

- '-': is the subtraction operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Integers and floats, you can also subtract one of each type together.
    - Examples:
        - "5 2 -": pushes 5 and 2 onto the stack, then subtracts 2 from 5, pushes the result 3 onto the stack, and removes the elements included in the subtraction
        - "7 3.5 -": Same as above but here the integer 7 is converted to a float and 3.5 is subtracted, resulting in 3.5 which is added to the stack.

- '*': is the multiplication operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Integers and floats, you can also multiply one of each type together.
    - Examples:
        - "3 4 *": pushes 3 and 4 onto the stack, then multiplies them together, pushes the result 12 onto the stack, and removes the elements included in the multiplication
        - "6 2.5 *": Same as above but here the integer 6 is converted to a float and multiplied by 2.5, resulting in 15.0 which is added to the stack.

- '/': is the division operation for floats, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Integers and floats, you can also divide one of each type together.
    - Examples:
        - "8 2 /": pushes 8 and 2 onto the stack, then divides 8 by 2, pushes the result 4 onto the stack, and removes the elements included in the division
        - "9 4.5 /": Same as above but here the integer 9 is converted to a float and divided by 4.5, resulting in 2.0 which is added to the stack.


- 'div': is the integer division operation, there needs to be two integer elements on the stack before performing this operation. The types supported for this operation are: Integers and floats.
    - Examples:
        - "10 3 div": pushes 10 and 3 onto the stack, then performs integer division (10 divided by 3), pushes the result 3 onto the stack, and removes the elements included in the division
        - "9 4.5 /": Same as above but here the float 4.5 is converted to the integer 4, 9 / 4 resulting in 2.25 but it is converted to an integer so 2 is added to the stack.


- '<': is the less-than comparison operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Integers, floats, codeblocks, strings and lists. Floats and integers can be compared to one another, but not against the others and the others can only be comapred against the same type. So list against list, codeblock against codeblock and string against string.
    - Example:
        - "4 7 <": pushes 4 and 7 onto the stack, then compares if 4 is less than 7, pushes the result true onto the stack, and removes the elements included in the comparison. This works how you would expect it to when comparing an int to a float.  

- '>': The exact same as above but just the opposite order so 7 4 > checks if 7 is greater than 4

- '==': is the equality comparison operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Integers, floats, strings, codeblocks and lists. You can also compare integers with floats. But the same rules apply as the < and > operation when it comes to list, strings and codeblocks.
    - Examples:
    - "4 4 ==": pushes 4 and 4 onto the stack, then compares if 4 is equal to 4, pushes the result true onto the stack, and removes the elements included in the comparison
    - "2.0 2 ==": Same as above but here the float 2.0 is compared to the integer 2, resulting in true which is added to the stack.
    - ""hello" "hello" ==": pushes "hello" and "hello" onto the stack, then compares if the two strings are equal, pushes the result true onto the stack, and removes the elements included in the comparison

### Logical operations: 

- '&&': is the logical AND operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Booleans.
    - Examples:
    - "true true &&": pushes true and true onto the stack, then performs the logical AND operation, pushes the result true onto the stack, and removes the elements included in the operation
    - "true false &&": Same as above but the result will be false since the logical AND operation requires both operands to be true.

- '||': is the logical OR operation, there needs to be two elements on the stack before performing this operation. The types supported for this operation are: Booleans.
    - Examples:
        - "true false ||": pushes true and false onto the stack, then performs the logical OR operation, pushes the result true onto the stack, and removes the elements included in the operation
        - "false false ||": Same as above but the result will be false since the logical OR operation requires at least one of the operands to be true.

- 'not': is the logical NOT operation, there needs to be one element on the stack before performing this operation. The types supported for this operation are: Booleans, integers, and floats. For integers and floats, it negates their values.
    - Examples:
        - "true not": pushes true onto the stack, then performs the logical NOT operation, pushes the result false onto the stack, and removes the element included in the operation
        - "5 not": pushes 5 onto the stack, then negates the integer, pushes the result -5 onto the stack, and removes the element included in the operation
        - "2.5 not": pushes 2.5 onto the stack, then negates the float, pushes the result -2.5 onto the stack, and removes the element included in the operation

### Stack operations: 

- 'dup': duplicates the top element on the stack. There needs to be at least one element on the stack before performing this operation.
    - Examples:
        - "5 dup": pushes 5 onto the stack, then duplicates the top element, resulting in the stack containing two 5's: [5, 5]
        - ""hello" dup": pushes "hello" onto the stack, then duplicates the top element, resulting in the stack containing two "hello"'s: ["hello", "hello" ]

- 'swap': swaps the two top elements on the stack. There needs to be at least two elements on the stack before performing this operation.
    - Examples:
        - "5 3 swap": pushes 5 and 3 onto the stack, then swaps the top two elements, resulting in the stack containing 3 and 5: [3, 5]
        - "false true swap": pushes false and true onto the stack, then swaps the top two elements, resulting in the stack containing true and false: [true, false]

- 'pop': removes the top element from the stack. There needs to be at least one element on the stack before performing this operation.
    - Examples:
        "5 pop": pushes 5 onto the stack, then removes the top element, resulting in an empty stack: []
        - "true " this is popped " pop": pushes true and "this is popped" onto the stack, then removes the top element ("this is popped"), resulting in the stack containing just true: [true]

### Parsing Strings

- 'parseInteger': takes a string from the stack and converts it to an Integer, then pushes the Integer onto the stack. There needs to be a string representing a valid integer on the stack before performing this operation.
    - Example:
        - ""123" parseInteger": pushes the string "123" onto the stack, then converts it to an Integer 123 and pushes it onto the stack, resulting in the stack containing the integer 123: [123]

- 'parseFloat': takes a string from the stack and converts it to a Float, then pushes the Float onto the stack. There needs to be a string representing a valid float on the stack before performing this operation.
    - Example:
        - ""3.14" parseFloat": pushes the string "3.14" onto the stack, then converts it to a Float 3.14 and pushes it onto the stack, resulting in the stack containing the float 3.14: [3.14]

- 'words': takes a string from the stack, splits it using Rust's split_whitespace function, and pushes a list of tokens (strings) onto the stack. There needs to be a string on the stack before performing this operation.
    - Example:
    - ""hello world" words": pushes the string "hello world" onto the stack, then splits it into a list of tokens ["hello", "world"] and pushes the list onto the stack, resulting in the stack containing the list: [["hello", "world"]]

### List operations

- 'head': Takes a list from the stack and returns its head (first element). There needs to be a list on the stack before performing this operation.
    - Example:
        - "[ " this is repetetive and tiresome " 2 3 ] head": pushes the list ["this is repetetive and tiresome" , 2, 3] onto the stack, then takes the head "this is repetetive and tiresome" and pushes it onto the stack, resulting in the stack containing ["this is repetetive and tiresome", [1, 2, 3]]

- 'tail': Takes a list from the stack and returns the tail (all elements except the first one). There needs to be a list on the stack before performing this operation.
    - Example:
        - "[ 1 2 3 ] tail": pushes the list [1, 2, 3] onto the stack, then takes the tail ([2, 3]) and pushes it onto the stack, resulting in the stack containing: [[2, 3], [1, 2, 3]]

- 'empty': Takes a list from the stack and returns true if the list is empty. There needs to be a list on the stack before performing this operation.
    - Example:
        - "[] empty": pushes an empty list onto the stack, then checks if its empty and pushes true onto the stack, resulting in the stack containing the boolean true: [true]

- 'length': Takes a list from the stack and pushes the length of the list onto the stack. There needs to be a list on the stack before performing this operation.
    - Example:
        - "[ 1 2 3 ] length": pushes the list [1, 2, 3] onto the stack, then takes its length (3) and pushes it onto the stack, resulting in the stack containing the integer 3: [3]

- 'cons': Takes an item and a list from the stack, and appends the item to the front of the list, then pushes the new list onto the stack. There needs to be an item and a list on the stack before performing this operation.
    - Example:
        - "{ 1 2 3 } [ 4 5 6 7 ] cons": pushes the codeblock { 1 2 3 }  and the list [2, 3] onto the stack, then appends { 1 2 3 } to the front of the list, resulting in the new list [{ 1 2 3 }, 4, 5, 6, 7] on the stack: [[{ 1 2 3 }, 4, 5, 6, 7] ]

- 'append': Takes two lists from the stack and concatenates them, then pushes the new list onto the stack. There needs to be two lists on the stack before performing this operation.
    - Example:
        - "[ 1 2 ] [ 3 4 ] append": pushes the lists [1, 2] and [3, 4] onto the stack, then concatenates them and pushes the new list [1, 2, 3, 4] onto the stack: [[1, 2, 3, 4]]

- 'each': Takes a list and a code block from the stack, and executes the code block on each of the elements of the list. There needs to be a list and a code block on the stack before performing this operation. It does not update the original list, but instead pushes the result of doing the codeblock on the list element to the stack. 
    - Example:
        - "[ 1 2 3 ] each { 20 + }": pushes the list [1, 2, 3] and the code block {1 +} onto the stack, then adds 20 to each element it processes and pushes that element directly to the stack, resulting in the stack:   [21, 22, 23]

- 'map': Takes a list and a code block or operation from the stack, and executes the block or operation on each of the elements of the list, forming a new list that is put on the stack. There needs to be a list and a code block or operation on the stack before performing this operation. It updates the original list with the results of applying the code block or operation to each element.
    - Example:
        - "[ 1 2 3 ] map { 1 + }": pushes the list [1, 2, 3] and the code block { 1 + } onto the stack, then adds 1 to each element it processes and updates the original list with the results, resulting in the stack: [ [2, 3, 4] ]

- 'foldl': Takes a list, an initial accumulator value, and a code block or operation from the stack, and folds the list from left to right by applying the code block or operation to the accumulator and each element of the list. The result is the final accumulated value, which is pushed onto the stack. There needs to be a list, an initial accumulator value, and a code block or operation on the stack before performing this operation.
    - Example:
        - "[ 1 2 3 ] 0 foldl { + }": pushes the list [1, 2, 3], the initial accumulator value 0, and the operation '+' onto the stack, then folds the list by adding each element to the accumulator, resulting in the stack: [ 6 ]

### I/O operations:

- 'print': Takes the top element from the stack and prints it to the standard output. The element is removed from the stack. There needs to be at least one element on the stack before performing this operation and it is only allowed on strings.
    - Example:
        - "" not much left " print ": pushes the string "not much left" onto the stack, then prints it to the standard output. The stack is now empty

- 'read': Reads a line from the standard input and pushes it onto the stack as a string. This operation does not require any elements on the stack before performing it.
    - Example:
        - "read": Waits for the user to input a line of text, and then pushes the entered text onto the stack as a string. If the user entered "hello", the resulting stack would be: [ "hello" ]

### Codeblock operations:

- 'exec': Takes a code block from the top of the stack and executes it. There needs to be a code block on the top of the stack before performing this operation.
    - Example:
        - "{ 2 3 + } exec": pushes the code block {2 3 +} onto the stack, then executes it, which adds 2 and 3 together, and pushes the result (5) onto the stack. The resulting stack would be: [ 5 ]

### Single instruction syntax:

- Single instruction quotations: For operations that accept a code block with a single instruction, you can provide the instruction directly without enclosing it in a code block.
    - Examples:
        - "3 times { 10 }" can be written as "3 times 10": This will push the number 3 onto the stack, then execute the "times" operation with the number 10, pushing the number 10 onto the stack three times. The resulting stack would be: [ 10, 10, 10 ]
        - "10 10 true if { + } { * }" can be written as "10 10 true if + -": This will push the ints 10 and the boolean value 'true' onto the stack, then execute the "if" operation with the "+" and "*" instructions. Since the condition is true, the "+" operation will be executed. The resulting stack will then be [ 20 ]

### Control flow operations

- 'if': The syntax is for the operation is: "boolean if then_block else_block". If the boolean value is true, it executes the then_block, and if false, it executes the else_block. The then and else block needs to be either a codeblock or a single instruction.
    - Example:
        - If the stack already contained the values [true, 5 10], then we could do the syntax "if + -". here the + will be executed since true is the top value on the stack, then the 5 and 10 will be added together resulting in: [ 15 ] 
        - "5 9 10 == if { 10 + } { 10 * }": Here we push 5 onto the stack then one 9 and a 10 the == will check if 9 == 10 which it does not, so the stack is now [false, 5]. Therefore the else_block will be exectued ({ 10 * }), so the stack becomes [ 50 ]

- 'loop': The syntax for the operation is: "loop break_condition block". The loop executes the block until the break_condition evaluates to true. Loop must be provided with 2 codeblocks and does not support single instruction. The break condition evaluating to true or false does not leave that value on the stack (it is consumed by the loop).
    - Example:
        - "1 loop { dup 6 > } { dup 1 + } [ ] 5 times { cons }": will push 1 onto the stack then it duplicates the top element and adds 1 to it, so after one round of the loop the stack is [2, 1] then the next round it will duplicate 2 and add 1 to it so the stack becomes [3, 2, 1] this will continue until the top element on the stack is greater than 6 as indicated by the break block. the resulting stack will be:
        [7, 6, 5, 4, 3, 2, 1]

- 'times': The syntax for the operation is: "Integer times block". The operation repeats the block for the specified number of times. The block needs to be either a code block or a single instruction.
    - Example:
    - If the stack already contained the values [3, 5], then we could do: " times { 2 + }" this will push 2 to the stack and do the + operation 3 times so the resulting stack will be [ 11 ]

### Assignment to symbol operations:

- ':=' : The syntax for the assignment operation is: "symbol value :=". The assignment takes two arguments; the first argument must be a symbol (What we normally call a variable), and the second argument must be a valid literal, for example: integer, list, codeblock and so on. The stack will not be affected by this operation
    - Examples:
    - "x 5 :=": Assigns the integer 5 to the variable x. 
    - "x [ 1, 2, 3, 4, 5,] :=": Assigns the list [ 1, 2, 3, 4, 5 ] to the variable x. 


- 'fun': The syntax for the function definition operation is: "symbol code_block fun". The function definition takes two arguments;the first argument must be a symbol (What we normally call a variable), and the second argument must be a code block.
    - Example:
    - "add_five { 5 + } fun": Defines a function named 'add_five' that adds 5 to the top element of the stack when called. 

- Symbol evaluation: Once a symbol is bound to a function or variable, using that symbol will either run the function or evaluate the variable to a value.
    - Example:
        - If we have declared the add_five function as described above, and a variable x that is assigned the integer 5. Then we can do the input: "x add_five", first 5 is added to the stack and then "5 +" is done because of the add_five function, and the resulting stack is [ 10 ]