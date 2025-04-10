use swiftness::types::Felt;
use swiftness_air::Transcript;
use swiftness_stark::types::StarkProof;

use crate::{
    Cache,
    intermediate::{Intermediate, VerifyIntermediate},
    task::{Task, Tasks},
};

#[derive(Debug)]
pub struct InitTranscriptTask<'a> {
    proof: &'a StarkProof,
    intermediate: &'a mut VerifyIntermediate,
}

impl Task for InitTranscriptTask<'_> {
    fn execute(&mut self) -> Vec<Tasks> {
        // Compute the initial hash seed for the Fiat-Shamir transcript.
        // Construct the transcript.
        self.intermediate.transcript = Transcript::new(
            // self.proof
            //     .public_input
            //     .get_hash(self.proof.config.n_verifier_friendly_commitment_layers),
            Felt::ZERO,
        );

        self.children()
    }

    fn children(&self) -> Vec<Tasks> {
        vec![]
    }
}

impl<'a> InitTranscriptTask<'a> {
    pub fn view(
        proof: &'a StarkProof,
        _cache: &'a mut Cache,
        intermediate: &'a mut Intermediate,
    ) -> Self {
        InitTranscriptTask {
            proof,
            intermediate: &mut intermediate.verify,
        }
    }
}
