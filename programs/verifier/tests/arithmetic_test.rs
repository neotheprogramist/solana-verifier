use arithmetic::*;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_add_operation() {
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(add::Add::new(48, 52));

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
    assert_eq!(result, 100);
    stack.pop_front();
}

#[test]
fn test_multiply_operation() {
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(mul::Mul::new(5, 7));

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
    assert_eq!(result, 35);
    stack.pop_front();
}

#[test]
fn test_factorial_operation() {
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(factorial::Factorial::new(5));

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
    assert_eq!(result, 120); // 5! = 5 * 4 * 3 * 2 * 1 = 120
    stack.pop_front();
}

#[test]
fn test_exponentiation_operation() {
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(exp::Exp::new(2, 10));

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
    assert_eq!(result, 1024);
    stack.pop_front();
}

#[test]
fn test_fibonacci_operation() {
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(fib::Fibonacci::new(19));

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
    assert_eq!(result, 4181);
    stack.pop_front();
}
