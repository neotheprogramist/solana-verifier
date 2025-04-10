use funvec::{FunVec, FUNVEC_OODS, FUNVEC_QUERIES};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
pub use starknet_crypto::Felt;
use swiftness_air::layout::{recursive_with_poseidon::Layout, LayoutTrait};
use swiftness_commitment::CacheCommitment;
use swiftness_fri::FriVerifyCache;

use crate::config;

#[derive(Debug, Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod, PartialEq)]
#[repr(C)]
pub struct LegacyCache {
    pub stark: CacheStark,
    pub verify: VerifyCache,
}

#[derive(Debug, Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod, PartialEq)]
#[repr(C)]
pub struct CacheStark {
    pub commitment: CacheCommitment,
    pub fri: FriVerifyCache,
    pub powers_array: PowersArrayCache,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct PowersArrayCache {
    pub powers_array: FunVec<Felt, 256>,
}

unsafe impl bytemuck::Pod for PowersArrayCache {}
unsafe impl bytemuck::Zeroable for PowersArrayCache {}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct VerifyCache {
    pub queries: FunVec<Felt, FUNVEC_QUERIES>,
}

unsafe impl bytemuck::Pod for VerifyCache {}
unsafe impl bytemuck::Zeroable for VerifyCache {}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    bytemuck::Zeroable,
    bytemuck::Pod,
)]
#[repr(C)]
pub struct StarkProof {
    pub config: config::StarkConfig,
    pub public_input: swiftness_air::public_memory::PublicInput,
    pub unsent_commitment: StarkUnsentCommitment,
    pub witness: StarkWitness,
}

#[serde_as]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StarkUnsentCommitment {
    pub traces: swiftness_air::trace::UnsentCommitment,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub composition: Felt,
    // n_oods_values elements. The i-th value is the evaluation of the i-th mask item polynomial at
    // the OODS point, where the mask item polynomial is the interpolation polynomial of the
    // corresponding column shifted by the corresponding row_offset.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub oods_values: FunVec<Felt, FUNVEC_OODS>,
    pub fri: swiftness_fri::types::UnsentCommitment,
    pub proof_of_work: swiftness_pow::pow::UnsentCommitment,
}

unsafe impl bytemuck::Zeroable for StarkUnsentCommitment {}
unsafe impl bytemuck::Pod for StarkUnsentCommitment {}

#[serde_as]
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct StarkCommitment {
    pub traces: swiftness_air::trace::Commitment<<Layout as LayoutTrait>::InteractionElements>,
    pub composition: swiftness_commitment::table::types::Commitment,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub interaction_after_composition: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub oods_values: FunVec<Felt, FUNVEC_OODS>,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub interaction_after_oods: FunVec<Felt, FUNVEC_OODS>,
    pub fri: swiftness_fri::types::Commitment,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct StarkWitness {
    pub traces_decommitment: swiftness_air::trace::Decommitment,
    pub traces_witness: swiftness_air::trace::Witness,
    pub composition_decommitment: swiftness_commitment::table::types::Decommitment,
    pub composition_witness: swiftness_commitment::table::types::Witness,
    pub fri_witness: swiftness_fri::types::Witness,
}

unsafe impl bytemuck::Zeroable for StarkWitness {}
unsafe impl bytemuck::Pod for StarkWitness {}
