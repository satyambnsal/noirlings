// variables1.nr
// Execute `noirlings hint variables1` or use the `hint` watch subcommand for a hint

// Change `can_vote` method to make test cases pass.

// I AM NOT DONE



fn can_vote() -> bool{
  let age = 15; // Change this line
  let mut can_vote: bool = false;

  if age >= 18 {
    can_vote = true;
  } else {
    can_vote = false;
  }
  can_vote
}

#[test]
fn test_should_vote() {
  assert(can_vote());
}
