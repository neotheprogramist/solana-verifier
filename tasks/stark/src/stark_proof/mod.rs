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

pub mod segments {
    pub const BITWISE: usize = 5;
    pub const EXECUTION: usize = 1;
    pub const N_SEGMENTS: usize = 7;
    pub const OUTPUT: usize = 2;
    pub const PEDERSEN: usize = 3;
    pub const POSEIDON: usize = 6;
    pub const PROGRAM: usize = 0;
    pub const RANGE_CHECK: usize = 4;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyPublicInputStep {
    Init,
    Output,
    Program,
    Done,
}
#[repr(C)]
pub struct VerifyPublicInput {
    step: VerifyPublicInputStep,
    program_start: usize,
    program_end: usize,
    program_len: usize,
    output_start: usize,
    output_end: usize,
    output_len: usize,
}

impl_type_identifiable!(VerifyPublicInput);

impl VerifyPublicInput {
    pub fn new() -> Self {
        Self {
            step: VerifyPublicInputStep::Init,
            program_start: 0,
            program_end: 0,
            output_start: 0,
            output_end: 0,
            program_len: 0,
            output_len: 0,
        }
    }
}

impl Executable for VerifyPublicInput {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyPublicInputStep::Init => {
                let proof_reference: &mut [u8] = stack.get_proof_reference();
                let proof: &mut StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                let public_segments = &proof.public_input.segments;

                let initial_pc: usize = public_segments
                    .get(segments::PROGRAM)
                    .unwrap()
                    .begin_addr
                    .try_into()
                    .unwrap();
                let initial_fp: usize = public_segments
                    .get(segments::EXECUTION)
                    .unwrap()
                    .begin_addr
                    .try_into()
                    .unwrap();

                //1. Program segment
                let program_end_pc: usize = initial_fp - 2;
                let program_len = program_end_pc - initial_pc;

                let output_start: usize = public_segments
                    .get(segments::OUTPUT)
                    .unwrap()
                    .begin_addr
                    .try_into()
                    .unwrap();
                let output_end: usize = public_segments
                    .get(segments::OUTPUT)
                    .unwrap()
                    .stop_ptr
                    .try_into()
                    .unwrap();
                let output_len: usize = (output_end - output_start).try_into().unwrap();
                let output_start = proof.public_input.main_page.0.len() - output_len;

                self.output_start = output_start;
                self.output_end = proof.public_input.main_page.0.len();
                self.output_len = output_len;

                self.program_end = program_len;
                self.program_len = program_len;

                self.step = VerifyPublicInputStep::Output;
                vec![]
            }
            VerifyPublicInputStep::Output => {
                let inputs_len = self.output_len + 1;
                let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

                for i in (self.output_start..self.output_end).rev() {
                    let proof_reference: &mut [u8] = stack.get_proof_reference();
                    let proof: &mut StarkProof =
                        cast_slice_to_struct::<StarkProof>(proof_reference);
                    let memory = proof.public_input.main_page.0.as_slice();
                    let item = memory[i].value;
                    stack.push_front(&item.to_bytes_be()).unwrap();
                }

                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                self.step = VerifyPublicInputStep::Program;
                vec![]
            }
            VerifyPublicInputStep::Program => {
                let inputs_len = self.program_len + 1;
                let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }

                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();
                for i in (self.program_start..self.program_end).rev() {
                    let proof_reference: &mut [u8] = stack.get_proof_reference();
                    let proof: &mut StarkProof =
                        cast_slice_to_struct::<StarkProof>(proof_reference);
                    let memory = proof.public_input.main_page.0.as_slice();
                    let item = memory[i].value;
                    stack.push_front(&item.to_bytes_be()).unwrap();
                }
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                self.step = VerifyPublicInputStep::Done;

                vec![HashPublicInputs::new(self.program_len, self.output_len).to_vec_with_type_tag()]
            }
            VerifyPublicInputStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyPublicInputStep::Done
    }
}
