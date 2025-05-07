use animals::bird::Bird;
use animals::cat::Cat;
use animals::frog::Frog;
use dog::Dog;
use mouse::Mouse;
use utils::BidirectionalStack;
use utils::Executable;
use verifier::state::BidirectionalStackAccount;

pub mod dog;
pub mod mouse;

// Include the auto-generated code
include!(concat!(env!("OUT_DIR"), "/executable_dispatch.rs"));

fn main() {
    println!("Dynamic Executable Type System Test");
    println!("===================================");
    
    // Print a list of all found executable types
    println!("Found the following executable types:");
    println!("1. Dog (TYPE_TAG: {}) from dynamic crate", Dog::TYPE_TAG);
    println!("2. Mouse (TYPE_TAG: {}) from dynamic crate", Mouse::TYPE_TAG);
    println!("3. Cat (TYPE_TAG: {}) from animals crate", Cat::TYPE_TAG); 
    println!("4. Bird (TYPE_TAG: {}) from animals crate", Bird::TYPE_TAG);
    println!("5. Frog (TYPE_TAG: {}) from animals crate", Frog::TYPE_TAG);
    
    // Create a new stack for testing
    let mut stack = BidirectionalStackAccount::default();
    
    // Create and push some executable types
    push_executable(&mut stack, Dog::new("Rex"));
    push_executable(&mut stack, Mouse::new("Jerry"));
    push_cat(&mut stack, Cat::new("Black"));
    push_bird(&mut stack, Bird::new("Eagle", true));
    push_frog(&mut stack, Frog::new("Kermit", false));
    
    println!("\nExecuting types from stack:");
    println!("===========================");
    
    // Execute each type
    while !stack.is_empty_front() {
        execute(&mut stack);
    }
}

// Define the push_executable function for local types that implement our Executable trait
pub fn push_executable<T: Executable>(stack: &mut BidirectionalStackAccount, executable: T) {
    let mut serialized = Vec::new();
    serialized.push(T::TYPE_TAG);
    serialized.extend_from_slice(executable.as_bytes());
    stack.push_front(&serialized).unwrap();
}

// Define specialized functions for animals crate types
pub fn push_cat(stack: &mut BidirectionalStackAccount, cat: Cat) {
    let mut serialized = Vec::new();
    serialized.push(Cat::TYPE_TAG);
    serialized.extend_from_slice(cat.as_bytes());
    stack.push_front(&serialized).unwrap();
}

pub fn push_bird(stack: &mut BidirectionalStackAccount, bird: Bird) {
    let mut serialized = Vec::new();
    serialized.push(Bird::TYPE_TAG);
    serialized.extend_from_slice(bird.as_bytes());
    stack.push_front(&serialized).unwrap();
}

pub fn push_frog(stack: &mut BidirectionalStackAccount, frog: Frog) {
    let mut serialized = Vec::new();
    serialized.push(Frog::TYPE_TAG);
    serialized.extend_from_slice(frog.as_bytes());
    stack.push_front(&serialized).unwrap();
}
