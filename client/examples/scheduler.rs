use client::{
    initialize_client, setup_account, setup_payer, setup_program, ClientError, Config, Result,
};
use scheduler::utils::Scheduler;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::path::Path;
use utils::AccountCast;

/// Push a task onto the scheduler
pub fn push_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    scheduler_account: &Keypair,
    task: &dyn scheduler::utils::SchedulerTask,
) -> Result<()> {
    use scheduler::instruction::SchedulerInstruction;

    println!("Pushing task to scheduler...");

    // Serialize the task
    let mut task_data = Vec::new();
    ciborium::ser::into_writer(task, &mut task_data)
        .map_err(|e| ClientError::SerializationError(e.to_string()))?;

    // Create the push task instruction
    let push_ix = Instruction::new_with_borsh(
        *program_id,
        &SchedulerInstruction::PushTask(task_data),
        vec![AccountMeta::new(scheduler_account.pubkey(), false)],
    );

    let blockhash = client
        .get_latest_blockhash()
        .map_err(ClientError::SolanaClientError)?;

    let push_tx =
        Transaction::new_signed_with_payer(&[push_ix], Some(&payer.pubkey()), &[payer], blockhash);

    client.send_and_confirm_transaction(&push_tx).map_err(|e| {
        ClientError::TransactionError(format!("Failed to push task to scheduler: {}", e))
    })?;

    println!("Task pushed successfully!");
    Ok(())
}

/// Execute a task from the scheduler
pub fn execute_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    scheduler_account: &Keypair,
) -> Result<()> {
    use scheduler::instruction::SchedulerInstruction;

    println!("Executing task from scheduler...");

    // Create the execute task instruction
    let execute_ix = Instruction::new_with_borsh(
        *program_id,
        &SchedulerInstruction::ExecuteTask,
        vec![AccountMeta::new(scheduler_account.pubkey(), false)],
    );

    let blockhash = client
        .get_latest_blockhash()
        .map_err(ClientError::SolanaClientError)?;

    let execute_tx = Transaction::new_signed_with_payer(
        &[execute_ix],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    client
        .send_and_confirm_transaction(&execute_tx)
        .map_err(|e| ClientError::TransactionError(format!("Failed to execute task: {}", e)))?;

    println!("Task executed successfully!");
    Ok(())
}

/// Get result from scheduler
pub fn get_result_u128(client: &RpcClient, scheduler_account: &Keypair) -> Result<u128> {
    println!("Getting result from scheduler...");

    // Get account data
    let mut account_data = client
        .get_account_data(&scheduler_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;

    // Cast to scheduler
    let scheduler = Scheduler::cast_mut(&mut account_data);

    // Pop data from scheduler
    let result: u128 = scheduler.pop_data().map_err(|e| {
        ClientError::TransactionError(format!("Failed to pop data from scheduler: {}", e))
    })?;

    println!("Result retrieved successfully!");
    Ok(result)
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
    let program_path = Path::new("target/deploy/scheduler.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path)?;

    // Setup scheduler account
    let space = 1048576; // Large enough to store the serialized scheduler
    let scheduler_account = setup_account(
        &client,
        &payer,
        &program_id,
        &config,
        space,
        "scheduler-account",
    )?;

    // Create an Add task
    let add_task = arithmetic::add::Add::new(42, 58);

    // Push the Add task to the scheduler
    push_task(&client, &payer, &program_id, &scheduler_account, &add_task)?;

    // Execute the task
    execute_task(&client, &payer, &program_id, &scheduler_account)?;

    // Get the result
    let result = get_result_u128(&client, &scheduler_account)?;
    println!("Addition result: {}", result);

    println!("Scheduler example completed successfully!");
    Ok(())
}
