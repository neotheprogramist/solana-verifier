use arithmetic::add::Add;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
    error::VerifierError,
    instruction::VerifierInstruction,
    state::{GreetingAccount, SchedulerAccount},
};

/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the increment counter instruction
    pub fn process_increment_counter(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("Processing IncrementCounter instruction");

        // Get the account to increment counter
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // The account must be owned by the program in order to modify its data
        if account.owner != program_id {
            msg!("Greeted account does not have the correct program id");
            return Err(VerifierError::InvalidOwner.into());
        }

        // Increment and store the number of times the account has been greeted
        let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
        greeting_account.counter += 1;
        greeting_account.serialize(&mut *account.data.borrow_mut())?;

        msg!("Greeted {} time(s)!", greeting_account.counter);

        Ok(())
    }

    /// Process the schedule add instruction
    pub fn process_schedule_add(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        x: u128,
        y: u128,
    ) -> ProgramResult {
        msg!("Processing ScheduleAdd instruction");

        // Get the scheduler account
        let accounts_iter = &mut accounts.iter();
        let scheduler_account_info = next_account_info(accounts_iter)?;

        // The account must be owned by the program
        if scheduler_account_info.owner != program_id {
            msg!("Scheduler account does not have the correct program id");
            return Err(VerifierError::InvalidOwner.into());
        }

        // Get the scheduler from the account
        let mut scheduler_account =
            match SchedulerAccount::try_from_slice(&scheduler_account_info.data.borrow()) {
                Ok(account) => account,
                Err(_) => {
                    // If the account doesn't exist yet, create a new one
                    msg!("Creating new scheduler account");
                    SchedulerAccount::new()
                }
            };

        // Get the scheduler instance
        let mut scheduler = match scheduler_account.get_scheduler() {
            Ok(s) => s,
            Err(e) => {
                msg!("Error deserializing scheduler: {}", e);
                return Err(VerifierError::SchedulerDeserializationError.into());
            }
        };

        // Create and push the Add task
        let add_task = Box::new(Add::new(x, y));
        if let Err(e) = scheduler.push_task(add_task) {
            msg!("Error pushing task: {}", e);
            return Err(VerifierError::SchedulerTaskPushError.into());
        }

        // Execute the task
        if let Err(e) = scheduler.execute() {
            msg!("Error executing task: {}", e);
            return Err(VerifierError::SchedulerExecutionError.into());
        }

        // Get the result
        let result: u128 = match scheduler.pop_data() {
            Ok(r) => r,
            Err(e) => {
                msg!("Error getting result: {}", e);
                return Err(VerifierError::SchedulerDataPopError.into());
            }
        };

        msg!("Add result: {} + {} = {}", x, y, result);

        // Update the scheduler account
        if let Err(e) = scheduler_account.update_scheduler(&scheduler) {
            msg!("Error serializing scheduler: {}", e);
            return Err(VerifierError::SchedulerSerializationError.into());
        }

        // Save the updated scheduler account
        scheduler_account.serialize(&mut *scheduler_account_info.data.borrow_mut())?;

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Unpack the instruction
    let instruction = VerifierInstruction::unpack(instruction_data)?;

    // Process the instruction
    match instruction {
        VerifierInstruction::IncrementCounter => {
            Processor::process_increment_counter(program_id, accounts)
        }
        VerifierInstruction::ScheduleAdd(x, y) => {
            Processor::process_schedule_add(program_id, accounts, x, y)
        }
    }
}
