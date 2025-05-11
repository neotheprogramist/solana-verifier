use crate::felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

pub mod constants;
pub mod hades;
pub mod poseidon;

#[repr(C)]
pub struct PoseidonTask {
    x: Felt,
    state: [Felt; 3],
}

impl_type_identifiable!(PoseidonTask);

impl PoseidonTask {
    pub fn new(x: Felt) -> Self {
        Self {
            x,
            state: [Felt::ZERO, Felt::ZERO, Felt::ZERO],
        }
    }
}

impl Executable for PoseidonTask {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        self.state[0] += self.x;
        self.state[1] += Felt::ONE;

        stack.push_front(&self.state[0].to_bytes_be()).unwrap();

        Vec::new()
    }

    fn is_finished(&mut self) -> bool {
        true
    }
}
