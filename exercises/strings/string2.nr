// strings2.nr
// I AM NOT DONE

fn get_ascii_sum(text: str<5>) -> Field {
    // TODO: Return the sum of ASCII values of all characters
    // Hint: Convert string to bytes and sum them

}

fn main() {
    assert(get_ascii_sum("Hello") == 500); // Sum of ASCII values of "Hello"
}

#[test]
fn test_hex_and_ascii() {
    main();
}
