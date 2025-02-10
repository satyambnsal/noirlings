// structs1.nr
// use the `hint` watch subcommand for a hint.

// TODO: Define a struct called 'Point' with two fields:
// - x: Field
// - y: Field

// I AM NOT DONE

fn main() {
    let point = Point {
        x: 5,
        y: 10,
    };
    
    assert(point.x == 5);
    assert(point.y == 10);
}

#[test]
fn test_point() {
    main();
}
