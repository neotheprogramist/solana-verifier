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

    /// Schedules an Add task
    ///
    /// Accounts expected:
    /// 0. `[writable]` The scheduler account
    ///
    /// Data:
    /// * u128 - First operand
    /// * u128 - Second operand
    ScheduleAdd(u128, u128),
}

impl VerifierInstruction {
    /// Unpacks a byte buffer into a VerifierInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            return Ok(Self::IncrementCounter);
        }

        match input[0] {
            0 => Ok(Self::IncrementCounter),
            1 => {
                if input.len() < 33 {
                    // 1 byte for instruction + 16 bytes for each u128
                    return Err(ProgramError::InvalidInstructionData);
                }

                let x = u128::from_le_bytes(input[1..17].try_into().unwrap());
                let y = u128::from_le_bytes(input[17..33].try_into().unwrap());

                Ok(Self::ScheduleAdd(x, y))
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
