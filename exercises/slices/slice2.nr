// slice2.nr
// use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn process_slice(mut slice: [Field]) -> Field {
    // TODO:
    // 1. Insert value 42 at index 2
    // 2. Remove the element at index 0
    // 3. Return the length of the final slice

    // Your code here
}

fn main() {
    let mut initial_slice: [Field] = &[1, 2, 3, 4];
    let result = process_slice(initial_slice);

    assert(result == 4); // Length should be 4 after operations
}

#[test]
fn test_slice_methods() {
    main();
}
