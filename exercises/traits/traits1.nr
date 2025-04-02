// traits1.nr
// use the `hint` watch subcommand for a hint.


// Basic Trait Implementation
// This exercise introduces basic trait definition and implementation

// I AM NOT DONE

// A trait for shapes that can calculate their area
trait Shape {
    fn area(self) -> Field;
}

struct Rectangle {
    width: Field,
    height: Field,
}

// The area should be calculated as width * height
impl Shape for Rectangle {
    //TODO: Complete `area` method implementation
}

#[test]
fn test_rectangle_area() {
    let rect = Rectangle { width: 5, height: 3 };
    assert(rect.area() == 15);
}
