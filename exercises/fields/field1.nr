// field1.nr
// use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// For Noir default backend, Grumpkin curve, order of the field value P is 21888242871839275222246405745257275088548364400416034343698204186575808495617


fn main() {
  let p: Field = 21888242871839275222246405745257275088548364400416034343698204186575808495616;

  let p1: Field = p + 1;

  // TODO: replace ? with correct p1 value
  assert(p1 == 0, "incorrect p1 value");

  let p2: Field = p * p;
  // TODO: replace ? with correct p1 value
  assert(p2 == ?, "incorrect p2 value");
}


