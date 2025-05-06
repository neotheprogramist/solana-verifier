use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions supported by the scheduler program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SchedulerInstruction {
    /// Push a task onto the scheduler
    ///
    /// Accounts expected:
    /// 0. `[writable]` The scheduler account
    ///
    /// Data: Serialized task
    PushTask(Vec<u8>),

    /// Execute the next task in the scheduler
    ///
    /// Accounts expected:
    /// 0. `[writable]` The scheduler account
    ExecuteTask,
}
