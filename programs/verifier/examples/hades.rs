use stark::felt::Felt;
use stark::poseidon::hades::HadesPermutation;
use utils::{BidirectionalStack, Executable, Scheduler};
use verifier::state::BidirectionalStackAccount;

fn main() {
    println!("Hades Permutation Example");
    println!("=======================");

    println!(
        "HadesTask (TYPE_TAG: {}) from stark crate",
        HadesPermutation::TYPE_TAG
    );

    // Create a new stack for testing
    let mut stack = BidirectionalStackAccount::default();

    // Create initial state for Hades permutation
    // Example values - you would typically use specific field elements
    let state = [
        Felt::from_hex("0x9").unwrap(),
        Felt::from_hex("0xb").unwrap(),
        Felt::from_hex("0x2").unwrap(),
    ];

    // Push a HadesTask to the stack
    stack.push_task(HadesPermutation::new(state));

    println!("\nExecuting Hades permutation task:");
    println!("==============================");

    // Execute each operation
    let mut steps = 0;
    while !stack.is_empty_back() {
        println!("Stack front index: {:?}", stack.front_index);
        println!("Stack back index: {:?}", 65536 - stack.back_index);
        stack.execute();
        print!(".");
        steps += 1;
        if steps % 50 == 0 {
            println!(" {}", steps);
        }
    }
    println!("\nTotal steps: {}", steps);

    println!("Stack front index: {:?}", stack.front_index);
    println!("Stack back index: {:?}", 65536 - stack.back_index);

    let bytes = stack.borrow_front();
    let felt = Felt::from_bytes_be(&bytes.try_into().unwrap());
    println!("Result: {}", felt);
    stack.pop_front();

    println!("Stack front index: {:?}", stack.front_index);
    println!("Stack back index: {:?}", stack.back_index);
}
