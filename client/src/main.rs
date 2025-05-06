use anyhow::{anyhow, Context, Result};
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
use std::{fs, path::Path, thread::sleep, time::Duration};
use verifier::GreetingAccount;

// Configuration constants
const RPC_URL: &str = "http://localhost:8899";
const RPC_TIMEOUT_SECS: u64 = 30;
const PROGRAM_PATH: &str = "../target/deploy/verifier.so";
const PAYER_KEYPAIR_PATH: &str = "payer-keypair.json";
const PROGRAM_KEYPAIR_PATH: &str = "program-keypair.json";
const GREETING_KEYPAIR_PATH: &str = "greeting-keypair.json";
const AIRDROP_AMOUNT: u64 = 2_000_000_000; // 2 SOL
const ADDITIONAL_AIRDROP_MULTIPLIER: u64 = 5;
const TRANSACTION_RETRY_COUNT: usize = 10;
const RETRY_SLEEP_DURATION: Duration = Duration::from_secs(1);
const BUFFER_CHUNK_SIZE: usize = 900; // Slightly less than 1KB to account for transaction overhead

/// Main entry point for the Solana program client
fn main() -> Result<()> {
    // Initialize the Solana client
    let client = initialize_client().context("Failed to initialize Solana client")?;

    // Setup the payer account
    let payer = setup_payer(&client).context("Failed to setup payer account")?;

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer).context("Failed to setup program")?;

    // Setup greeting account
    let greeting_account = setup_greeting_account(&client, &payer, &program_id)
        .context("Failed to setup greeting account")?;

    // Interact with the program
    interact_with_program(&client, &payer, &program_id, &greeting_account)
        .context("Failed to interact with program")?;

    Ok(())
}

/// Initialize the Solana RPC client and verify connection
fn initialize_client() -> Result<RpcClient> {
    println!("Using RPC URL: {}", RPC_URL);

    let client = RpcClient::new_with_timeout_and_commitment(
        RPC_URL.to_string(),
        Duration::from_secs(RPC_TIMEOUT_SECS),
        CommitmentConfig::confirmed(),
    );

    // Verify connection to validator
    client.get_version()
        .map(|version| {
            println!("Connected to Solana validator version: {}", version.solana_core);
            client
        })
        .map_err(|err| {
            anyhow!(
                "Failed to connect to Solana validator: {}.\nPlease ensure a local validator is running with 'solana-test-validator'",
                err
            )
        })
}

/// Setup the payer account, creating a new one and funding it if necessary
fn setup_payer(client: &RpcClient) -> Result<Keypair> {
    match read_keypair_file(PAYER_KEYPAIR_PATH) {
        Ok(keypair) => {
            println!("Using existing payer keypair");
            Ok(keypair)
        }
        Err(_) => {
            let keypair = Keypair::new();
            write_keypair_file(&keypair, PAYER_KEYPAIR_PATH)
                .context("Failed to write new payer keypair to file")?;

            println!("Created new payer keypair: {}", keypair.pubkey());

            // Fund the account with airdrops
            request_and_confirm_airdrop(client, &keypair, AIRDROP_AMOUNT)
                .context("Failed to process initial airdrop")?;

            request_and_confirm_airdrop(
                client,
                &keypair,
                AIRDROP_AMOUNT * ADDITIONAL_AIRDROP_MULTIPLIER,
            )
            .context("Failed to process additional airdrop")?;

            println!(
                "Airdropped {} SOL to payer",
                (AIRDROP_AMOUNT * (1 + ADDITIONAL_AIRDROP_MULTIPLIER)) as f64 / 1_000_000_000.0
            );

            Ok(keypair)
        }
    }
}

/// Request an airdrop and confirm the transaction
fn request_and_confirm_airdrop(client: &RpcClient, keypair: &Keypair, amount: u64) -> Result<()> {
    let message = if amount == AIRDROP_AMOUNT {
        "Airdrop"
    } else {
        "Additional airdrop"
    };
    println!("{} requested, waiting for confirmation...", message);

    let sig = client
        .request_airdrop(&keypair.pubkey(), amount)
        .context(format!(
            "Failed to request {} of {} SOL",
            message,
            amount as f64 / 1_000_000_000.0
        ))?;

    confirm_transaction_with_retries(client, &sig, TRANSACTION_RETRY_COUNT)
        .context(format!("Failed to confirm {} transaction", message))?;

    println!("{} confirmed!", message);
    Ok(())
}

