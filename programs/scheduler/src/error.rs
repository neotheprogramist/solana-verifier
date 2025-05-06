use solana_program::{msg, program_error::ProgramError};
use thiserror::Error;

/// Custom errors for the scheduler program
#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("Account not owned by program")]
    InvalidOwner,

    #[error("Error deserializing scheduler")]
    SchedulerDeserializationError,

    #[error("Error pushing task to scheduler")]
    SchedulerTaskPushError,

    #[error("Error executing scheduler task")]
    SchedulerExecutionError,

    #[error("Error popping data from scheduler")]
    SchedulerDataPopError,

    #[error("Error serializing scheduler")]
    SchedulerSerializationError,

    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Account too small to store scheduler data")]
    AccountTooSmall,
}

impl From<SchedulerError> for ProgramError {
    fn from(e: SchedulerError) -> Self {
        msg!("Error: {}", e);
        ProgramError::Custom(e as u32)
    }
}

impl From<crate::utils::Error> for SchedulerError {
    fn from(e: crate::utils::Error) -> Self {
        match e {
            crate::utils::Error::Deserialization(_) => {
                SchedulerError::SchedulerDeserializationError
            }
            crate::utils::Error::Serialization(_) => SchedulerError::SchedulerSerializationError,
            crate::utils::Error::StackCapacity(_) => SchedulerError::SchedulerTaskPushError,
            crate::utils::Error::EmptyStack => SchedulerError::SchedulerDataPopError,
            crate::utils::Error::Execution(_) => SchedulerError::SchedulerExecutionError,
            crate::utils::Error::InvalidTaskLength => SchedulerError::SchedulerTaskPushError,
            crate::utils::Error::Task(_) => SchedulerError::SchedulerExecutionError,
            crate::utils::Error::InvalidData(_) => SchedulerError::SchedulerDeserializationError,
            crate::utils::Error::Io(_) => SchedulerError::SchedulerDeserializationError,
        }
    }
}
