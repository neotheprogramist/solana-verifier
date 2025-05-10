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

    fn compute(&self) -> u128 {
        // Simple exponentiation with overflow protection
        let mut result: u128 = 1;
        for _ in 0..self.exponent {
            result = result.saturating_mul(self.base);
        }
        result
    }
}

impl Executable for Exp {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let result = self.compute();
        println!(
            "Exponentiation: {}^{} = {}",
            self.base, self.exponent, result
        );

        // Convert result to bytes and push to stack
        let result_bytes = result.to_be_bytes().to_vec();
        stack.push_front(&result_bytes).unwrap();

        Vec::new()
    }

    fn is_finished(&mut self) -> bool {
        true
    }
}
