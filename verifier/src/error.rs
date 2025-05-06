use solana_program::{msg, program_error::ProgramError};
use thiserror::Error;

/// Custom errors for the verifier program
#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Account not owned by program")]
    InvalidOwner,
}

impl From<VerifierError> for ProgramError {
    fn from(e: VerifierError) -> Self {
        msg!("Error: {}", e);
        ProgramError::Custom(e as u32)
    }
}
