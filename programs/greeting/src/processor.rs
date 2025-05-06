use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{instruction::GreetingInstruction, state::GreetingAccount};

/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the increment counter instruction
    pub fn process_increment_counter(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing IncrementCounter instruction");

        // Get the account to increment counter
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Increment and store the number of times the account has been greeted
        let mut data = account.try_borrow_mut_data()?;
        let greeting_account = GreetingAccount::cast_mut(*data);
        greeting_account.counter += 1;
        greeting_account.double_counter += 2;

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Unpack the instruction
    let instruction = GreetingInstruction::try_from_slice(instruction_data)?;

    // Process the instruction
    match instruction {
        GreetingInstruction::IncrementCounter => Processor::process_increment_counter(accounts),
    }
}
