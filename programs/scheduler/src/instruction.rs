use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

/// Instructions supported by the scheduler program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SchedulerInstruction {
    /// Initialize a new scheduler account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The scheduler account
    Initialize,

    /// Push a task onto the scheduler
    ///
    /// Accounts expected:
    /// 0. `[writable]` The scheduler account
    ///
    /// Data: Serialized task
    PushTask(Vec<u8>),

    /// Execute the next task in the scheduler
    ///
    /// Accounts expected:
    /// 0. `[writable]` The scheduler account
    ExecuteTask,
}

impl SchedulerInstruction {
    /// Unpacks a byte buffer into a SchedulerInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Initialize,
            1 => {
                let task_data = rest.to_vec();
                Self::PushTask(task_data)
            }
            2 => Self::ExecuteTask,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
