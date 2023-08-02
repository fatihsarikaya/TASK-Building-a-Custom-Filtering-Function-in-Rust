//cargo new my_project
//cd my_project


struct FilterCondition {
    condition: i32,
}

impl FilterCondition {
    // Implement the is_match method
    fn is_match(&self, item: &i32) -> bool {
        *item == self.condition
    }
}


fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_items = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            filtered_items.push(*item);
        }
    }

    filtered_items
}


fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_condition = FilterCondition { condition: 5 };
    let filtered_result = custom_filter(&data, &filter_condition);

    // Print the filtered result to the console
    println!("Filtered Result: {:?}", filtered_result);
}

// To compile and run the program
//cargo run

// Output:
//Filtered Result: [5]