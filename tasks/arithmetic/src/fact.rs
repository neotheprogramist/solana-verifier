use crate::mul::Mul;
use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Factorial {
    n: u128,
}

impl_type_identifiable!(Factorial);

impl Factorial {
    pub fn new(n: u128) -> Self {
        Self { n }
    }
}

#[repr(C)]
pub struct FactorialInternal {
    result: u128,
    current: u128,
    max: u128,
}

impl_type_identifiable!(FactorialInternal);

impl FactorialInternal {
    pub fn new(result: u128, current: u128, max: u128) -> Self {
        Self { result, current, max }
    }
}

impl Executable for FactorialInternal {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Get the result of the previous multiplication
        let mul_result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());

        // Update internal state
        self.result = mul_result;
        self.current += 1;

        // Remove the result from the stack
        stack.pop_front();

        if self.current <= self.max {
            // Continue multiplying by creating another Mul task
            vec![Mul::new(self.result, self.current).to_vec_with_type_tag()]
        } else {
            // We're done, push the final result
            stack.push_front(&self.result.to_be_bytes()).unwrap();
            Vec::new()
        }
    }

    fn is_finished(&mut self) -> bool {
        self.current > self.max
    }
}

impl Executable for Factorial {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.n == 0 || self.n == 1 {
            // Special case: 0! = 1! = 1
            stack.push_front(&1u128.to_be_bytes()).unwrap();
            Vec::new()
        } else {
            // Create tasks for first multiplication (1 * 2)
            // and tracking factorial progress
            vec![
                Mul::new(1, 2).to_vec_with_type_tag(),
                FactorialInternal::new(1, 2, self.n).to_vec_with_type_tag(),
            ]
        }
    }

    fn is_finished(&mut self) -> bool {
        true // The main Factorial task is finished after creating subtasks
    }
}