/// Confirm a transaction with retries
fn confirm_transaction_with_retries(
    client: &RpcClient,
    signature: &solana_sdk::signature::Signature,
    retries: usize,
) -> Result<()> {
    for attempt in 1..=retries {
        match client.confirm_transaction(signature) {
            Ok(true) => return Ok(()),
            Ok(false) if attempt < retries => {
                sleep(RETRY_SLEEP_DURATION);
            }
            Ok(false) => {
                return Err(anyhow!(
                    "Transaction not confirmed after {} attempts",
                    retries
                ));
            }
            Err(err) if attempt < retries => {
                println!(
                    "Confirmation attempt {}/{} failed: {}",
                    attempt, retries, err
                );
                sleep(RETRY_SLEEP_DURATION);
            }
            Err(err) => {
                return Err(anyhow!("Failed to confirm transaction: {}", err));
            }
        }
    }

    Err(anyhow!(
        "Transaction confirmation failed after {} attempts",
        retries
    ))
}

/// Setup the program - either use existing deployment or deploy a new one
fn setup_program(client: &RpcClient, payer: &Keypair) -> Result<solana_sdk::pubkey::Pubkey> {
    // Read the program binary
    if !Path::new(PROGRAM_PATH).exists() {
        return Err(anyhow!(
            "Program binary not found at {}. Please build the program first with 'cargo build-sbf' in the verifier directory.",
            PROGRAM_PATH
        ));
    }

    let program_data = fs::read(PROGRAM_PATH)
        .context(format!("Failed to read program binary at {}", PROGRAM_PATH))?;
    println!("Program binary size: {} bytes", program_data.len());

    // Deploy the program or use existing deployment
    if Path::new(PROGRAM_KEYPAIR_PATH).exists() {
        let program_keypair = read_keypair_file(PROGRAM_KEYPAIR_PATH).context(format!(
            "Failed to read program keypair from {}",
            PROGRAM_KEYPAIR_PATH
        ))?;
        let program_id = program_keypair.pubkey();

        // Check if the program is already deployed
        match client.get_account(&program_id) {
            Ok(_) => {
                println!("Program already deployed at ID: {}", program_id);
                Ok(program_id)
            }
            Err(_) => {
                println!("Deploying program with ID: {}", program_id);
                deploy_program(client, payer, &program_keypair, &program_data)
                    .context("Failed to deploy program")?;
                println!("Program deployed successfully!");
                Ok(program_id)
            }
        }
    } else {
        // Create a new program deployment
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();
        println!("Deploying new program with ID: {}", program_id);

        deploy_program(client, payer, &program_keypair, &program_data)
            .context("Failed to deploy program")?;

        write_keypair_file(&program_keypair, PROGRAM_KEYPAIR_PATH).context(format!(
            "Failed to write program keypair to {}",
            PROGRAM_KEYPAIR_PATH
        ))?;

        println!("Program deployed successfully!");
        Ok(program_id)
    }
}

/// Setup the greeting account - either use existing or create a new one
fn setup_greeting_account(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
) -> Result<Keypair> {
    if Path::new(GREETING_KEYPAIR_PATH).exists() {
        let greeting_keypair = read_keypair_file(GREETING_KEYPAIR_PATH).context(format!(
            "Failed to read greeting keypair from {}",
            GREETING_KEYPAIR_PATH
        ))?;
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
            .context("Failed to calculate rent for greeting account")?;

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
            .context("Failed to get latest blockhash")?;

        let create_tx = Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&payer.pubkey()),
            &[payer, &greeting_keypair],
            blockhash,
        );

        // Send and confirm the transaction
        let create_sig = client
            .send_and_confirm_transaction(&create_tx)
            .context("Failed to send and confirm greeting account creation transaction")?;
        println!("Created greeting account: {}", create_sig);

        // Save the keypair for future use
        write_keypair_file(&greeting_keypair, GREETING_KEYPAIR_PATH).context(format!(
            "Failed to write greeting keypair to {}",
            GREETING_KEYPAIR_PATH
        ))?;

        Ok(greeting_keypair)
    }
}

