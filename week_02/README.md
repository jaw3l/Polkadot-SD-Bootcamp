# Hands On Task #2

## Task Details

Create a simple **calculator** using enums and pattern matching.

## Steps

1. Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
2. Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
3. Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
4. In the main function, prompt the user to input the first number, the operation to be performed, and the second number.
5. Parse the user input into appropriate variables.
6. Create an Operation enum instance with the parsed input values.
7. Call the calculate function with the created Operation enum instance.
8. Print the result to the console.
9. Compile and run the program to ensure it works as expected.

## Checklist

- [x] Define the Operation enum with the appropriate variants and values.
- [x] Write the calculate function signature.
- [x] Implement the calculate function using pattern matching to perform the appropriate arithmetic operation.
- [x] Prompt the user to input the first number, operation, and second number.
- [x] Parse the user input into appropriate variables.
- [x] Create an Operation enum instance with the parsed input values.
- [x] Call the calculate function with the created Operation enum instance.
- [x] Print the result to the console.
- [x] Compile and run the program to test its functionality.

## Output

```text
Please type your first number:
555

Please choose an operation:
1-Add 2-Subtract 3-Multiply 4-Divide
3

Please type your second number:
2332
Result: 1294260
```
