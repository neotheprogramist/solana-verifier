use crate::state::BidirectionalStackAccount;
use utils::{BidirectionalStack, Executable, Scheduler};

// Include the generated dispatch code
include!(concat!(env!("OUT_DIR"), "/verifier_executable_dispatch.rs"));

impl Scheduler for BidirectionalStackAccount {}

impl BidirectionalStackAccount {
    pub fn execute(&mut self) {
        execute(self);
        self.pop_back();
    }
}
