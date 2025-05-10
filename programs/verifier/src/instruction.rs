use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions supported by the verifier program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VerifierInstruction {
    /// Initializes the verifier account with default values
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    Initialize,

    /// Pushes a task to the verifier account's bidirectional stack
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    PushTask(Vec<u8>),

    /// Executes the next task in the verifier account's bidirectional stack
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    Execute,
}
