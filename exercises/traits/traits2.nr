// traits2.nr
// use the `hint` watch subcommand for a hint.
// Multiple Trait Implementation: This exercise shows how to implement multiple traits for a type

// I AM NOT DONE


trait Perimeter {
    fn perimeter(self) -> Field;
}

trait Area {
    fn area(self) -> Field;
}

struct Circle {
    radius: Field,
}

// TODO: Implement both Area and Perimeter traits for Circle
// Note: Use 3 as an approximation for PI
impl Area for Circle {
    // Add your implementation here
}

impl Perimeter for Circle {
    // Add your implementation here
}

#[test]
fn test_circle_calculations() {
    let circle = Circle { radius: 2 };
    assert(circle.area() == 12); // Using PI ~ 3
    assert(circle.perimeter() == 12); // Using PI ~ 3
}
