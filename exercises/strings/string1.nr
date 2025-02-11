// strings1.nr
// Learn about string declarations and basic assertions
// I AM NOT DONE

fn check_password(password: str<12>) -> bool {
    // TODO: Return true if the password:
    // 1. Is exactly equal to "SuperSecret!"
}

fn main() {
    let password1 = "SuperSecret!";
    
    assert(check_password(password1) == true);
    
    // TODO: Create a string with escape characters that prints:
    // Hello
    //     World!
    // (with one tab space before World)
    let formatted_text = "Hello\n\tWorld!";
    assert(formatted_text == "Hello\n\tWorld!");
}

#[test]
fn test_passwords() {
    main();
}
