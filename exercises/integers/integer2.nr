// integers2.nr
// Learn about signed and unsigned integers with temperature conversion
// I AM NOT DONE

fn celsius_to_fahrenheit(celsius: i8) -> i16 {
    // TODO: Convert Celsius to Fahrenheit
    // Formula: (celsius * 9/5) + 32
    // Handle both positive and negative temperatures
    // Return i16 as fahrenheit can exceed i8 range
    0
}

fn is_freezing(temps: [i8; 3]) -> [bool; 3] {
    // TODO: Return array of booleans indicating if each temperature is below freezing (0°C)
    [false; 3]
}

fn main() {
    // Test temperature conversion
    assert(celsius_to_fahrenheit(0) == 32);
    assert(celsius_to_fahrenheit(-40) == -40);
    assert(celsius_to_fahrenheit(100) == 212);
    
    // Test freezing temperatures
    let temps = [-5, 0, 20];
    let freezing = is_freezing(temps);
    assert(freezing == [true, false, false]);
}

#[test]
fn test_temperature() {
    main();
}
