use borsh::BorshDeserialize;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_program::{
    bpf_loader_upgradeable,
    instruction::{AccountMeta, Instruction},
    system_instruction,
};
use solana_sdk::{
    bpf_loader_upgradeable::UpgradeableLoaderState,
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{fs, path::Path, thread::sleep};

use crate::{ClientError, Config, Result};
use verifier::GreetingAccount;

/// Initialize the Solana RPC client and verify connection
pub fn initialize_client(config: &Config) -> Result<RpcClient> {
    println!("Using RPC URL: {}", config.rpc_url);

    let client = RpcClient::new_with_timeout_and_commitment(
        config.rpc_url.clone(),
        config.rpc_timeout_duration(),
        CommitmentConfig::confirmed(),
    );

    // Verify connection to validator
    client
        .get_version()
        .map(|version| {
            println!(
                "Connected to Solana validator version: {}",
                version.solana_core
            );
            client
        })
        .map_err(|err| {
            ClientError::ConnectionError(format!(
                "{}.\nPlease ensure a local validator is running with 'solana-test-validator'",
                err
            ))
        })
}

/// Setup the payer account, creating a new one and funding it if necessary
pub fn setup_payer(client: &RpcClient, config: &Config) -> Result<Keypair> {
    match read_keypair_file(&config.payer_keypair_path) {
        Ok(keypair) => {
            println!("Using existing payer keypair");
            Ok(keypair)
        }
        Err(_) => {
            let keypair = Keypair::new();
            write_keypair_file(&keypair, &config.payer_keypair_path)?;

            println!("Created new payer keypair: {}", keypair.pubkey());

            // Fund the account with airdrops
            request_and_confirm_airdrop(client, &keypair, config.airdrop_amount, config)?;

            request_and_confirm_airdrop(
                client,
                &keypair,
                config.airdrop_amount * config.additional_airdrop_multiplier,
                config,
            )?;

            println!(
                "Airdropped {} SOL to payer",
                (config.airdrop_amount * (1 + config.additional_airdrop_multiplier)) as f64
                    / 1_000_000_000.0
            );

            Ok(keypair)
        }
    }
}

/// Request an airdrop and confirm the transaction
pub fn request_and_confirm_airdrop(
    client: &RpcClient,
    keypair: &Keypair,
    amount: u64,
    config: &Config,
) -> Result<()> {
    let message = if amount == config.airdrop_amount {
        "Airdrop"
    } else {
        "Additional airdrop"
    };
    println!("{} requested, waiting for confirmation...", message);

    let sig = client
        .request_airdrop(&keypair.pubkey(), amount)
        .map_err(|e| {
            ClientError::TransactionError(format!(
                "Failed to request {} of {} SOL: {}",
                message,
                amount as f64 / 1_000_000_000.0,
                e
            ))
        })?;

    confirm_transaction_with_retries(client, &sig, config.transaction_retry_count, config)?;

    println!("{} confirmed!", message);
    Ok(())
}

/// Confirm a transaction with retries
pub fn confirm_transaction_with_retries(
    client: &RpcClient,
    signature: &solana_sdk::signature::Signature,
    retries: usize,
    config: &Config,
) -> Result<()> {
    for attempt in 1..=retries {
        match client.confirm_transaction(signature) {
            Ok(true) => return Ok(()),
            Ok(false) if attempt < retries => {
                sleep(config.retry_sleep_duration());
            }
            Ok(false) => {
                return Err(ClientError::TransactionError(format!(
                    "Transaction not confirmed after {} attempts",
                    retries
                )));
            }
            Err(err) if attempt < retries => {
                println!(
                    "Confirmation attempt {}/{} failed: {}",
                    attempt, retries, err
                );
                sleep(config.retry_sleep_duration());
            }
            Err(err) => {
                return Err(ClientError::TransactionError(format!(
                    "Failed to confirm transaction: {}",
                    err
                )));
            }
        }
    }

    Err(ClientError::TransactionError(format!(
        "Transaction confirmation failed after {} attempts",
        retries
    )))
}

/// Setup the program - either use existing deployment or deploy a new one
pub fn setup_program(
    client: &RpcClient,
    payer: &Keypair,
    config: &Config,
) -> Result<solana_sdk::pubkey::Pubkey> {
    // Read the program binary
    if !config.program_path.exists() {
        return Err(ClientError::ProgramNotFound(
            format!("Program binary not found at {}. Please build the program first with 'cargo build-sbf' in the verifier directory.", 
                config.program_path.display()
            )
        ));
    }

    let program_data = fs::read(&config.program_path).map_err(ClientError::IoError)?;
    println!("Program binary size: {} bytes", program_data.len());

    // Deploy the program or use existing deployment
    if config.program_keypair_path.exists() {
        let program_keypair = read_keypair_file(&config.program_keypair_path)?;
        let program_id = program_keypair.pubkey();

        // Check if the program is already deployed
        match client.get_account(&program_id) {
            Ok(_) => {
                println!("Program already deployed at ID: {}", program_id);
                Ok(program_id)
            }
            Err(_) => {
                println!("Deploying program with ID: {}", program_id);
                deploy_program(client, payer, &program_keypair, &program_data, config)?;
                println!("Program deployed successfully!");
                Ok(program_id)
            }
        }
    } else {
        // Create a new program deployment
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();
        println!("Deploying new program with ID: {}", program_id);

        deploy_program(client, payer, &program_keypair, &program_data, config)?;

        write_keypair_file(&program_keypair, &config.program_keypair_path)?;

        println!("Program deployed successfully!");
        Ok(program_id)
    }
}

/// Setup the greeting account - either use existing or create a new one
pub fn setup_greeting_account(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    config: &Config,
) -> Result<Keypair> {
    if config.greeting_keypair_path.exists() {
        let greeting_keypair = read_keypair_file(&config.greeting_keypair_path)?;
        println!(
            "Using existing greeting account: {}",
            greeting_keypair.pubkey()
        );
        Ok(greeting_keypair)
    } else {
        let greeting_keypair = Keypair::new();
        println!("Creating greeting account: {}", greeting_keypair.pubkey());

        // Calculate the space needed for the greeting account
        let space = std::mem::size_of::<GreetingAccount>();
        let rent = client
            .get_minimum_balance_for_rent_exemption(space)
            .map_err(ClientError::SolanaClientError)?;

        // Create a transaction to create the greeting account
        let create_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &greeting_keypair.pubkey(),
            rent,
            space as u64,
            program_id,
        );

        let blockhash = client
            .get_latest_blockhash()
            .map_err(ClientError::SolanaClientError)?;

        let create_tx = Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&payer.pubkey()),
            &[payer, &greeting_keypair],
            blockhash,
        );

        // Send and confirm the transaction
        let create_sig = client
            .send_and_confirm_transaction(&create_tx)
            .map_err(|e| {
                ClientError::TransactionError(format!(
                    "Failed to send and confirm greeting account creation transaction: {}",
                    e
                ))
            })?;
        println!("Created greeting account: {}", create_sig);

        // Save the keypair for future use
        write_keypair_file(&greeting_keypair, &config.greeting_keypair_path)?;

        Ok(greeting_keypair)
    }
}

