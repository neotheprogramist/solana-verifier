use client::{
    initialize_client, interact_with_program, setup_greeting_account, setup_payer, setup_program,
    Config,
};

/// Main entry point for the Solana program client
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config)?;

    // Setup greeting account
    let greeting_account = setup_greeting_account(&client, &payer, &program_id, &config)?;

    // Interact with the program
    interact_with_program(&client, &payer, &program_id, &greeting_account)?;

    Ok(())
}
