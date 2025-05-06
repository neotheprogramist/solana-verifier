use borsh::{BorshDeserialize, BorshSerialize};
use scheduler::Scheduler;
use std::io::Cursor;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

/// Scheduler account for managing tasks
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchedulerAccount {
    /// The scheduler instance
    pub scheduler: Vec<u8>, // Serialized scheduler
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
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&scheduler, &mut buffer).unwrap_or_default();
        Self { scheduler: buffer }
    }

    /// Get the scheduler instance
    pub fn get_scheduler(&self) -> Result<Scheduler, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(&self.scheduler);
        let scheduler = ciborium::de::from_reader(&mut cursor)?;
        Ok(scheduler)
    }

    /// Update the scheduler instance
    pub fn update_scheduler(
        &mut self,
        scheduler: &Scheduler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(scheduler, &mut buffer)?;
        self.scheduler = buffer;
        Ok(())
    }
}
