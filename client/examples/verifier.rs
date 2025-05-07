use arithmetic::add::Add;
use client::{
    initialize_client, interact_with_program_instructions, setup_account, setup_payer,
    setup_program, ClientError, Config,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signer::Signer,
};
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack};
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

    // Setup verifier account
    let space = size_of::<BidirectionalStackAccount>();
    println!("Verifier account space: {}", space);
    let verifier_account = setup_account(
        &client,
        &payer,
        &program_id,
        &config,
        space,
        "verifier-account",
    )?;

    // Create an Add task with operands 42 and 58
    let add_task = Add::new(42, 58);

    // Serialize the task using CBOR
    let mut task_data = Vec::new();
    ciborium::ser::into_writer(&add_task, &mut task_data)
        .map_err(|e| ClientError::SerializationError(format!("Failed to serialize task: {}", e)))?;

    println!("Serialized task size: {} bytes", task_data.len());

    // Create instructions
    let instructions = vec![
        // Push the Add task to the stack
        Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushTask(task_data),
            vec![AccountMeta::new(verifier_account.pubkey(), false)],
        ),
        // Execute the task
        Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::Execute,
            vec![AccountMeta::new(verifier_account.pubkey(), false)],
        ),
    ];

    // Interact with the program using the instructions directly
    interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &verifier_account,
        &instructions,
    )?;

    println!("Verifier program interaction completed successfully!");

    // Get the account data to check the result
    let mut account_data = client
        .get_account_data(&verifier_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack_account = BidirectionalStackAccount::cast_mut(&mut account_data);

    // Read the result from the front of the stack
    let result: u128 = ciborium::de::from_reader(stack_account.borrow_front()).map_err(|e| {
        ClientError::SerializationError(format!("Failed to deserialize result: {}", e))
    })?;

    println!("Stack front index: {}", stack_account.front_index);
    println!("Stack back index: {}", stack_account.back_index);
    println!("Result of 42 + 58 = {}", result);

    Ok(())
}
