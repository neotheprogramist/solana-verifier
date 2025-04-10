use crate::{
    commit::stark_commit,
    queries::generate_queries,
    types::{LegacyCache, StarkCommitment, StarkProof},
    verify::stark_verify,
};
use alloc::vec::Vec;
use starknet_crypto::Felt;
use swiftness_air::{
    domains::StarkDomains,
    layout::{GenericLayoutTrait, LayoutTrait, PublicInputError},
};
pub use swiftness_commitment::CacheCommitment;
use swiftness_transcript::transcript::Transcript;

impl StarkProof {
    #[inline(always)]
    pub fn verify<Layout: GenericLayoutTrait + LayoutTrait>(
        &mut self,
        cache: &mut LegacyCache,
        security_bits: Felt,
    ) -> Result<(Felt, Vec<Felt>), Error> {
        let n_original_columns =
            Layout::get_num_columns_first(&self.public_input).ok_or(Error::ColumnMissing)?;
        let n_interaction_columns =
            Layout::get_num_columns_second(&self.public_input).ok_or(Error::ColumnMissing)?;

        self.config.validate(
            security_bits,
            n_original_columns.into(),
            n_interaction_columns.into(),
        )?;

        // Validate the public input.
        let stark_domains =
            StarkDomains::new(self.config.log_trace_domain_size, self.config.log_n_cosets);

        Layout::validate_public_input(&self.public_input, &stark_domains)?;

        // Compute the initial hash seed for the Fiat-Shamir transcript.
        // Construct the transcript.
        let mut transcript = Transcript::new(
            self.public_input.get_hash(self.config.n_verifier_friendly_commitment_layers),
        );

        let LegacyCache { stark, .. } = cache;

        let mut stark_commitment = StarkCommitment::default();

        // STARK commitment phase.
        stark_commit(
            &mut stark_commitment,
            stark,
            &mut transcript,
            &self.public_input,
            &self.unsent_commitment,
            &self.config,
            &stark_domains,
        )?;

        // Generate queries.
        let queries = generate_queries(
            &mut transcript,
            self.config.n_queries,
            stark_domains.eval_domain_size,
        );

        // Moves queries to the cache to free up memory.
        let queries = cache.verify.queries.move_to(queries);

        // STARK verify phase.
        stark_verify(
            stark,
            n_original_columns,
            n_interaction_columns,
            &self.public_input,
            queries,
            &stark_commitment,
            &mut self.witness,
            &stark_domains,
        )?;

        Ok(Layout::verify_public_input(&self.public_input)?)
    }
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),

    #[error("PublicInputError Error")]
    PublicInputError(#[from] PublicInputError),

    #[error("Commit Error")]
    Commit(#[from] crate::commit::Error),

    #[error("Verify Error")]
    Verify(#[from] crate::verify::Error),

    #[error("Column missing")]
    ColumnMissing,
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),

    #[error("PublicInputError Error")]
    PublicInputError(#[from] PublicInputError),

    #[error("Commit Error")]
    Commit(#[from] crate::commit::Error),

    #[error("Verify Error")]
    Verify(#[from] crate::verify::Error),

    #[error("Column missing")]
    ColumnMissing,
}
