use std::num::TryFromIntError;

use thiserror::Error;

/// Custom errors for the verifier program
#[derive(Error, Debug)]
pub enum VerifierError {
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

    #[error(transparent)]
    TryFromInt(#[from] TryFromIntError),
}
