use client::{
    initialize_client, interact_with_program, setup_account, setup_payer, setup_program,
    Config,
};
use greeting::state::GreetingAccount;
use std::{mem::size_of, path::Path};

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
    let greeting_account = setup_account(&client, &payer, &program_id, &config, space, "greeting-account")?;

    // Interact with the program
    interact_with_program(&client, &payer, &program_id, &greeting_account)?;

    println!("Greeting program interaction completed successfully!");

    Ok(())
}
