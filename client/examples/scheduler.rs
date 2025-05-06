use client::{
    initialize_client, initialize_scheduler, setup_payer, setup_program, setup_scheduler_account,
    Config,
};
use std::path::Path;

/// Main entry point for the Solana program client
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Define program path
    let program_path = Path::new("target/deploy/scheduler.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path)?;

    // Setup scheduler account
    let scheduler_account = setup_scheduler_account(&client, &payer, &program_id, &config)?;

    // Initialize the scheduler
    initialize_scheduler(&client, &payer, &program_id, &scheduler_account)?;

    println!("Scheduler program initialization completed successfully!");

    Ok(())
}
