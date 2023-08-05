use std::io;

// Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
fn calculate(operation: Operation) -> f64 {
    // Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    // In the main function, prompt the user to input the first number, the operation to be performed, and the second number.
    println!("Please type your first number:");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Error while reading first number!");

    // Parse the user input into appropriate variables.
    let first_number: f64 = first_number
        .trim()
        .parse()
        .expect("Error while parsing first number!");

    println!("\nPlease choose an operation:");
    println!("1-Add 2-Subtract 3-Multiply 4-Divide");

    // Match the operation input to the appropriate Operation enum variant.
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Error while reading operation!");
    let operation: u32 = operation
        .trim()
        .parse()
        .expect("Error while parsing operation!");

    println!("\nPlease type your second number:");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error while reading second number!");
    let second_number: f64 = second_number
        .trim()
        .parse()
        .expect("Error while parsing second number!");

    // Create an Operation enum instance with the parsed input values.
    let operation_enum = match operation {
        1 => Operation::Add(first_number, second_number),
        2 => Operation::Subtract(first_number, second_number),
        3 => Operation::Multiply(first_number, second_number),
        4 => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operation!"),
    };

    // Call the calculate function with the created Operation enum instance.
    let result = calculate(operation_enum);

    // Print the result to the console.
    println!("Result: {result}");
}
