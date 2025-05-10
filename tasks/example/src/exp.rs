use crate::mul::Mul;
use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Exp {
    base: u128,
    exponent: u32,
}

impl_type_identifiable!(Exp);

impl Exp {
    pub fn new(base: u128, exponent: u32) -> Self {
        Self { base, exponent }
    }
}

#[repr(C)]
pub struct ExpInternal {
    base: u128,
    exponent: u32,
    result: u128,
    counter: u32,
}

impl_type_identifiable!(ExpInternal);

impl ExpInternal {
    pub fn new(base: u128, exponent: u32, result: u128, counter: u32) -> Self {
        Self {
            base,
            exponent,
            result,
            counter,
        }
    }
}

impl Executable for ExpInternal {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Get the result of the previous multiplication
        let mul_result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());

        // Update internal state
        self.counter += 1;
        self.result = mul_result;

        // Remove the result from the stack
        stack.pop_front();

        if self.counter < self.exponent {
            // Continue multiplying by creating another Mul task
            vec![Mul::new(self.result, self.base).to_vec_with_type_tag()]
        } else {
            // We're done, push the final result
            stack.push_front(&self.result.to_be_bytes()).unwrap();
            Vec::new()
        }
    }

    fn is_finished(&mut self) -> bool {
        self.counter >= self.exponent
    }
}

impl Executable for Exp {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.exponent == 0 {
            // Special case: any number raised to 0 is 1
            stack.push_front(&1u128.to_be_bytes()).unwrap();
            Vec::new()
        } else {
            // Create tasks for first multiplication and tracking exponentiation progress
            vec![
                Mul::new(1, self.base).to_vec_with_type_tag(),
                ExpInternal::new(self.base, self.exponent, self.base, 0).to_vec_with_type_tag(),
            ]
        }
    }

    fn is_finished(&mut self) -> bool {
        true // The main Exp task is finished after creating subtasks
    }
}
