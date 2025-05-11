use crate::felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct HadesPermutation {
    state: [Felt; 3],
    phase: u8, // 0: first half full rounds, 1: partial rounds, 2: second half full rounds
    round_index: usize,
    constants_index: usize,
}

impl_type_identifiable!(HadesPermutation);

impl HadesPermutation {
    pub fn new(state: [Felt; 3]) -> Self {
        Self {
            state,
            phase: 0,
            round_index: 0,
            constants_index: 0,
        }
    }

    /// Redefined mix function for optimization purposes
    #[inline(always)]
    fn mix(state: &mut [Felt]) {
        let t = &state[0] + &state[1] + &state[2];
        state[0] = &t + &state[0].double();
        state[1] = &t - &state[1].double();
        state[2] = &t - (&state[2] + &state[2] + &state[2]);
    }
}

impl Executable for HadesPermutation {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            0 => {
                // First half of full rounds
                if self.round_index < Self::N_FULL_ROUNDS / 2 {
                    // Perform full round
                    for (i, value) in self.state.iter_mut().enumerate() {
                        *value = &(*value) + &Self::ROUND_CONSTANTS[self.constants_index + i];
                        *value = &(*value).square() * &*value;
                    }

                    // Mix step is common for both round types
                    Self::mix(&mut self.state);

                    self.round_index += 1;
                    self.constants_index += Self::N_ROUND_CONSTANTS_COLS;

                    vec![]
                } else {
                    // Move to partial rounds phase
                    self.phase = 1;
                    self.round_index = 0;

                    vec![]
                }
            }
            1 => {
                // Partial rounds
                if self.round_index < Self::N_PARTIAL_ROUNDS {
                    // Perform partial round
                    self.state[2] = self.state[2] + Self::ROUND_CONSTANTS[self.constants_index];
                    self.state[2] = self.state[2].square() * self.state[2];

                    // Mix step
                    Self::mix(&mut self.state);

                    self.round_index += 1;
                    self.constants_index += 1;

                    vec![]
                } else {
                    // Move to second half of full rounds phase
                    self.phase = 2;
                    self.round_index = 0;

                    vec![]
                }
            }
            2 => {
                // Second half of full rounds
                if self.round_index < Self::N_FULL_ROUNDS / 2 {
                    // Perform full round
                    for (i, value) in self.state.iter_mut().enumerate() {
                        *value = *value + Self::ROUND_CONSTANTS[self.constants_index + i];
                        *value = (*value).square() * *value;
                    }

                    // Mix step
                    Self::mix(&mut self.state);

                    self.round_index += 1;
                    self.constants_index += Self::N_ROUND_CONSTANTS_COLS;

                    vec![]
                } else {
                    // Push the result to the stack
                    self.round_index += 1;
                    stack.push_front(&self.state[0].to_bytes_be()).unwrap();
                    vec![]
                }
            }
            _ => unreachable!(),
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == 2 && self.round_index > Self::N_FULL_ROUNDS / 2
    }
}
