#![no_std]

use funvec::{FunVec, FUNVEC_QUERIES};
use layer::FriLayerQuery;
use starknet_crypto::Felt;
use swiftness_commitment::table::types::Decommitment as TableDecommitment;
use swiftness_commitment::CacheCommitment;

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod first_layer;
pub mod formula;
pub mod fri;
pub mod group;
pub mod last_layer;
pub mod layer;
pub mod types;

#[cfg(any(test, feature = "test_fixtures"))]
pub mod fixtures;
#[cfg(test)]
pub mod tests;

pub type FriQueries = FunVec<FriLayerQuery, { FUNVEC_QUERIES * 3 }>;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FriVerifyCache {
    pub fri_queries: FriQueries,
    pub commitment: CacheCommitment,
    pub next_layer_cache: ComputeNextLayerCache,
    pub decommitment: TableDecommitment,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ComputeNextLayerCache {
    pub next_queries: FunVec<FriLayerQuery, 256>,
    pub verify_indices: FunVec<Felt, 256>,
    pub verify_y_values: FunVec<Felt, 256>,
    pub coset_elements: FunVec<Felt, FUNVEC_QUERIES>,
}

unsafe impl bytemuck::Pod for FriVerifyCache {}
unsafe impl bytemuck::Zeroable for FriVerifyCache {}
