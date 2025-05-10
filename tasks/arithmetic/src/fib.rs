use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Fibonacci {
    n: u32,
}

impl_type_identifiable!(Fibonacci);

impl Fibonacci {
    pub fn new(n: u32) -> Self {
        Self { n }
    }
}

#[repr(C)]
pub struct FibonacciCombiner {
    n: u32,
}

impl_type_identifiable!(FibonacciCombiner);

impl FibonacciCombiner {
    pub fn new(n: u32) -> Self {
        Self { n }
    }
}

impl Executable for Fibonacci {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.n {
            0 => {
                // Base case: F(0) = 0
                stack.push_front(&0u128.to_be_bytes()).unwrap();
                Vec::new()
            }
            1 => {
                // Base case: F(1) = 1
                stack.push_front(&1u128.to_be_bytes()).unwrap();
                Vec::new()
            }
            n => {
                // Recursive case: F(n) = F(n-1) + F(n-2)
                vec![
                    Fibonacci::new(n - 1).to_vec_with_type_tag(),
                    Fibonacci::new(n - 2).to_vec_with_type_tag(),
                    FibonacciCombiner::new(n).to_vec_with_type_tag(),
                ]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        true // The main Fibonacci task is finished after creating subtasks
    }
}

impl Executable for FibonacciCombiner {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Pop F(n-2) and F(n-1) from the stack
        let fib_n_2 = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let fib_n_1 = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        // Compute F(n) = F(n-1) + F(n-2)
        let result = fib_n_1.saturating_add(fib_n_2);

        // Push the result back to the stack
        stack.push_front(&result.to_be_bytes()).unwrap();

        Vec::new()
    }

    fn is_finished(&mut self) -> bool {
        true
    }
}
