use crate::{
    felt::Felt,
    funvec::{FunVec, FUNVEC_LAST_LAYER, FUNVEC_LAYERS, FUNVEC_LEAVES},
    swiftness::commitment::table,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    pub inner_layers: FunVec<Felt, FUNVEC_LAYERS>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: FunVec<Felt, FUNVEC_LAST_LAYER>,
}

// A witness for the decommitment of the FRI layers over queries.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Witness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    pub layers: FunVec<LayerWitness, FUNVEC_LAYERS>,
}

// A witness for a single FRI layer. This witness is required to verify the transition from an
// inner layer to the following layer.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LayerWitness {
    pub leaves: FunVec<Felt, FUNVEC_LEAVES>,
    // Table commitment witnesses for decommiting all the leaves.
    pub table_witness: table::types::Witness,
}
