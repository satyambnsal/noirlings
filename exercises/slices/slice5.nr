// slice5.nr
// use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Transaction {
    amount: Field,
    valid: bool,
}

fn process_transactions(transactions: [Transaction]) -> Field {
    // TODO:
    // 1. Filter out invalid transactions
    // 2. Extract only the amounts using map
    // 3. Calculate the total using reduce
    // 4. Return the final sum

    // Your code here

    sum
}

fn main() {
    let transactions = &[
        Transaction { amount: 100, valid: true },
        Transaction { amount: 50, valid: false },
        Transaction { amount: 75, valid: true },
        Transaction { amount: 200, valid: true },
    ];

    let total = process_transactions(transactions);
    assert(total == 375); // Sum of valid transactions
}

#[test]
fn test_advanced_operations() {
    main();
}
