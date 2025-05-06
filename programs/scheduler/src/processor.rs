use crate::utils::SchedulerTask;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::SchedulerError, instruction::SchedulerInstruction, state::SchedulerAccount};

/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the initialize instruction
    pub fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing Initialize instruction");

        // Get the scheduler account
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // The account must be owned by the program in order to modify its data
        if account.owner != program_id {
            msg!("Scheduler account does not have the correct program id");
            return Err(SchedulerError::InvalidOwner.into());
        }

        // Initialize the scheduler account
        let scheduler_account = SchedulerAccount::new();

        // Ensure the account has enough space
        if account.data_len() < borsh::to_vec(&scheduler_account).unwrap().len() {
            msg!("Scheduler account does not have enough space");
            return Err(SchedulerError::AccountTooSmall.into());
        }

        scheduler_account.serialize(&mut *account.data.borrow_mut())?;

        msg!("Scheduler initialized");

        Ok(())
    }

    /// Process the push task instruction
    pub fn process_push_task(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        task_data: &[u8],
    ) -> ProgramResult {
        msg!("Processing PushTask instruction");

        // Get the scheduler account
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // The account must be owned by the program in order to modify its data
        if account.owner != program_id {
            msg!("Scheduler account does not have the correct program id");
            return Err(SchedulerError::InvalidOwner.into());
        }

        // Deserialize the scheduler account
        let mut scheduler_account = SchedulerAccount::try_from_slice(&account.data.borrow())?;

        // Get the scheduler
        let mut scheduler = scheduler_account.get_scheduler()?;

        // Deserialize the task
        let task: Box<dyn SchedulerTask> = ciborium::de::from_reader(task_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Push the task onto the scheduler
        scheduler
            .push_task(task)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Update the scheduler account
        scheduler_account.update_scheduler(&scheduler)?;

        // Serialize the scheduler account
        scheduler_account.serialize(&mut *account.data.borrow_mut())?;

        msg!("Task pushed to scheduler");

        Ok(())
    }

    /// Process the execute task instruction
    pub fn process_execute_task(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing ExecuteTask instruction");

        // Get the scheduler account
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // The account must be owned by the program in order to modify its data
        if account.owner != program_id {
            msg!("Scheduler account does not have the correct program id");
            return Err(SchedulerError::InvalidOwner.into());
        }

        // Deserialize the scheduler account
        let mut scheduler_account = SchedulerAccount::try_from_slice(&account.data.borrow())?;

        // Get the scheduler
        let mut scheduler = scheduler_account.get_scheduler()?;

        // Execute the next task
        scheduler
            .execute()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Update the scheduler account
        scheduler_account.update_scheduler(&scheduler)?;

        // Serialize the scheduler account
        scheduler_account.serialize(&mut *account.data.borrow_mut())?;

        msg!("Task executed");

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Scheduler program entrypoint");

    // Unpack the instruction
    let instruction = SchedulerInstruction::unpack(instruction_data)?;

    // Process the instruction
    match instruction {
        SchedulerInstruction::Initialize => Processor::process_initialize(program_id, accounts),
        SchedulerInstruction::PushTask(task_data) => {
            Processor::process_push_task(program_id, accounts, &task_data)
        }
        SchedulerInstruction::ExecuteTask => Processor::process_execute_task(program_id, accounts),
    }
}
