// embedded_curve1.nr
// Introduction to EmbeddedCurvePoint structure
// Use the `hint` watch subcommand for a hint.

// TODO: Define a struct called 'EmbeddedCurvePoint' with three fields:
// - x: Field (x-coordinate)
// - y: Field (y-coordinate)
// - is_infinite: bool (flag indicating if it's the point at infinity)


// I AM NOT DONE

fn main() {
    // Create a regular point
    let point = EmbeddedCurvePoint {
        x: 1,
        y: 2,
        is_infinite: false,
    };
    
    // Create a point at infinity
    let infinity = EmbeddedCurvePoint {
        x: 0, 
        y: 0,
        is_infinite: true,
    };
    
    // Test regular point
    assert(point.x == 1);
    assert(point.y == 2);
    assert(!point.is_infinite);
    
    // Test point at infinity
    assert(infinity.is_infinite);
}

#[test]
fn test_embedded_curve_point() {
    main();
}
