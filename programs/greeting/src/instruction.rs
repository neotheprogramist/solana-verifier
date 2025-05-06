use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

/// Instructions supported by the verifier program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GreetingInstruction {
    /// Increments the counter in the greeting account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The greeting account
    IncrementCounter,
}

impl GreetingInstruction {
    /// Unpacks a byte buffer into a GreetingInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            return Ok(Self::IncrementCounter);
        }

        match input[0] {
            0 => Ok(Self::IncrementCounter),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
