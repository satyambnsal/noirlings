
// pedersen_hash.nr: Understanding Pedersen Hash
// 
// Pedersen hash is a cryptographic hash function based on elliptic curve operations.
// It is widely used in zero-knowledge proofs and privacy-preserving blockchain applications.

// Key Properties of Pedersen hash:
// 1. It's a one-way function(difficult to reverse)
// 2. It has collision resistance(hard to find two inputs that hash to the same output)
// 3. Its deterministic(same input always produces the same output)
// 4. It has homomorphic properties(especially useful in zero-knowledge proofs)


// In this exercise, you will:
// 1. Compute a basic Pedersen hash
// 2. Verify its deterministic property
// 3. Explore how changes to input affect the output (avalanche effect)

// This function computes the Pedersen hash of two field elements
fn compute_pedersen_hash(x: Field, y: Field) -> Field {
    // TODO: Implement this function using the Noir standard library
    // HINT: Use std::hash::pedersen_hash([x, y]);
    // Replace this with your implementation
}

// This function tests if changing one input value creates a completely different hash
fn test_avalanche_effect(x: Field, y: Field, modified_x: Field) -> bool {
    let hash1 = std::hash::pedersen_hash([x,y]);
    let new_hash = std::hash::pedersen_hash([modified_x, y]);
    hash1 != new_hash
}

fn main(x: Field, y: Field, expected_hash: Field) -> pub bool {
    let hash = compute_pedersen_hash(x, y);
    
    // Test deterministic property - same inputs always give same output
    let hash2 = compute_pedersen_hash(x, y);
    let deterministic = hash == hash2;
    
    let correct_hash = hash == expected_hash;
    
    // Test avalanche effect with a slightly modified x value
    let modified_x = x + 1;
    let has_avalanche = test_avalanche_effect(x, y, modified_x);
    
    // Return true only if all tests pass
    deterministic & correct_hash & has_avalanche
}

#[test]
fn test_pedersen_hash_1() {
    let x1 = 1;
    let y1 = 2;
    let expected1 = std::hash::pedersen_hash([x1, y1]); 
    assert(main(x1, y1, expected1));
    
    let x2 = 123456789;
    let y2 = 987654321;
    let expected2 = std::hash::pedersen_hash([x2, y2]);
    assert(main(x2, y2, expected2));
}

#[test]
fn test_string_hash() {
  let message = "Noir Gud";
  let arr = message.as_bytes().map(|charCode| charCode as Field);
  let expected_hash: Field = 0x23176ca31135ed3307b6dc2206fd3471f844d15889a8c1821b24b62049bb4832;
  let hash = std::hash::pedersen_hash(arr);
  assert(hash == expected_hash, "hash should match");
}
