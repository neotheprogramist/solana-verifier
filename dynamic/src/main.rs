use animals::bird::Bird;
use animals::cat::Cat;
use animals::traits::Executable as AnimalsExecutable;
use crate::traits::Executable;
use dog::Dog;
use mouse::Mouse;
use utils::BidirectionalStack;
use verifier::state::BidirectionalStackAccount;

pub mod dog;
pub mod mouse;
pub mod traits;

// Include the auto-generated code
include!(concat!(env!("OUT_DIR"), "/executable_dispatch.rs"));

// The `execute` function is now generated in the included file
// Also `push_executable` function is generated to handle serialization

// Define the push_executable function for local types that implement our Executable trait
pub fn push_executable<T: traits::Executable>(stack: &mut BidirectionalStackAccount, executable: T) {
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

fn main() {
    let dog: Dog = Dog::new("Buddy");
    let cat: Cat = Cat::new("Tabby");
    let mouse: Mouse = Mouse::new("Jerry");
    let bird: Bird = Bird::new("Sparrow", true);

    let mut stack = BidirectionalStackAccount::default();

    // Use the specialized functions for different types
    push_cat(&mut stack, cat);
    push_executable(&mut stack, dog);
    push_executable(&mut stack, mouse);
    push_bird(&mut stack, bird);

    // Execute them all using the generated function
    execute(&mut stack);
    execute(&mut stack);
    execute(&mut stack);
    execute(&mut stack);
}
