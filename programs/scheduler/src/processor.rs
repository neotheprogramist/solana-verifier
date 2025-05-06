use crate::utils::SchedulerTask;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use utils::AccountCast;

use crate::{instruction::SchedulerInstruction, utils::Scheduler};

/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the push task instruction
    pub fn process_push_task(accounts: &[AccountInfo], task_data: &[u8]) -> ProgramResult {
        msg!("Processing PushTask instruction");

        // Get the scheduler account
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Get the scheduler
        let mut scheduler_account_data = account.try_borrow_mut_data()?;
        let scheduler = Scheduler::cast_mut(&mut scheduler_account_data);

        // Deserialize the task
        let task: Box<dyn SchedulerTask> = ciborium::de::from_reader(task_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Push the task onto the scheduler
        scheduler
            .push_task(task)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        msg!("Task pushed to scheduler");

        Ok(())
    }

    /// Process the execute task instruction
    pub fn process_execute_task(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing ExecuteTask instruction");

        // Get the scheduler account
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Get the scheduler
        let mut scheduler_account_data = account.try_borrow_mut_data()?;
        let scheduler = Scheduler::cast_mut(&mut scheduler_account_data);

        // Execute the next task
        scheduler
            .execute()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        msg!("Task executed");

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Scheduler program entrypoint");

    // Unpack the instruction
    let instruction = SchedulerInstruction::try_from_slice(instruction_data)?;

    // Process the instruction
    match instruction {
        SchedulerInstruction::PushTask(task_data) => {
            Processor::process_push_task(accounts, &task_data)
        }
        SchedulerInstruction::ExecuteTask => Processor::process_execute_task(accounts),
    }
}
