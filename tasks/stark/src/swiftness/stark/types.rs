use crate::swiftness::air::public_memory::PublicInput;

use super::config::StarkConfig;

pub fn cast_slice_to_struct(slice: &mut [u8]) -> &mut StarkProof {
    assert_eq!(slice.len(), std::mem::size_of::<StarkProof>());
    unsafe { &mut *(slice.as_mut_ptr() as *mut StarkProof) }
}
pub fn cast_struct_to_slice(proof: &mut StarkProof) -> &mut [u8] {
    let ptr = proof as *mut StarkProof as *mut u8;
    let len = std::mem::size_of::<StarkProof>();
    unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StarkProof {
    pub config: StarkConfig,
    pub public_input: PublicInput,
}

#[cfg(test)]
mod test {
    use crate::{
        felt::Felt,
        funvec::FunVec,
        swiftness::{
            air::public_memory::{Page, PublicInput},
            stark::{
                config::StarkConfig,
                types::{cast_slice_to_struct, cast_struct_to_slice, StarkProof},
            },
        },
    };

    #[test]
    fn test_stark_proof() {
        let proof = StarkProof {
            public_input: PublicInput {
                log_n_steps: Felt::from(1),
                range_check_min: Felt::from(2),
                range_check_max: Felt::from(3),
                layout: Felt::from(4),
                segments: FunVec::default(),
                padding_addr: Felt::from(5),
                padding_value: Felt::from(6),
                main_page: Page::default(),
                continuous_page_headers: FunVec::default(),
            },
            config: StarkConfig::default(),
        };
        let mut proof_clone = proof.clone();
        let mut bytes = cast_struct_to_slice(&mut proof_clone);

        let proof_from_bytes = cast_slice_to_struct(&mut bytes);
        assert_eq!(proof_from_bytes, &proof);
    }
}
