use example::*;
use utils::{BidirectionalStack, Executable, Scheduler};
use verifier::state::BidirectionalStackAccount;

fn main() {
    println!("Dynamic Arithmetic Operations Example");
    println!("====================================");

    // Print a list of all available arithmetic operations
    println!("Available arithmetic operations:");
    println!(
        "1. Add (TYPE_TAG: {}) from arithmetic_example crate",
        add::Add::TYPE_TAG
    );
    println!(
        "2. Multiply (TYPE_TAG: {}) from arithmetic_example crate",
        mul::Mul::TYPE_TAG
    );
    println!(
        "3. Exponentiation (TYPE_TAG: {}) from arithmetic_example crate",
        exp::Exp::TYPE_TAG
    );
    println!(
        "4. Fibonacci (TYPE_TAG: {}) from arithmetic_example crate",
        fib::Fibonacci::TYPE_TAG
    );

    // Create a new stack for testing
    let mut stack = BidirectionalStackAccount::default();

    // Push arithmetic tasks to the stack
    // stack.push_task(add::Add::new(10, 25));
    stack.push_task(mul::Mul::new(5, 7));
    // stack.push_task(exp::Exp::new(2, 8));
    // stack.push_task(fib::Fibonacci::new(10));

    println!("\nExecuting arithmetic operations from stack:");
    println!("=========================================");

    // Execute each operation
    while !stack.is_empty_back() {
        stack.execute();
    }
}
