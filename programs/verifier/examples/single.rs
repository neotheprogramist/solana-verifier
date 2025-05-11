use arithmetic::*;
use utils::{BidirectionalStack, Executable, Scheduler};
use verifier::state::BidirectionalStackAccount;

fn main() {
    println!("Dynamic Arithmetic Operations Example");
    println!("====================================");

    // Print a list of all available arithmetic operations
    println!("Available arithmetic operations:");
    println!(
        "1. Add (TYPE_TAG: {}) from arithmetic crate",
        add::Add::TYPE_TAG
    );
    println!(
        "2. Multiply (TYPE_TAG: {}) from arithmetic crate",
        mul::Mul::TYPE_TAG
    );
    println!(
        "3. Multiply Internal (TYPE_TAG: {}) from arithmetic crate",
        mul::MulInternal::TYPE_TAG
    );
    println!(
        "4. Exponentiation (TYPE_TAG: {}) from arithmetic crate",
        exp::Exp::TYPE_TAG
    );
    println!(
        "5. Exponentiation Internal (TYPE_TAG: {}) from arithmetic crate",
        exp::ExpInternal::TYPE_TAG
    );
    println!(
        "6. Fibonacci (TYPE_TAG: {}) from arithmetic crate",
        fib::Fibonacci::TYPE_TAG
    );
    println!(
        "7. Fibonacci Combiner (TYPE_TAG: {}) from arithmetic crate",
        fib::FibonacciCombiner::TYPE_TAG
    );

    // Create a new stack for testing
    let mut stack = BidirectionalStackAccount::default();

    // Push arithmetic tasks to the stack
    stack.push_task(exp::Exp::new(2, 10));

    println!("\nExecuting arithmetic operations from stack:");
    println!("=========================================");

    // Execute each operation
    while !stack.is_empty_back() {
        stack.execute();
        print!(".");
    }
    println!("");

    println!(
        "Result: {:?}",
        u128::from_be_bytes(stack.borrow_front().try_into().unwrap())
    );
    stack.pop_front();

    println!("Stack: {:?}", stack.front_index);
    println!("Stack: {:?}", stack.back_index);
}
