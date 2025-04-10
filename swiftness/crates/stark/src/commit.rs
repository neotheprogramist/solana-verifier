use starknet_crypto::Felt;
use swiftness_air::{
    domains::StarkDomains,
    layout::{recursive_with_poseidon::Layout, LayoutTrait},
    public_memory::PublicInput,
};
use swiftness_commitment::table::commit::table_commit;
use swiftness_fri::{fri::fri_commit, types::Commitment};
use swiftness_pow::pow;
use swiftness_transcript::transcript::Transcript;

// STARK commitment phase.
pub fn stark_commit(
    result: &mut StarkCommitment,
    cache: &mut CacheStark,
    transcript: &mut Transcript,
    public_input: &PublicInput,
    unsent_commitment: &StarkUnsentCommitment,
    config: &StarkConfig,
    stark_domains: &StarkDomains,
) -> Result<(), Error> {
    // Read the commitment of the 'traces' component.

    let traces_commitment =
        Layout::traces_commit(transcript, &unsent_commitment.traces, config.traces.clone());

    // Generate interaction values after traces commitment.
    let composition_alpha = transcript.random_felt_to_prover();
    powers_array(
        cache.powers_array.powers_array.unchecked_slice_mut(Layout::N_CONSTRAINTS),
        Felt::ONE,
        composition_alpha,
        Layout::N_CONSTRAINTS as u32,
    );
    let traces_coefficients =
        cache.powers_array.powers_array.unchecked_slice(Layout::N_CONSTRAINTS);

    // Read composition commitment.
    let composition_commitment =
        table_commit(transcript, unsent_commitment.composition, config.composition.clone());

    // Generate interaction values after composition.
    let interaction_after_composition = transcript.random_felt_to_prover();

    // Read OODS values.
    transcript.read_felt_vector_from_prover(&unsent_commitment.oods_values.to_vec());

    // Check that the trace and the composition agree at oods_point.
    verify_oods::<Layout>(
        cache.commitment.verify_oods.inner(),
        unsent_commitment.oods_values.as_slice(),
        &traces_commitment.interaction_elements,
        public_input,
        &traces_coefficients,
        &interaction_after_composition,
        &stark_domains.trace_domain_size,
        &stark_domains.trace_generator,
    )?;

    // Generate interaction values after OODS.
    let oods_alpha = transcript.random_felt_to_prover();

    cache.powers_array.powers_array.flush();
    let n = Layout::MASK_SIZE + Layout::CONSTRAINT_DEGREE;
    powers_array(
        cache.powers_array.powers_array.unchecked_slice_mut(n),
        Felt::ONE,
        oods_alpha,
        n as u32,
    );
    let oods_coefficients = cache.powers_array.powers_array.unchecked_slice(n);

    // Read fri commitment.
    let mut fri_commitment = Commitment::default();
    fri_commit(&mut fri_commitment, transcript, &unsent_commitment.fri, &config.fri);

    // Proof of work commitment phase.
    unsent_commitment.proof_of_work.commit(transcript, &config.proof_of_work)?;

    let StarkCommitment {
        traces,
        composition,
        interaction_after_composition: interaction,
        oods_values,
        interaction_after_oods,
        fri,
    } = result;

    // Return commitment.
    *traces = traces_commitment;
    *composition = composition_commitment;
    *interaction = interaction_after_composition;
    *fri = fri_commitment;

    oods_values.overwrite(unsent_commitment.oods_values.as_slice());
    interaction_after_oods.overwrite(&oods_coefficients);

    Ok(())
}

pub fn powers_array(powers_array: &mut [Felt], initial: Felt, alpha: Felt, n: u32) {
    // let mut array = Vec::with_capacity(n as usize);
    let mut value = initial;

    for i in 0..n as usize {
        powers_array[i] = value;
        value *= alpha;
    }
}

use crate::{
    config::StarkConfig,
    oods::{self, verify_oods},
    types::{CacheStark, StarkCommitment, StarkUnsentCommitment},
};

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("POW Error")]
    POW(#[from] pow::Error),

    #[error("OodsVerifyError Error")]
    Oods(#[from] oods::OodsVerifyError),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("POW Error")]
    POW(#[from] pow::Error),

    #[error("OodsVerifyError Error")]
    Oods(#[from] oods::OodsVerifyError),
}
