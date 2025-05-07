use bird::Bird;
use cat::Cat;
use dog::Dog;
use mouse::Mouse;
use utils::BidirectionalStack;
use verifier::state::BidirectionalStackAccount;

pub mod bird;
pub mod cat;
pub mod dog;
pub mod mouse;
pub mod traits;

// Include the auto-generated code
include!(concat!(env!("OUT_DIR"), "/executable_dispatch.rs"));

// The `execute` function is now generated in the included file
// Also `push_executable` function is generated to handle serialization

fn main() {
    let dog: Dog = Dog::new("Buddy");
    let cat: Cat = Cat::new("Tabby");
    let mouse: Mouse = Mouse::new("Jerry");
    let bird: Bird = Bird::new("Sparrow", true);

    let mut stack = BidirectionalStackAccount::default();

    // Use the generated helper function - notice we can add new types
    // without modifying the execution logic
    push_executable(&mut stack, cat);
    push_executable(&mut stack, dog);
    push_executable(&mut stack, mouse);
    push_executable(&mut stack, bird);

    // Execute them all using the generated function
    execute(&mut stack);
    execute(&mut stack);
    execute(&mut stack);
    execute(&mut stack);
}
