// This exercise will help you understand array methods like map, fold, sort, and predicates

// I AM NOT DONE

fn find_even_numbers(arr: [u64; 6]) -> [u64; 6] {
    // TODO: Return an array where even numbers are kept and odd numbers are replaced with 0
    // Hint: Use the map method
}

fn get_even(num: u64) -> u64 {
  if num % 2 == 0 {
    num
  } else {
  0
  }
}


#[test]
fn test_find_even_numbers() {
    let arr = [1, 2, 3, 4, 5, 6];
    let result = find_even_numbers(arr);
    println(result);
    assert(result == [0, 2, 0, 4, 0, 6]);
}
