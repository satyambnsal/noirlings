// tuples1.nr
// Use the `hint` watch subcommand for a hint.

// I AM NOT DONE


fn create_person(name: str<10>, age: Field, zip: Field) -> (str<10>,Field,Field) {
    // TODO: Create and return a tuple with the given parameters
}

fn main() {
    let person = create_person("JohhnDOOEE", 30, 12345);

    let (name, age, zip_code) = person;

    assert(age == 30);
    assert(zip_code == 12345);
}

#[test]
fn test_person() {
    main();
}
