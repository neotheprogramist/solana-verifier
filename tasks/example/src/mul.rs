use crate::add::Add;
use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Mul {
    x: u128,
    y: u128,
}

impl_type_identifiable!(Mul);

impl Mul {
    pub fn new(x: u128, y: u128) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
pub struct MulInternal {
    x: u128,
    y: u128,
    result: u128,
    counter: u128,
}

impl_type_identifiable!(MulInternal);

impl MulInternal {
    pub fn new(x: u128, y: u128, result: u128, counter: u128) -> Self {
        Self {
            x,
            y,
            result,
            counter,
        }
    }
}

impl Executable for MulInternal {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Get the result of the previous addition
        let add_result = u128::from_be_bytes(stack.borrow_front().try_into().unwrap());

        // Update internal state
        self.counter += 1;
        self.result = add_result;

        // Remove the result from the stack
        stack.pop_front();

        if self.counter < self.y {
            // Continue adding by creating another Add task
            vec![Add::new(self.result, self.x).to_vec_with_type_tag()]
        } else {
            // We're done, push the final result
            stack.push_front(&self.result.to_be_bytes()).unwrap();
            Vec::new()
        }
    }

    fn is_finished(&mut self) -> bool {
        self.counter >= self.y
    }
}

impl Executable for Mul {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.y == 0 {
            // Shortcut for multiplication by zero
            stack.push_front(&0u128.to_be_bytes()).unwrap();
            Vec::new()
        } else {
            // Create tasks for initial addition and tracking multiplication progress
            vec![
                Add::new(0, self.x).to_vec_with_type_tag(),
                MulInternal::new(self.x, self.y, 0, 0).to_vec_with_type_tag(),
            ]
        }
    }

    fn is_finished(&mut self) -> bool {
        true // The main Mul task is finished after creating subtasks
    }
}
