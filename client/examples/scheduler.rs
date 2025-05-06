use client::{
    initialize_client, schedule_add_task, setup_payer, setup_program, setup_scheduler_account,
    Config,
};

/// Main entry point for the Solana scheduler example
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config)?;

    // Setup scheduler account
    let scheduler_account = setup_scheduler_account(&client, &payer, &program_id, &config)?;

    // Schedule an Add task with operands 42 and 58
    schedule_add_task(&client, &payer, &program_id, &scheduler_account, 42, 58)?;
    println!("Successfully executed Add task: 42 + 58 = 100");

    // Try another example with larger numbers
    schedule_add_task(
        &client,
        &payer,
        &program_id,
        &scheduler_account,
        1234567890,
        9876543210,
    )?;
    println!("Successfully executed Add task: 1234567890 + 9876543210 = 11111111100");

    println!("Scheduler example completed successfully!");
    Ok(())
}
