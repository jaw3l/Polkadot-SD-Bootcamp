// Define the FilterCondition struct with the desired type for filtering.
struct FilterCondition <T>{
    filter: T,
}
// Implement the is_match method on the FilterCondition struct.
impl <T: PartialOrd> FilterCondition <T>{
    fn is_match(&self, item: &T) -> bool {
        return item > &self.filter
    }
}

// Define the custom_filter function to filter elements based on the condition.
fn custom_filter <T> (collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T> where T: PartialOrd{
    return collection.into_iter().filter(|item: &T| condition.is_match(item)).collect();
}

fn main() {
    // Create a collection and a FilterCondition object in the main function.
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let condition: FilterCondition<i32> = FilterCondition { filter: 3 };
    
    // Call the custom_filter function and store the result.
    let filtered_numbers: Vec<i32> = custom_filter(numbers, &condition);
    
    // Print the filtered result to the console.
    println!("Filtered Numbers: {:?}", filtered_numbers);
}