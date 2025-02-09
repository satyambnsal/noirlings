// Noir is an open-source DSL(Domain specific language) for construction of privacy-preserving zk programs.

// ZK programs are programs that can generate short proof of statements without revealing all inputs to the statements.


// Welcome to Noirlings
// This exercise doesn't do anything yet but it still compiles! Noir file getting run
// needs to have a `main` function. So this file is a valid Noir file.
// Other exercises will require you to write Noir code to make the exercise file compile.
// Remove "I AM NOT DONE" comment when you are ready to move to the next exercise.


// I AM NOT DONE

fn main(x : Field, y : pub Field) {
    assert(x != y);
}


#[test]
fn test_main(){
    let a: Field = 2;
    let b: Field = 5;
    main(a,b);   
}

#[test(should_fail)]
fn test_failure() {
    main(2,2);
}
