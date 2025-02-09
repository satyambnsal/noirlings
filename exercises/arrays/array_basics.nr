// This exercise will help you understand array creation, indexing, and basic operations

// I AM NOT DONE

fn create_array() -> [u64; 5] {
    // TODO: Create and return an array with values [1, 2, 3, 4, 5]
    [1, 2, 3, 4, 5]
}

fn sum_array(arr: [u64; 5]) -> u64 {
    // TODO: Return the sum of all elements in the array
    // Hint: You can use a for loop or the fold method

}

fn double_elements(arr: [u64; 4]) -> [u64; 4] {
    // TODO: Return a new array with each element doubled
    // Hint: You can use the map method
}

fn fill_array(value: Field) -> [Field; 3] {
    // TODO: Create and return an array of size 3 filled with the given value
}

#[test]
fn test_create_array() {
    let arr = create_array();
    assert(arr[0] == 1);
    assert(arr[1] == 2);
    assert(arr[2] == 3);
    assert(arr[3] == 4);
    assert(arr[4] == 5);
}

#[test]
fn test_sum_array() {
    let arr = [1, 2, 3, 4, 5];
    assert(sum_array(arr) == 15);

    let arr2 = [10, 20, 30, 40, 50];
    assert(sum_array(arr2) == 150);
}

#[test]
fn test_double_elements() {
    let arr = [1, 2, 3, 4];
    let doubled = double_elements(arr);
    assert(doubled == [2, 4, 6, 8]);
}

#[test]
fn test_fill_array() {
    let arr = fill_array(42);
    assert(arr[0] == 42);
    assert(arr[1] == 42);
    assert(arr[2] == 42);
    assert(arr.len() == 3);
}