/// Interact with the deployed program
fn interact_with_program(
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
        .context("Failed to get latest blockhash")?;

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
        .context("Failed to send and confirm program interaction transaction")?;
    println!("Transaction signature: {}", signature);

    // Read the greeting account data
    let account_data = client
        .get_account_data(&greeting_account.pubkey())
        .context("Failed to get greeting account data")?;
    let greeting_account_data = GreetingAccount::try_from_slice(&account_data)
        .context("Failed to deserialize greeting account data")?;
    println!("Greeting counter: {}", greeting_account_data.counter);

    Ok(())
}

/// Deploy a program to the blockchain using BPF loader
fn deploy_program(
    client: &RpcClient,
    payer: &Keypair,
    program_keypair: &Keypair,
    program_data: &[u8],
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
        .context("Failed to calculate buffer rent")?;

    // Create buffer account
    let create_buffer_ix = bpf_loader_upgradeable::create_buffer(
        &payer.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        buffer_balance,
        buffer_data_len,
    )
    .context("Failed to create buffer instruction")?;

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .context("Failed to get latest blockhash for buffer creation")?;

    // Create and send transaction
    let create_buffer_tx = Transaction::new_signed_with_payer(
        &create_buffer_ix,
        Some(&payer.pubkey()),
        &[payer, &buffer_keypair],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&create_buffer_tx)
        .context("Failed to create buffer account")?;
    println!("Buffer account created: {}", signature);

    // Write program data to the buffer account in chunks
    write_program_to_buffer(client, payer, &buffer_keypair, program_data)
        .context("Failed to write program data to buffer")?;

    // Calculate rent for the program data
    let programdata_len = program_len;
    let programdata_balance = client
        .get_minimum_balance_for_rent_exemption(
            programdata_len + UpgradeableLoaderState::size_of_programdata_metadata(),
        )
        .context("Failed to calculate program data rent")?;

    // Create deploy instruction
    let deploy_ix = bpf_loader_upgradeable::deploy_with_max_program_len(
        &payer.pubkey(),
        &program_keypair.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        programdata_balance,
        programdata_len,
    )
    .context("Failed to create deploy instruction")?;

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .context("Failed to get latest blockhash for deployment")?;

    // Create and send transaction
    let deploy_tx = Transaction::new_signed_with_payer(
        &deploy_ix,
        Some(&payer.pubkey()),
        &[payer, program_keypair],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&deploy_tx)
        .context("Failed to deploy program")?;
    println!("Program deployed: {}", signature);

    Ok(())
}

/// Write program data to buffer in chunks
fn write_program_to_buffer(
    client: &RpcClient,
    payer: &Keypair,
    buffer_keypair: &Keypair,
    program_data: &[u8],
) -> Result<()> {
    let mut offset = 0;

    while offset < program_data.len() {
        let chunk_end = std::cmp::min(offset + BUFFER_CHUNK_SIZE, program_data.len());
        let chunk = &program_data[offset..chunk_end];

        let write_ix = bpf_loader_upgradeable::write(
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            offset as u32,
            chunk.to_vec(),
        );

        // Get latest blockhash for each chunk to avoid expired blockhash issues
        let blockhash = client.get_latest_blockhash().context(format!(
            "Failed to get blockhash for chunk at offset {}",
            offset
        ))?;

        let write_tx = Transaction::new_signed_with_payer(
            &[write_ix],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&write_tx)
            .context(format!("Failed to write chunk at offset {}", offset))?;
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
fn read_keypair_file(path: &str) -> Result<Keypair> {
    let file_content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read keypair file '{}': {}", path, e))?;

    let bytes: Vec<u8> = serde_json::from_str(&file_content)
        .map_err(|e| anyhow!("Failed to parse keypair JSON from '{}': {}", path, e))?;

    Keypair::from_bytes(&bytes)
        .map_err(|e| anyhow!("Failed to create keypair from bytes in '{}': {}", path, e))
}

/// Write a keypair to file with improved error handling
fn write_keypair_file(keypair: &Keypair, path: &str) -> Result<()> {
    let json = serde_json::to_string(&keypair.to_bytes().to_vec())
        .map_err(|e| anyhow!("Failed to serialize keypair: {}", e))?;
    fs::write(path, json).map_err(|e| anyhow!("Failed to write keypair to file '{}': {}", path, e))
}
