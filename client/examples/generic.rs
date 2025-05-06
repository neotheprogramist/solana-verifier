use client::{
    initialize_client, interact_with_program, send_instruction, setup_account, setup_payer,
    setup_program, ClientError, Config, ProgramInteraction, Result,
};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::path::Path;

/// Example of a generic program interaction
pub struct GenericProgramInteraction {
    /// Instruction code (first byte of instruction data)
    pub instruction_code: u8,
    /// Additional instruction data
    pub additional_data: Vec<u8>,
}

impl ProgramInteraction for GenericProgramInteraction {
    fn process_account_data(client: &RpcClient, account: &Keypair) -> Result<()> {
        // Get the account data
        let account_data = client
            .get_account_data(&account.pubkey())
            .map_err(ClientError::SolanaClientError)?;

        println!("Account data size: {} bytes", account_data.len());
        println!(
            "First few bytes: {:?}",
            &account_data[..std::cmp::min(10, account_data.len())]
        );

        Ok(())
    }

    fn get_instruction_data(&self) -> Vec<u8> {
        // Combine instruction code with additional data
        let mut data = vec![self.instruction_code];
        data.extend_from_slice(&self.additional_data);
        data
    }

    fn get_accounts(&self, account: &Keypair) -> Vec<AccountMeta> {
        // Override the default implementation to show customization
        vec![
            AccountMeta::new(account.pubkey(), false),
            // Add additional accounts if needed
        ]
    }
}

/// Example of how to send a custom instruction directly
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

    // Create a generic interaction with instruction code 1 and some additional data
    let interaction = GenericProgramInteraction {
        instruction_code: 1,
        additional_data: vec![10, 20, 30, 40],
    };

    // Interact with the program using the generic function
    interact_with_program(&client, &payer, &program_id, &program_account, &interaction)?;

    // Example of sending a custom instruction directly
    send_custom_instruction(&client, &payer, &program_id, &program_account, 2)?;

    println!("Generic program interaction completed successfully!");

    Ok(())
}
