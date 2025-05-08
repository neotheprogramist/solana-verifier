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
    
    fn compute(&self) -> u128 {
        if self.n <= 1 {
            return self.n as u128;
        }
        
        // Calculate Fibonacci with overflow protection
        let mut a: u128 = 0;
        let mut b: u128 = 1;
        
        for _ in 2..=self.n {
            let next = a.saturating_add(b);
            a = b;
            b = next;
        }
        
        b
    }
}

impl Executable for Fibonacci {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let result = self.compute();
        println!("Fibonacci({}) = {}", self.n, result);
        
        // Convert result to bytes and push to stack
        let result_bytes = result.to_le_bytes().to_vec();
        stack.push_front(&result_bytes).unwrap();
        
        Vec::new()
    }
    
    fn is_finished(&mut self) -> bool {
        true
    }
} 