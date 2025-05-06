use crate::utils::Scheduler;
use utils::AccountCast;

/// Define the type of state stored in accounts
#[derive(Debug)]
pub struct SchedulerAccount {
    /// The scheduler instance serialized with CBOR
    pub scheduler: [u8; 65536],
}

impl SchedulerAccount {
    /// Get the scheduler instance
    pub fn get_scheduler_mut(&mut self) -> &mut Scheduler {
        Scheduler::cast_mut(&mut self.scheduler)
    }
}
