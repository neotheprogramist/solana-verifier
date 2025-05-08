use dynamic::*;
use utils::{BidirectionalStack, Executable, Scheduler};
use verifier::state::BidirectionalStackAccount;

fn main() {
    println!("Dynamic Executable Type System Test");
    println!("===================================");

    // Print a list of all found executable types
    println!("Found the following executable types:");
    println!("1. Dog (TYPE_TAG: {}) from dog crate", dog::Dog::TYPE_TAG);
    println!(
        "2. Mouse (TYPE_TAG: {}) from dynamic crate",
        mouse::Mouse::TYPE_TAG
    );
    println!("3. Cat (TYPE_TAG: {}) from cat crate", cat::Cat::TYPE_TAG);
    println!(
        "4. Bird (TYPE_TAG: {}) from bird crate",
        bird::Bird::TYPE_TAG
    );
    println!(
        "5. Frog (TYPE_TAG: {}) from frog crate",
        frog::Frog::TYPE_TAG
    );

    // Create a new stack for testing
    let mut stack = BidirectionalStackAccount::default();

    stack.push_task(dog::Dog::new("Rex"));
    stack.push_task(mouse::Mouse::new("Jerry"));
    stack.push_task(cat::Cat::new("Black"));
    stack.push_task(bird::Bird::new("Eagle", true));
    stack.push_task(frog::Frog::new("Kermit", false));

    println!("\nExecuting types from stack:");
    println!("===========================");

    // Execute each type
    while !stack.is_empty_back() {
        stack.execute();
    }
}
