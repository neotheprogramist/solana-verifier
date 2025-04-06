use solana_program::program_error::ProgramError;
pub use swiftness_stark::types::{Felt, StarkProof};

use crate::Cache;
use crate::verify::generate_queries::GenerateQueriesTask;
use crate::verify::stark_commit::{
    StarkCommitAssignTask, StarkCommitFriTask, StarkCommitOodsCoefTask, StarkCommitTask,
};
use crate::verify::stark_verify::StarkVerifyTask;
use crate::verify::stark_verify::fri_verify::StarkVerifyFriTask;
use crate::verify::stark_verify::fri_verify::fri_verify_layers::StarkVerifyLayersTask;
use crate::verify::stark_verify::fri_verify::fri_verify_layers::assign_next::StarkVerifyLayerAssignNextTask;
use crate::verify::stark_verify::fri_verify::fri_verify_layers::compute_next_layer::ComputeNextTask;
use crate::verify::stark_verify::fri_verify::fri_verify_layers::compute_next_layer::next_inner::ComputeNextInnerTask;
use crate::verify::stark_verify::fri_verify::fri_verify_layers::decommitment_mont::StarkVerifyLayerDecommitmentMontTask;
use crate::verify::stark_verify::fri_verify::fri_verify_layers::layer::StarkVerifyLayerTask;
use crate::verify::stark_verify::fri_verify::last_layer::StarkVerifyLastLayerTask;
use crate::verify::stark_verify::table_decommit::{TableDecommitTarget, TableDecommitTask};
use crate::verify::verify_output::VerifyOutputTask;
use crate::{
    intermediate::Intermediate,
    verify::{
        VerifyProofTask,
        init_transcript::InitTranscriptTask,
    }
};

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Tasks {
    #[default]
    VerifyProof = 0,
    StarkCommit = 1,
    GenerateQueries = 2,
    StarkVerify = 3,
    VerifyOutput = 4,
    TableDecommit(TableDecommitTarget) = 5,
    StarkCommitOodsCoef = 6,
    StarkCommitFri = 7,
    StarkCommitAssign = 8,
    StarkVerifyFri = 9,
    StarkVerifyLayersTask = 10,
    StarkVerifyLastLayerTask = 11,
    StarkVerifyFriLayer(usize) = 12,
    StarkVerifyLayerAssignNext = 13,
    StarkVerifyLayerDecommitmentMont(usize) = 14,
    ComputeNextLayer(usize) = 15,
    ComputeNextInner(usize) = 16,
    InitTranscript = 17,
}

pub type RawTask = [u8; 4];

pub trait Task {
    fn execute(&mut self) -> Vec<Tasks>;
    fn children(&self) -> Vec<Tasks>;
}

impl Tasks {
    pub fn view<'a>(
        self,
        proof: &'a mut StarkProof,
        cache: &'a mut Cache,
        intermediate: &'a mut Intermediate,
    ) -> Box<dyn Task + 'a> {
        match self {
            Tasks::VerifyProof => Box::new(VerifyProofTask::view(proof, cache, intermediate)),
            Tasks::StarkVerify => Box::new(StarkVerifyTask::view(proof, cache, intermediate)),
            Tasks::VerifyOutput => Box::new(VerifyOutputTask::view(proof, cache, intermediate)),
            Tasks::TableDecommit(target) => {
                Box::new(TableDecommitTask::view(target, proof, cache, intermediate))
            }
            Tasks::StarkCommit => Box::new(StarkCommitTask::view(proof, cache, intermediate)),
            Tasks::GenerateQueries => Box::new(GenerateQueriesTask::view(proof, cache, intermediate)),
            Tasks::StarkCommitOodsCoef => {
                Box::new(StarkCommitOodsCoefTask::view(proof, cache, intermediate))
            }
            Tasks::StarkCommitFri => Box::new(StarkCommitFriTask::view(proof, cache, intermediate)),
            Tasks::StarkCommitAssign => {
                Box::new(StarkCommitAssignTask::view(proof, cache, intermediate))
            }
            Tasks::StarkVerifyFri => Box::new(StarkVerifyFriTask::view(proof, cache, intermediate)),
            Tasks::StarkVerifyLayersTask => {
                Box::new(StarkVerifyLayersTask::view(proof, cache, intermediate))
            }
            Tasks::StarkVerifyLastLayerTask => {
                Box::new(StarkVerifyLastLayerTask::view(proof, cache, intermediate))
            }
            Tasks::StarkVerifyFriLayer(i) => {
                Box::new(StarkVerifyLayerTask::view(i, proof, cache, intermediate))
            }
            Tasks::StarkVerifyLayerAssignNext => Box::new(StarkVerifyLayerAssignNextTask::view(
                proof,
                cache,
                intermediate,
            )),
            Tasks::StarkVerifyLayerDecommitmentMont(i) => Box::new(
                StarkVerifyLayerDecommitmentMontTask::view(i, proof, cache, intermediate),
            ),
            Tasks::ComputeNextLayer(i) => {
                Box::new(ComputeNextTask::view(i, proof, cache, intermediate))
            }
            Tasks::ComputeNextInner(i) => {
                Box::new(ComputeNextInnerTask::view(i, proof, cache, intermediate))
            }
            Tasks::InitTranscript => Box::new(InitTranscriptTask::view(proof, cache, intermediate)),
        }
    }
}

