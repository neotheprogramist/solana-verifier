use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions supported by the verifier program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VerifierInstruction {
    /// Increments the counter in the verifier account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    IncrementCounter,
}
