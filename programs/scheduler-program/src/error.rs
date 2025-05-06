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
}

impl From<SchedulerError> for ProgramError {
    fn from(e: SchedulerError) -> Self {
        msg!("Error: {}", e);
        ProgramError::Custom(e as u32)
    }
}

impl From<scheduler::Error> for SchedulerError {
    fn from(e: scheduler::Error) -> Self {
        match e {
            scheduler::Error::Deserialization(_) => SchedulerError::SchedulerDeserializationError,
            scheduler::Error::Serialization(_) => SchedulerError::SchedulerSerializationError,
            scheduler::Error::StackCapacity(_) => SchedulerError::SchedulerTaskPushError,
            scheduler::Error::EmptyStack => SchedulerError::SchedulerDataPopError,
            scheduler::Error::Execution(_) => SchedulerError::SchedulerExecutionError,
            scheduler::Error::InvalidTaskLength => SchedulerError::SchedulerTaskPushError,
            scheduler::Error::Task(_) => SchedulerError::SchedulerExecutionError,
            scheduler::Error::InvalidData(_) => SchedulerError::SchedulerDeserializationError,
            scheduler::Error::Io(_) => SchedulerError::SchedulerDeserializationError,
        }
    }
}
