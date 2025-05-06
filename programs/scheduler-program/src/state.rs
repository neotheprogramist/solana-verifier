use borsh::{BorshDeserialize, BorshSerialize};
use scheduler::Scheduler;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchedulerAccount {
    /// The scheduler instance
    pub scheduler_data: Vec<u8>,
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
    pub fn get_scheduler(&self) -> Result<Scheduler, scheduler::Error> {
        let mut cursor = std::io::Cursor::new(&self.scheduler_data);
        let scheduler = ciborium::de::from_reader(&mut cursor)
            .map_err(|e| scheduler::Error::Deserialization(e))?;

        Ok(scheduler)
    }

    /// Update the scheduler instance
    pub fn update_scheduler(&mut self, scheduler: &Scheduler) -> Result<(), scheduler::Error> {
        self.scheduler_data.clear();
        ciborium::ser::into_writer(scheduler, &mut self.scheduler_data)
            .map_err(|e| scheduler::Error::Serialization(e))?;

        Ok(())
    }
}
