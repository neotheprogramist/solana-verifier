use client::{
    initialize_client, interact_with_program_instructions, setup_account, setup_payer,
    setup_program, ClientError, Config,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signer::Signer,
};
use std::{mem::size_of, path::Path};
use utils::AccountCast;
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

/// Main entry point for the Solana program client
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Define program path
    let program_path = Path::new("target/deploy/verifier.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path)?;

    // Setup greeting account
    let space = size_of::<BidirectionalStackAccount>();
    println!("Greeting account space: {}", space);
    let greeting_account = setup_account(
        &client,
        &payer,
        &program_id,
        &config,
        space,
        "greeting-account",
    )?;

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::IncrementCounter,
        vec![AccountMeta::new(greeting_account.pubkey(), false)],
    )];

    // Interact with the program using the instructions directly
    interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &greeting_account,
        &instructions,
    )?;

    println!("Greeting program interaction completed successfully!");
    let mut account_data = client
        .get_account_data(&greeting_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack_account = BidirectionalStackAccount::cast_mut(&mut account_data);
    println!("Stack front index: {}", stack_account.front_index);
    println!("Stack back index: {}", stack_account.back_index);

    Ok(())
}
