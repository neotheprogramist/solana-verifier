use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Add {
    x: u128,
    y: u128,
}

impl_type_identifiable!(Add);

impl Add {
    pub fn new(x: u128, y: u128) -> Self {
        Self { x, y }
    }
    
    fn compute(&self) -> u128 {
        self.x.saturating_add(self.y)
    }
}

impl Executable for Add {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let result = self.compute();
        println!("Addition: {} + {} = {}", self.x, self.y, result);
        
        // Convert result to bytes and push to stack
        let result_bytes = result.to_le_bytes().to_vec();
        stack.push_front(&result_bytes).unwrap();
        
        Vec::new()
    }
    
    fn is_finished(&mut self) -> bool {
        true
    }
} 