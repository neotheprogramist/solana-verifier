use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{error::VerifierError, instruction::GreetingInstruction, state::GreetingAccount};

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
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Unpack the instruction
    let instruction = GreetingInstruction::unpack(instruction_data)?;

    // Process the instruction
    match instruction {
        GreetingInstruction::IncrementCounter => {
            Processor::process_increment_counter(program_id, accounts)
        }
    }
}
