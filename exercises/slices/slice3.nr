// slice3.nr
// use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn transform_slice(input: [Field]) -> [Field] {
    // TODO:
    // 1. Double each element in the slice using map
    // 2. Filter out all elements that are less than 10
    // 3. Return the resulting slice

    // Your code here
}

fn main() {
    let input = &[2, 4, 6, 8];
    let result = transform_slice(input);

    assert(result.len() == 2);
    assert(result[0] == 4);
    assert(result[1] == 8);
}

#[test]
fn test_transformation() {
    main();
}
