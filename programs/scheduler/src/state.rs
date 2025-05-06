use crate::utils::Scheduler;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchedulerAccount {
    /// The scheduler instance serialized with CBOR
    pub scheduler_data: Vec<u8>,
}

impl Default for SchedulerAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulerAccount {
    /// Create a new scheduler account
    pub fn new() -> Self {
        let scheduler = Scheduler::new();
        let mut scheduler_data = Vec::with_capacity(128); // Smaller preallocated size
        ciborium::ser::into_writer(&scheduler, &mut scheduler_data)
            .expect("Failed to serialize scheduler");

        Self { scheduler_data }
    }

    /// Get the scheduler instance
    pub fn get_scheduler(&self) -> Result<Scheduler, ProgramError> {
        ciborium::de::from_reader(self.scheduler_data.as_slice())
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Update the scheduler instance
    pub fn update_scheduler(&mut self, scheduler: &Scheduler) -> Result<(), ProgramError> {
        self.scheduler_data.clear();
        ciborium::ser::into_writer(scheduler, &mut self.scheduler_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
}
