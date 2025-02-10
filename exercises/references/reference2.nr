// reference2.nr
// use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Counter {
    value: Field,
    step: Field,
}

impl Counter {
    fn new(initial: Field, step: Field) -> Self {
        Counter { value: initial, step }
    }

    // TODO: Implement the increment method that takes a mutable reference to self
    // and increases the value by step
    fn increment(&mut self) {
        // Your implementation here
    }

    // TODO: Implement the update_step method that takes a mutable reference to self
    // and updates the step value
    fn update_step(&mut self, new_step: Field) {
        // Your implementation here
    }
}

fn main() {
    let mut counter = Counter::new(0, 2);

    // Test increment
    counter.increment();
    assert(counter.value == 2);

    counter.increment();
    assert(counter.value == 4);

    // Test update_step
    counter.update_step(3);
    counter.increment();
    assert(counter.value == 7);
}

#[test]
fn test_counter() {
    main();
}
