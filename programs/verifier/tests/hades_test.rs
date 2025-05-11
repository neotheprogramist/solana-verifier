use stark::felt::Felt;
use stark::poseidon::hades::HadesPermutation;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_hades_permutation() {
    // Initialize a state to test
    let state = [
        Felt::from_hex("0x9").unwrap(),
        Felt::from_hex("0xb").unwrap(),
        Felt::from_hex("0x2").unwrap(),
    ];

    // Create a stack and push the Hades permutation task
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(HadesPermutation::new(state));

    // Execute until completion
    while !stack.is_empty_back() {
        stack.execute();
    }

    // Get the result from the stack
    let bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(bytes);

    // The expected output should match the result we got
    let expected_result =
        Felt::from_hex("0x510f3a3faf4084e3b1e95fd44c30746271b48723f7ea9c8be6a9b6b5408e7e6")
            .unwrap();

    assert_eq!(result, expected_result);
}
