use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use utils::{AccountCast, BidirectionalStack};

use crate::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the initialize instruction
    pub fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing Initialize instruction");

        // Get the account to initialize
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Initialize the bidirectional stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Set to default values - front_index to 0, back_index to CAPACITY
        *stack_account = BidirectionalStackAccount::default();
        msg!("Account initialized successfully");

        Ok(())
    }

    /// Process the push task instruction
    pub fn process_push_task(accounts: &[AccountInfo], task_data: Vec<u8>) -> ProgramResult {
        msg!("Processing PushTask instruction");

        // Get the account to push task to
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Push the task to the bidirectional stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Push the task data to the back of the stack
        stack_account.push_back(&task_data).map_err(|e| {
            msg!("Error pushing task: {:?}", e);
            ProgramError::InvalidInstructionData
        })?;
        msg!("Task pushed successfully");

        Ok(())
    }

    /// Process the execute instruction
    pub fn process_execute(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing Execute instruction");

        // Get the account to execute task from
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Execute the next task in the stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Execute the task
        stack_account.execute();
        msg!("Task executed successfully");

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Verifier Rust program entrypoint");

    // Unpack the instruction
    let instruction = VerifierInstruction::try_from_slice(instruction_data)?;

    // Process the instruction
    match instruction {
        VerifierInstruction::Initialize => Processor::process_initialize(accounts),
        VerifierInstruction::PushTask(task_data) => {
            Processor::process_push_task(accounts, task_data)
        }
        VerifierInstruction::Execute => Processor::process_execute(accounts),
    }
}
