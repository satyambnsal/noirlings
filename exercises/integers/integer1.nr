// integer1.nr
// I AM NOT DONE
use std::{wrapping_add,wrapping_mul};

fn calculate_game_score(kills: u8, deaths: u8, assists: u8) -> u8 {
    // TODO: Calculate player score using formula:
    // score = (kills * 2) + assists - deaths
    // Use wrapping_add and wrapping_sub to handle overflow
    // If deaths are more than kills*2 + assists, return 0
}

fn main() {
    // Normal case
    assert(calculate_game_score(10, 5, 3) == 18);
    
    assert(calculate_game_score(255, 1, 1) == 254); // Should wrap around
    
    // Deaths > kills*2 + assists
    assert(calculate_game_score(1, 10, 2) == 0);
}

#[test]
fn test_game_score() {
    main();
}
