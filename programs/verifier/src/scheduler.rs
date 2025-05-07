//! # Scheduler
//!
//! A task scheduler library that allows for serialization and execution of tasks.
//!
//! ## Features
//!
//! - Task-based execution model
//! - Bidirectional stack for storing tasks and data
//! - Serialization of tasks using CBOR
//! - Error handling
//!

use crate::{error::VerifierError, state::BidirectionalStackAccount};
use serde::{de::DeserializeOwned, Serialize};
use utils::BidirectionalStack;

/// Trait for tasks that can be executed by the scheduler.
///
/// Implementations must be serializable and deserializable.
#[typetag::serde(tag = "type")]
pub trait SchedulerTask: Send + Sync {
    /// Execute the task and return new tasks to be pushed onto the scheduler.
    ///
    /// The scheduler is provided for pushing/popping data during execution.
    fn execute(
        &mut self,
        scheduler: &mut BidirectionalStackAccount,
    ) -> Result<Vec<Box<dyn SchedulerTask>>, VerifierError>;

    fn is_finished(&mut self) -> bool {
        false
    }
}

impl BidirectionalStackAccount {
    pub fn push_task(&mut self, task: Box<dyn SchedulerTask>) -> Result<(), VerifierError> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer).map_err(VerifierError::Serialization)?;

        self.push_back(&buffer).unwrap();

        Ok(())
    }
    pub fn push_data<T: Serialize>(&mut self, data: &T) -> Result<(), VerifierError> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(data, &mut buffer).map_err(VerifierError::Serialization)?;

        self.push_front(&buffer).unwrap();

        Ok(())
    }
    pub fn peek_task(&mut self) -> Result<Box<dyn SchedulerTask>, VerifierError> {
        let data = self.borrow_back();
        Ok(ciborium::de::from_reader(data).unwrap())
    }
    pub fn peek_data<T: DeserializeOwned>(&mut self) -> Result<T, VerifierError> {
        let data = self.borrow_front();
        Ok(ciborium::de::from_reader(data).unwrap())
    }
    pub fn pop_task(&mut self) {
        self.pop_back();
    }
    pub fn pop_data(&mut self) {
        self.pop_front();
    }

    pub fn execute(&mut self) -> Result<(), VerifierError> {
        let mut task = self.peek_task()?;

        let tasks = task
            .execute(self)
            .map_err(|e| VerifierError::Execution(format!("Task execution failed: {}", e)))?;

        if task.is_finished() {
            self.pop_task();
        }

        // Push tasks in reverse order so they execute in the order they were returned
        for task in tasks.into_iter().rev() {
            self.push_task(task)?;
        }

        Ok(())
    }

    pub fn execute_all(&mut self) -> Result<(), VerifierError> {
        while !self.is_empty_back() {
            self.execute()?;
        }
        Ok(())
    }
}
