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
