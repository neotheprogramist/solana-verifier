use crate::state::BidirectionalStackAccount;
use solana_program::msg;
use utils::{BidirectionalStack, Executable, Scheduler};

// Include the generated dispatch code
include!(concat!(env!("OUT_DIR"), "/verifier_executable_dispatch.rs"));

impl Scheduler for BidirectionalStackAccount {}

impl BidirectionalStackAccount {
    pub fn execute(&mut self) {
        msg!("Executing tasks");
        let (tasks, is_finished) = execute(self);

        if is_finished {
            self.pop_back();
        }

        for task in tasks.iter().rev() {
            let _ = self.push_back(task);
        }
    }
}
