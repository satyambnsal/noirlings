// slice4.nr
// use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Stats {
    sum: Field,
    count: Field,
}

fn calculate_average(numbers: [Field]) -> Field {
    // TODO:
    // 1. Use fold to accumulate both sum and count of elements
    // 2. Calculate and return the average
    // Hint: Create a Stats struct to hold intermediate values
    // Your code here
}

fn main() {
    let numbers = &[2, 4, 6, 8, 10];
    let average = calculate_average(numbers);

    assert(average == 6);
}

#[test]
fn test_reduction() {
    main();
}

//PS: This is though one. don't hesitate using hint😄
