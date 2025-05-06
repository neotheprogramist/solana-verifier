use client::{
    initialize_client, interact_with_program_instructions, send_instruction, setup_account,
    setup_payer, setup_program, ClientError, Config, Result,
};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::path::Path;

/// Send a custom instruction directly
fn send_custom_instruction(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    account: &Keypair,
    instruction_code: u8,
) -> Result<()> {
    // Create instruction data
    let instruction_data = vec![instruction_code, 1, 2, 3, 4];

    // Create accounts
    let accounts = vec![
        AccountMeta::new(account.pubkey(), false),
        // Add more accounts as needed
    ];

    // Send the instruction
    let signature = send_instruction(client, payer, program_id, accounts, &instruction_data)?;

    println!(
        "Custom instruction sent successfully with signature: {}",
        signature
    );

    Ok(())
}

/// Main entry point for the generic program client
fn main() -> Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Define program path (replace with your program path)
    let program_path = Path::new("target/deploy/program.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path)?;

    // Setup program account
    let space = 1024; // Set appropriate space for your program
    let program_account = setup_account(
        &client,
        &payer,
        &program_id,
        &config,
        space,
        "generic-account",
    )?;

    // Create instruction with instruction code 1 and some additional data
    let mut instruction_data = vec![1]; // instruction code
    instruction_data.extend_from_slice(&[10, 20, 30, 40]); // additional data

    let instructions = vec![Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![AccountMeta::new(program_account.pubkey(), false)],
    )];

    // Interact with the program using instructions directly
    interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &program_account,
        &instructions,
    )?;

    // Example of sending a custom instruction directly
    send_custom_instruction(&client, &payer, &program_id, &program_account, 2)?;

    println!("Generic program interaction completed successfully!");

    // Get the account data
    let account_data = client
        .get_account_data(&program_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;

    println!("Account data size: {} bytes", account_data.len());
    println!(
        "First few bytes: {:?}",
        &account_data[..std::cmp::min(10, account_data.len())]
    );

    Ok(())
}
