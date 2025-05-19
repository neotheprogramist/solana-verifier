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
    current: u128,
    result: u128,
}

impl_type_identifiable!(FactorialInternal);

impl FactorialInternal {
    pub fn new(current: u128, result: u128) -> Self {
        Self { current, result }
    }
}

impl Executable for FactorialInternal {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Get the result of the previous multiplication
        let mul_result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
        
        // Remove the result from the stack
        stack.pop_front();

        // Update result
        self.result = mul_result;
        
        // Decrement current number
        self.current -= 1;

        if self.current > 1 {
            // Continue multiplying by creating another Mul task
            vec![Mul::new(self.result, self.current).to_vec_with_type_tag()]
        } else {
            // We're done, push the final result
            stack.push_front(&self.result.to_be_bytes()).unwrap();
            Vec::new()
        }
    }

    fn is_finished(&mut self) -> bool {
        self.current <= 1
    }
}

impl Executable for Factorial {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.n {
            0 | 1 => {
                // 0! = 1! = 1
                stack.push_front(&1u128.to_be_bytes()).unwrap();
                Vec::new()
            }
            n => {
                // For n > 1, start with n * (n-1) and continue multiplying down
                vec![
                    Mul::new(n, n - 1).to_vec_with_type_tag(),
                    FactorialInternal::new(n - 1, 0).to_vec_with_type_tag(),
                ]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        true
    }
} 