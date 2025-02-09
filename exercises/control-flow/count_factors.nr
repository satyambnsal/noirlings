// Complete the count_factors function that counts the number of factors
// of a given number, and find_special_numbers that finds numbers with exactly
// k factors within a range

// I AM NOT DONE

fn count_factors(n: u64) -> u64 {
    let mut count = 0;
    for i in 1..n + 1 {
        //TODO: complete code
        // Remember: A number is a factor if n % factor == 0
    }

    count
}

#[test]
fn test_count_factors() {
    let result = count_factors(12);
    println(f"factor {result}");
    assert(count_factors(12) == 6); // Factors: 1, 2, 3, 4, 6, 12
    assert(count_factors(7) == 2); // Factors: 1, 7
    assert(count_factors(16) == 5); // Factors: 1, 2, 4, 8, 16
}