/// Interact with the deployed program
pub fn interact_with_program(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    greeting_account: &Keypair,
) -> Result<()> {
    // Create an instruction to call the program
    let instruction = Instruction::new_with_bytes(
        *program_id,
        &[],
        vec![AccountMeta::new(greeting_account.pubkey(), false)],
    );

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .map_err(ClientError::SolanaClientError)?;

    // Create a transaction with the instruction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    // Send and confirm the transaction
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| {
            ClientError::TransactionError(format!(
                "Failed to send and confirm program interaction transaction: {}",
                e
            ))
        })?;
    println!("Transaction signature: {}", signature);

    // Read the greeting account data
    let account_data = client
        .get_account_data(&greeting_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let greeting_account_data = GreetingAccount::try_from_slice(&account_data)
        .map_err(|e| ClientError::BorshError(e.to_string()))?;
    println!("Greeting counter: {}", greeting_account_data.counter);

    Ok(())
}

/// Deploy a program to the blockchain using BPF loader
pub fn deploy_program(
    client: &RpcClient,
    payer: &Keypair,
    program_keypair: &Keypair,
    program_data: &[u8],
    config: &Config,
) -> Result<()> {
    println!("Deploying program...");

    // Calculate the buffer size needed
    let program_len = program_data.len();
    println!("Program size: {} bytes", program_len);

    // Create a buffer account
    let buffer_keypair = Keypair::new();
    println!("Creating buffer account: {}", buffer_keypair.pubkey());

    // Calculate rent for the buffer
    let buffer_data_len = program_len;
    let buffer_balance = client
        .get_minimum_balance_for_rent_exemption(
            buffer_data_len + UpgradeableLoaderState::size_of_buffer_metadata(),
        )
        .map_err(ClientError::SolanaClientError)?;

    // Create buffer account
    let create_buffer_ix = bpf_loader_upgradeable::create_buffer(
        &payer.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        buffer_balance,
        buffer_data_len,
    )
    .map_err(|e| ClientError::DeploymentError(e.to_string()))?;

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .map_err(ClientError::SolanaClientError)?;

    // Create and send transaction
    let create_buffer_tx = Transaction::new_signed_with_payer(
        &create_buffer_ix,
        Some(&payer.pubkey()),
        &[payer, &buffer_keypair],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&create_buffer_tx)
        .map_err(|e| {
            ClientError::TransactionError(format!("Failed to create buffer account: {}", e))
        })?;
    println!("Buffer account created: {}", signature);

    // Write program data to the buffer account in chunks
    write_program_to_buffer(client, payer, &buffer_keypair, program_data, config)?;

    // Calculate rent for the program data
    let programdata_len = program_len;
    let programdata_balance = client
        .get_minimum_balance_for_rent_exemption(
            programdata_len + UpgradeableLoaderState::size_of_programdata_metadata(),
        )
        .map_err(ClientError::SolanaClientError)?;

    // Create deploy instruction
    let deploy_ix = bpf_loader_upgradeable::deploy_with_max_program_len(
        &payer.pubkey(),
        &program_keypair.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        programdata_balance,
        programdata_len,
    )
    .map_err(|e| ClientError::DeploymentError(e.to_string()))?;

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .map_err(ClientError::SolanaClientError)?;

    // Create and send transaction
    let deploy_tx = Transaction::new_signed_with_payer(
        &deploy_ix,
        Some(&payer.pubkey()),
        &[payer, program_keypair],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&deploy_tx)
        .map_err(|e| ClientError::TransactionError(format!("Failed to deploy program: {}", e)))?;
    println!("Program deployed: {}", signature);

    Ok(())
}

