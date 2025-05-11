use arithmetic::*;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_single_exponentiation() {
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(exp::Exp::new(2, 10));

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
    assert_eq!(result, 1024);
    stack.pop_front();

    // Verify stack is empty
    assert_eq!(stack.front_index, 0);
}
