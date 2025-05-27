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
pub struct VerifyPublicInput {
    pub a: Felt,
    pub b: Felt,
}

impl_type_identifiable!(VerifyPublicInput);

impl VerifyPublicInput {
    pub fn new(a: Felt, b: Felt) -> Self {
        Self { a, b }
    }
    fn push_input<T: BidirectionalStack>(inputs: &[Felt], stack: &mut T) {
        // Pad input with 1 followed by 0's (if necessary).
        let mut values = inputs.to_owned();
        values.push(Felt::ONE);
        values.resize(values.len().div_ceil(2) * 2, Felt::ZERO);

        assert!(values.len() % 2 == 0);

        values.iter().rev().for_each(|value| {
            stack.push_front(&value.to_bytes_be()).unwrap();
        });
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
    }
}

impl Executable for VerifyPublicInput {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let proof_reference: &mut [u8] = stack.get_proof_reference();
        let proof: &mut StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
        let public_segments = &proof.public_input.segments;
        let output_start = public_segments.get(2).unwrap().begin_addr;
        let output_end = public_segments.get(2).unwrap().stop_ptr;
        let output_len: usize = (output_end - output_start).try_into().unwrap();
        let start = proof.public_input.main_page.0.len() - output_len;
        let end = proof.public_input.main_page.0.len();
        let memory = proof.public_input.main_page.0.as_slice();
        let output = &memory[start..end];
        let output: Vec<Felt> = output.iter().map(|m| m.value).collect();
        Self::push_input(&output, stack);
        Self::push_input(&output, stack);
        vec![HashPublicInputs::new(output_len, output_len).to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        true
    }
}