/// Write program data to buffer in chunks
pub fn write_program_to_buffer(
    client: &RpcClient,
    payer: &Keypair,
    buffer_keypair: &Keypair,
    program_data: &[u8],
    config: &Config,
) -> Result<()> {
    let mut offset = 0;

    while offset < program_data.len() {
        let chunk_end = std::cmp::min(offset + config.buffer_chunk_size, program_data.len());
        let chunk = &program_data[offset..chunk_end];

        let write_ix = bpf_loader_upgradeable::write(
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            offset as u32,
            chunk.to_vec(),
        );

        // Get latest blockhash for each chunk to avoid expired blockhash issues
        let blockhash = client
            .get_latest_blockhash()
            .map_err(ClientError::SolanaClientError)?;

        let write_tx = Transaction::new_signed_with_payer(
            &[write_ix],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&write_tx)
            .map_err(|e| {
                ClientError::TransactionError(format!(
                    "Failed to write chunk at offset {}: {}",
                    offset, e
                ))
            })?;
        println!(
            "Wrote chunk at offset {}/{}: tx signature: {}",
            offset,
            program_data.len(),
            signature
        );

        offset = chunk_end;
    }

    Ok(())
}

/// Read a keypair from file with improved error handling
pub fn read_keypair_file<P: AsRef<Path>>(path: P) -> Result<Keypair> {
    let file_content = fs::read_to_string(&path).map_err(ClientError::IoError)?;

    let bytes: Vec<u8> = serde_json::from_str(&file_content).map_err(ClientError::SerdeError)?;

    Keypair::from_bytes(&bytes).map_err(|e| {
        ClientError::KeypairError(format!("Failed to create keypair from bytes: {}", e))
    })
}

/// Write a keypair to file with improved error handling
pub fn write_keypair_file<P: AsRef<Path>>(keypair: &Keypair, path: P) -> Result<()> {
    let json =
        serde_json::to_string(&keypair.to_bytes().to_vec()).map_err(ClientError::SerdeError)?;
    fs::write(&path, json).map_err(ClientError::IoError)
}
