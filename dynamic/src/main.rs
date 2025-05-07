use cat::Cat;
use dog::Dog;
use traits::Executable;
use utils::BidirectionalStack;
use verifier::state::BidirectionalStackAccount;

pub mod cat;
pub mod dog;
pub mod traits;

fn execute(stack: &mut BidirectionalStackAccount) {
    let data = stack.borrow_mut_front();
    match data[0] {
        Dog::TYPE_TAG => {
            let dog = Dog::cast_mut(&mut data[1..]);
            dog.execute();
        }
        Cat::TYPE_TAG => {
            let cat = Cat::cast_mut(&mut data[1..]);
            cat.execute();
        }
        _ => {
            panic!("Unknown tag: {}", data[0]);
        }
    }
    stack.pop_front();
}

fn main() {
    let dog: Dog = Dog::new("Buddy");
    let cat = Cat::new("Tabby");

    let mut stack = BidirectionalStackAccount::default();

    // Push dog to the stack
    let mut serialized_dog = Vec::new();
    serialized_dog.push(Dog::TYPE_TAG);
    serialized_dog.extend_from_slice(dog.as_bytes());
    stack.push_front(&serialized_dog).unwrap();

    // Push cat to the stack
    let mut serialized_cat = Vec::new();
    serialized_cat.push(Cat::TYPE_TAG);
    serialized_cat.extend_from_slice(cat.as_bytes());
    stack.push_front(&serialized_cat).unwrap();

    // Retrieve and execute
    execute(&mut stack);

    execute(&mut stack);
}
