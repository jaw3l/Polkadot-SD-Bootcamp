// Concatenate Two Strings
fn concatenate_strings (string1: &str, string2: &str) -> String {
    // Inside the concatenate_strings function, create a new String called result. 
    let mut result = String::new();
    
    // Use the push_str() method to append the contents of the first input string slice, followed by the second input string slice.
    result.push_str(string1);
    result.push_str(string2);
    
    // Return the result string from the function.
    return result
}

fn main() {
    // Create two String variables, string1 and string2, and initialize them with appropriate values.
    let string1 = "Rise";
    let string2 = "In";

    // Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices). 
    // Store the result in a new variable called concatenated_string.
    let concatenated_string = concatenate_strings(string1, string2);

    // Print the concatenated_string variable to the console.
    println!("{}", concatenated_string);
}