impl TryFrom<&RawTask> for Tasks {
    type Error = ProgramError;

    fn try_from(value: &RawTask) -> Result<Self, Self::Error> {
        let [variant, tail @ ..] = value;

        Ok(match variant {
            0 => Tasks::VerifyProof,
            1 => Tasks::StarkCommit,
            2 => Tasks::GenerateQueries,
            3 => Tasks::StarkVerify,
            4 => Tasks::VerifyOutput,
            5 => Tasks::TableDecommit(TableDecommitTarget::try_from([tail[0], tail[1]])?),
            6 => Tasks::StarkCommitOodsCoef,
            7 => Tasks::StarkCommitFri,
            8 => Tasks::StarkCommitAssign,
            9 => Tasks::StarkVerifyFri,
            10 => Tasks::StarkVerifyLayersTask,
            11 => Tasks::StarkVerifyLastLayerTask,
            12 => Tasks::StarkVerifyFriLayer(tail[0] as usize),
            13 => Tasks::StarkVerifyLayerAssignNext,
            14 => Tasks::StarkVerifyLayerDecommitmentMont(tail[0] as usize),
            15 => Tasks::ComputeNextLayer(tail[0] as usize),
            16 => Tasks::ComputeNextInner(tail[0] as usize),
            17 => Tasks::InitTranscript,
            _ => return Err(ProgramError::Custom(2)),
        })
    }
}

impl From<Tasks> for RawTask {
    fn from(task: Tasks) -> Self {
        match task {
            Tasks::VerifyProof => [0, 0, 0, 0],
            Tasks::StarkCommit => [1, 0, 0, 0],
            Tasks::GenerateQueries => [2, 0, 0, 0],
            Tasks::StarkVerify => [3, 0, 0, 0],
            Tasks::VerifyOutput => [4, 0, 0, 0],
            Tasks::TableDecommit(target) => match target {
                TableDecommitTarget::Invalid => [5, 0, 0, 0],
                TableDecommitTarget::Original => [5, 1, 0, 0],
                TableDecommitTarget::Interaction => [5, 2, 0, 0],
                TableDecommitTarget::Composition => [5, 3, 0, 0],
                TableDecommitTarget::Fri(i) => [5, 4, i as u8, 0],
            },
            Tasks::StarkCommitOodsCoef => [6, 0, 0, 0],
            Tasks::StarkCommitFri => [7, 0, 0, 0],
            Tasks::StarkCommitAssign => [8, 0, 0, 0],
            Tasks::StarkVerifyFri => [9, 0, 0, 0],
            Tasks::StarkVerifyLayersTask => [10, 0, 0, 0],
            Tasks::StarkVerifyLastLayerTask => [11, 0, 0, 0],
            Tasks::StarkVerifyFriLayer(i) => [12, i as u8, 0, 0],
            Tasks::StarkVerifyLayerAssignNext => [13, 0, 0, 0],
            Tasks::StarkVerifyLayerDecommitmentMont(i) => [14, i as u8, 0, 0],
            Tasks::ComputeNextLayer(i) => [15, i as u8, 0, 0],
            Tasks::ComputeNextInner(i) => [16, i as u8, 0, 0],
            Tasks::InitTranscript => [17, 0, 0, 0],
        }
    }
}
