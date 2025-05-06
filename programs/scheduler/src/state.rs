use crate::utils::Scheduler;
use borsh::{BorshDeserialize, BorshSerialize};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchedulerAccount {
    /// The scheduler instance
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
        let mut scheduler_data = Vec::new();
        ciborium::ser::into_writer(&scheduler, &mut scheduler_data)
            .expect("Failed to serialize scheduler");

        Self { scheduler_data }
    }

    /// Get the scheduler instance
    pub fn get_scheduler(&self) -> Result<Scheduler, crate::utils::Error> {
        let mut cursor = std::io::Cursor::new(&self.scheduler_data);
        let scheduler =
            ciborium::de::from_reader(&mut cursor).map_err(crate::utils::Error::Deserialization)?;

        Ok(scheduler)
    }

    /// Update the scheduler instance
    pub fn update_scheduler(&mut self, scheduler: &Scheduler) -> Result<(), crate::utils::Error> {
        self.scheduler_data.clear();
        ciborium::ser::into_writer(scheduler, &mut self.scheduler_data)
            .map_err(crate::utils::Error::Serialization)?;

        Ok(())
    }
}
