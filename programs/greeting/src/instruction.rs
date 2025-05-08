use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions supported by the verifier program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GreetingInstruction {
    /// Increments the counter in the greeting account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The greeting account
    IncrementCounter,
}
