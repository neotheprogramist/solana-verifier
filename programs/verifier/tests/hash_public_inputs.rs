use stark::stark_proof::HashPublicInputs;
use stark::{felt::Felt, poseidon::PoseidonHashMany};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn hash_public_inputs_basic() {
    let a = Felt::from_hex("0x1").unwrap();
    let b = Felt::from_hex("0x2").unwrap();
    let c = Felt::from_hex("0x3").unwrap();
    let d = Felt::from_hex("0x4").unwrap();
    let program = vec![a, b, c, d];
    let output = vec![a, b, c];
    let expected_program_hash =
        Felt::from_hex("0x26e3ad8b876e02bc8a4fc43dad40a8f81a6384083cabffa190bcf40d512ae1d")
            .unwrap();
    let expected_output_hash = Felt::from_hex_unchecked(
        "0x2f0d8840bcf3bc629598d8a6cc80cb7c0d9e52d93dab244bbf9cd0dca0ad082",
    );
    test_hash_with_inputs(
        &program,
        &output,
        expected_program_hash,
        expected_output_hash,
    );
}

fn test_hash_with_inputs(
    program: &[Felt],
    output: &[Felt],
    expected_program_hash: Felt,
    expected_output_hash: Felt,
) {
    let mut stack = BidirectionalStackAccount::default();
    let hash_task = HashPublicInputs::new(program.len(), output.len());
    PoseidonHashMany::push_input(output, &mut stack);
    PoseidonHashMany::push_input(program, &mut stack);

    stack.push_task(hash_task);
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    assert_eq!(result_program_hash, expected_program_hash);
    assert_eq!(result_output_hash, expected_output_hash);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}
