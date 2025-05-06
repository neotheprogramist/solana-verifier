use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
};

use crate::{
    account::{read_keypair_file, write_keypair_file},
    transaction::create_and_send_transaction,
    ClientError, Config, Result,
};

/// Setup the scheduler account for the Solana program
pub fn setup_scheduler_account(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    config: &Config,
) -> Result<Keypair> {
    // Check if scheduler keypair already exists
    let scheduler_keypair_path = config.data_dir.join("scheduler-keypair.json");

    match read_keypair_file(&scheduler_keypair_path) {
        Ok(keypair) => {
            println!("Using existing scheduler account: {}", keypair.pubkey());
            Ok(keypair)
        }
        Err(_) => {
            let scheduler_keypair = Keypair::new();
            println!(
                "Creating new scheduler account: {}",
                scheduler_keypair.pubkey()
            );

            // Calculate rent for the scheduler account
            let scheduler_size = 1024; // Size of the scheduler account data
            let rent = client
                .get_minimum_balance_for_rent_exemption(scheduler_size)
                .map_err(|err| {
                    ClientError::TransactionError(format!(
                        "Failed to get rent exemption for scheduler account: {}",
                        err
                    ))
                })?;

            // Create system account
            let create_ix = system_instruction::create_account(
                &payer.pubkey(),
                &scheduler_keypair.pubkey(),
                rent,
                scheduler_size as u64,
                program_id,
            );

            // Create and send transaction
            let signers = &[payer, &scheduler_keypair];
            let instruction = create_ix;
            create_and_send_transaction(client, instruction, signers, config)?;

            // Save the keypair
            write_keypair_file(&scheduler_keypair, &scheduler_keypair_path)?;
            println!("Scheduler account created!");

            Ok(scheduler_keypair)
        }
    }
}

/// Initialize the scheduler
pub fn initialize_scheduler(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    scheduler_account: &Keypair,
    config: &Config,
) -> Result<()> {
    // Create instruction to initialize scheduler
    let initialize_ix = Instruction::new_with_bytes(
        *program_id,
        &[0], // Initialize instruction = 0
        vec![AccountMeta::new(scheduler_account.pubkey(), false)],
    );

    // Send transaction
    let signers = &[payer];
    create_and_send_transaction(client, initialize_ix, signers, config)?;

    println!("Scheduler initialized!");
    Ok(())
}

/// Push a task onto the scheduler
pub fn push_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    scheduler_account: &Keypair,
    task_data: &[u8],
    config: &Config,
) -> Result<()> {
    // Create instruction to push task with the serialized data
    let mut instruction_data = vec![1]; // Push task instruction = 1
    instruction_data.extend_from_slice(task_data);

    let push_task_ix = Instruction::new_with_bytes(
        *program_id,
        &instruction_data,
        vec![AccountMeta::new(scheduler_account.pubkey(), false)],
    );

    // Send transaction
    let signers = &[payer];
    create_and_send_transaction(client, push_task_ix, signers, config)?;

    println!("Task pushed to scheduler!");
    Ok(())
}

/// Execute a task from the scheduler
pub fn execute_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    scheduler_account: &Keypair,
    config: &Config,
) -> Result<()> {
    // Create instruction to execute task
    let execute_task_ix = Instruction::new_with_bytes(
        *program_id,
        &[2], // Execute task instruction = 2
        vec![AccountMeta::new(scheduler_account.pubkey(), false)],
    );

    // Send transaction
    let signers = &[payer];
    create_and_send_transaction(client, execute_task_ix, signers, config)?;

    println!("Task executed!");
    Ok(())
}

/// Execute all tasks from the scheduler
pub fn execute_all_tasks(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    scheduler_account: &Keypair,
    config: &Config,
) -> Result<()> {
    // Create instruction to execute all tasks
    let execute_all_ix = Instruction::new_with_bytes(
        *program_id,
        &[3], // Execute all tasks instruction = 3
        vec![AccountMeta::new(scheduler_account.pubkey(), false)],
    );

    // Send transaction
    let signers = &[payer];
    create_and_send_transaction(client, execute_all_ix, signers, config)?;

    println!("All tasks executed!");
    Ok(())
}
