use borsh::BorshDeserialize;
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

/// Setup the greeting account for the Solana program
pub fn setup_greeting_account(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    config: &Config,
) -> Result<Keypair> {
    match read_keypair_file(&config.greeting_keypair_path) {
        Ok(keypair) => {
            println!("Using existing greeting account: {}", keypair.pubkey());
            Ok(keypair)
        }
        Err(_) => {
            let greeting_keypair = Keypair::new();
            println!(
                "Creating new greeting account: {}",
                greeting_keypair.pubkey()
            );

            // Calculate rent for the greeting account
            let greeting_size = 48; // Size of the greeting account data
            let rent = client
                .get_minimum_balance_for_rent_exemption(greeting_size)
                .map_err(|err| {
                    ClientError::TransactionError(format!(
                        "Failed to get rent exemption for greeting account: {}",
                        err
                    ))
                })?;

            // Create system account
            let create_ix = system_instruction::create_account(
                &payer.pubkey(),
                &greeting_keypair.pubkey(),
                rent,
                greeting_size as u64,
                program_id,
            );

            // Initialize greeting account
            let initialize_ix = Instruction::new_with_bytes(
                *program_id,
                &[0], // Initialize instruction = 0
                vec![AccountMeta::new(greeting_keypair.pubkey(), false)],
            );

            // Create and send transaction
            let signers = &[payer, &greeting_keypair];
            create_and_send_transaction(client, initialize_ix, signers, config)?;

            // Save the keypair
            write_keypair_file(&greeting_keypair, &config.greeting_keypair_path)?;
            println!("Greeting account created and initialized!");

            Ok(greeting_keypair)
        }
    }
}

/// Interact with the greeting program
pub fn interact_with_program(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    greeting_account: &Keypair,
    config: &Config,
) -> Result<()> {
    // Create instruction to say hello
    let hello_ix = Instruction::new_with_bytes(
        *program_id,
        &[1], // Say hello instruction = 1
        vec![AccountMeta::new(greeting_account.pubkey(), false)],
    );

    // Send transaction
    let signers = &[payer];
    create_and_send_transaction(client, hello_ix, signers, config)?;

    // Get the updated greeting account data
    let account_data = client
        .get_account_data(&greeting_account.pubkey())
        .map_err(|err| {
            ClientError::TransactionError(format!("Failed to get greeting account data: {}", err))
        })?;

    // Deserialize the greeting account data
    let greeting_data = match greeting::state::GreetingAccount::try_from_slice(&account_data) {
        Ok(data) => data,
        Err(err) => {
            return Err(ClientError::BorshError(format!(
                "Failed to deserialize greeting account data: {}",
                err
            )));
        }
    };

    println!("Greeting count: {}", greeting_data.counter);
    Ok(())
}
