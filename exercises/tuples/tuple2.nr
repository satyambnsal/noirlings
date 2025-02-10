// tuples2.nr
// Use the `hint` watch subcommand for a hint.

// I AM NOT DONE



fn create_coordinates(x: Field, y: Field, z: Field) ->  (Field,Field,Field){

}

fn main() {
    let coords = create_coordinates(10, 20, 30);

    let x = coords.0;
    let y = coords.1;
    let z = coords.2;

    assert(x == 10);
    assert(y == 20);
    assert(z == 30);
}

#[test]
fn test_coordinates() {
    main();
}
