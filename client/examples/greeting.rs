use borsh::BorshDeserialize;
use client::{
    initialize_client, interact_with_program, setup_account, setup_payer, setup_program,
    ClientError, Config, ProgramInteraction, Result,
};
use greeting::state::GreetingAccount;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use std::{mem::size_of, path::Path};

/// Greeting program interaction implementation
pub struct GreetingInteraction;

impl ProgramInteraction for GreetingInteraction {
    fn process_account_data(client: &RpcClient, account: &Keypair) -> Result<()> {
        // Read the greeting account data
        let account_data = client
            .get_account_data(&account.pubkey())
            .map_err(ClientError::SolanaClientError)?;
        let greeting_account_data = GreetingAccount::try_from_slice(&account_data)
            .map_err(|e| ClientError::BorshError(e.to_string()))?;
        println!("Greeting counter: {}", greeting_account_data.counter);

        Ok(())
    }

    fn get_instruction_data(&self) -> Vec<u8> {
        // Empty instruction data for greeting program
        vec![]
    }
}

/// Main entry point for the Solana program client
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Define program path
    let program_path = Path::new("target/deploy/greeting.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path)?;

    // Setup greeting account
    let space = size_of::<GreetingAccount>();
    let greeting_account = setup_account(
        &client,
        &payer,
        &program_id,
        &config,
        space,
        "greeting-account",
    )?;

    // Interact with the program using the generic function with GreetingInteraction
    interact_with_program(
        &client,
        &payer,
        &program_id,
        &greeting_account,
        &GreetingInteraction,
    )?;

    println!("Greeting program interaction completed successfully!");

    Ok(())
}
