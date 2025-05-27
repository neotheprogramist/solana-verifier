use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashPublicInputsStep {
    Init,
    ProgramHash,
    OutputHash,
    Done,
}

#[repr(C)]
pub struct HashPublicInputs {
    pub step: HashPublicInputsStep,
    pub program_input_length: usize,
    pub output_input_length: usize,
    pub program_hash: Felt,
}

impl_type_identifiable!(HashPublicInputs);

impl HashPublicInputs {
    pub fn new(program_input_length: usize, output_input_length: usize) -> Self {
        Self {
            step: HashPublicInputsStep::Init,
            program_input_length,
            output_input_length,
            program_hash: Felt::ZERO,
        }
    }
}

impl Executable for HashPublicInputs {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            HashPublicInputsStep::Init => {
                self.step = HashPublicInputsStep::ProgramHash;
                vec![PoseidonHashMany::new(self.program_input_length).to_vec_with_type_tag()]
            }
            HashPublicInputsStep::ProgramHash => {
                let bytes = stack.borrow_front();
                let program_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();
                self.program_hash = program_hash;
                self.step = HashPublicInputsStep::OutputHash;
                vec![PoseidonHashMany::new(self.output_input_length).to_vec_with_type_tag()]
            }
            HashPublicInputsStep::OutputHash => {
                let bytes = stack.borrow_front();
                let output_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                stack.push_front(&output_hash.to_bytes_be()).unwrap();
                stack.push_front(&self.program_hash.to_bytes_be()).unwrap();

                self.step = HashPublicInputsStep::Done;
                vec![]
            }
            HashPublicInputsStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == HashPublicInputsStep::Done
    }
}

#[repr(C)]
pub struct VerifyPublicInput {}

impl_type_identifiable!(VerifyPublicInput);

impl VerifyPublicInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executable for VerifyPublicInput {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let proof_reference: &mut [u8] = stack.get_proof_reference();
        let proof: &mut StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
        let public_segments = &proof.public_input.segments;
        // let initial_pc = public_segments.get(0).unwrap().begin_addr;
        // let initial_fp = public_segments.get(1).unwrap().begin_addr;

        // let final_ap = public_segments.get(1).unwrap().stop_ptr;
        let output_start = public_segments.get(2).unwrap().begin_addr;
        let output_end = public_segments.get(3).unwrap().stop_ptr;
        let memory = proof.public_input.main_page.0.as_slice();
        let output_len: usize = (output_end - output_start).try_into().unwrap();
        let _output: Vec<Felt> = memory[memory.len() - output_len..]
            .iter()
            .map(|m| m.value)
            .collect();
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        true
    }
}
