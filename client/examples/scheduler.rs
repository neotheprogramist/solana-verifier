use arithmetic::add::Add;
use client::{
    execute_task, initialize_client, initialize_scheduler, push_task, setup_payer, setup_program,
    setup_scheduler_account, Config,
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

    // Setup scheduler account
    let scheduler_account = setup_scheduler_account(&client, &payer, &program_id, &config)?;

    // Initialize the scheduler
    initialize_scheduler(&client, &payer, &program_id, &scheduler_account)?;

    // Create an Add task
    let add_task = Add::new(42, 58);

    // Push the task onto the scheduler
    push_task(&client, &payer, &program_id, &scheduler_account, &add_task)?;

    // Execute the task
    execute_task(&client, &payer, &program_id, &scheduler_account)?;

    println!("Example completed successfully!");
    println!("The result of 42 + 58 has been computed and stored in the scheduler's data stack.");

    Ok(())
}
