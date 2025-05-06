use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

/// Instructions supported by the verifier program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VerifierInstruction {
    /// Increments the counter in the greeting account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The greeting account
    IncrementCounter,
}

impl VerifierInstruction {
    /// Unpacks a byte buffer into a VerifierInstruction
    pub fn unpack(_input: &[u8]) -> Result<Self, ProgramError> {
        // Currently we only have one instruction, so we can just return it
        // In a more complex program, we would parse the instruction type from the input
        Ok(Self::IncrementCounter)
    }
}
