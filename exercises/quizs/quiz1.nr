// quiz1.nr
// No Hint this time. Perhaps discuss with other devs in Noir discord

// I AM NOT DONE

// Input 3 values using 'a'(array of length 3) and check if they all are equal.
// Return using signal 'c'.
fn equality(inputs: [u32;3]) -> bool {
 //TODO
}

#[test]
fn test_equality_true() {
  let result = equality([7,7,7]);
  assert(result == true, "Should be true")
}


#[test]
fn test_equality_false() {
  let result = equality([1,1,2]);
  println("h3llo");
  assert(result == false, "Should be false")
}
