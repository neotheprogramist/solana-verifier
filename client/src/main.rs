use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use solana_program::{
    bpf_loader_upgradeable,
    instruction::{AccountMeta, Instruction},
    system_instruction,
};
use solana_sdk::{
    bpf_loader_upgradeable::UpgradeableLoaderState, commitment_config::CommitmentConfig, signature::{Keypair, Signer, read_keypair_file as solana_read_keypair_file}, transaction::Transaction
};
use std::{fs, path::Path, time::Duration};
use verifier::GreetingAccount;

fn main() -> Result<()> {
    // Use local test validator instead of the Solana CLI config
    let rpc_url = "http://localhost:8899".to_string();
    println!("Using RPC URL: {}", rpc_url);
    
    // Create client with timeout for better error messages
    let client = RpcClient::new_with_timeout_and_commitment(
        rpc_url,
        Duration::from_secs(30),
        CommitmentConfig::confirmed(),
    );
    
    // Check if the validator is running
    match client.get_version() {
        Ok(version) => println!("Connected to Solana validator version: {}", version.solana_core),
        Err(err) => {
            return Err(anyhow!(
                "Failed to connect to Solana validator: {}.\nPlease ensure a local validator is running with 'solana-test-validator'",
                err
            ));
        }
    }

    // Load or create a payer keypair
    let payer = match read_keypair_file("payer-keypair.json") {
        Ok(keypair) => {
            println!("Using existing payer keypair");
            keypair
        },
        Err(_) => {
            let keypair = Keypair::new();
            write_keypair_file(&keypair, "payer-keypair.json")?;
            println!("Created new payer keypair: {}", keypair.pubkey());

            // Airdrop SOL to the payer
            let airdrop_amount = 2_000_000_000; // 2 SOL
            let sig = client.request_airdrop(&keypair.pubkey(), airdrop_amount)?;
            println!("Airdrop requested, waiting for confirmation...");
            
            // Wait for confirmation with retries
            let mut retries = 10;
            while retries > 0 {
                match client.confirm_transaction(&sig) {
                    Ok(confirmed) => {
                        if confirmed {
                            println!("Airdrop confirmed!");
                            break;
                        }
                    },
                    Err(_) => {}
                }
                retries -= 1;
                std::thread::sleep(Duration::from_secs(1));
                if retries == 0 {
                    return Err(anyhow!("Failed to confirm airdrop transaction"));
                }
            }
            
            // Request another airdrop for more funds
            let sig = client.request_airdrop(&keypair.pubkey(), airdrop_amount * 5)?;
            println!("Additional airdrop requested, waiting for confirmation...");
            
            // Wait for confirmation with retries
            let mut retries = 10;
            while retries > 0 {
                match client.confirm_transaction(&sig) {
                    Ok(confirmed) => {
                        if confirmed {
                            println!("Additional airdrop confirmed!");
                            break;
                        }
                    },
                    Err(_) => {}
                }
                retries -= 1;
                std::thread::sleep(Duration::from_secs(1));
                if retries == 0 {
                    return Err(anyhow!("Failed to confirm additional airdrop transaction"));
                }
            }
            
            println!(
                "Airdropped {} SOL to payer",
                (airdrop_amount * 6) as f64 / 1_000_000_000.0
            );

            keypair
        }
    };
    println!("Using payer: {}", payer.pubkey());

    // Read the program binary
    let program_path = "../target/deploy/verifier.so";
    if !Path::new(program_path).exists() {
        return Err(anyhow!("Program binary not found. Please build the program first with 'cargo build-sbf' in the verifier directory."));
    }

    let program_data = fs::read(program_path)?;
    println!("Program binary size: {} bytes", program_data.len());

    // Deploy the program or use existing deployment
    let program_id = if Path::new("program-keypair.json").exists() {
        let program_keypair = read_keypair_file("program-keypair.json")?;
        let program_id = program_keypair.pubkey();
        
        // Check if the program is already deployed
        match client.get_account(&program_id) {
            Ok(_) => {
                println!("Program already deployed at ID: {}", program_id);
                program_id
            },
            Err(_) => {
                println!("Deploying program with ID: {}", program_id);
                deploy_program(&client, &payer, &program_keypair, &program_data)?;
                println!("Program deployed successfully!");
                program_id
            }
        }
    } else {
        // Create a new program deployment
        let program_keypair = Keypair::new();
        println!("Deploying program with ID: {}", program_keypair.pubkey());

        deploy_program(&client, &payer, &program_keypair, &program_data)?;
        println!("Program deployed successfully!");

        write_keypair_file(&program_keypair, "program-keypair.json")?;
        program_keypair.pubkey()
    };

    println!("Using program ID: {}", program_id);

    // Create or load a greeting account
    let greeting_account = if Path::new("greeting-keypair.json").exists() {
        let greeting_keypair = read_keypair_file("greeting-keypair.json")?;
        println!("Using existing greeting account: {}", greeting_keypair.pubkey());
        greeting_keypair
    } else {
        let greeting_keypair = Keypair::new();
        println!("Creating greeting account: {}", greeting_keypair.pubkey());

        // Calculate the space needed for the greeting account
        let space = std::mem::size_of::<GreetingAccount>();
        let rent = client.get_minimum_balance_for_rent_exemption(space)?;

        // Create a transaction to create the greeting account
        let create_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &greeting_keypair.pubkey(),
            rent,
            space as u64,
            &program_id,
        );

        let create_tx = Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&payer.pubkey()),
            &[&payer, &greeting_keypair],
            client.get_latest_blockhash()?,
        );

        // Send and confirm the transaction
        let create_sig = client.send_and_confirm_transaction(&create_tx)?;
        println!("Created greeting account: {}", create_sig);
        
        // Save the keypair for future use
        write_keypair_file(&greeting_keypair, "greeting-keypair.json")?;
        
        greeting_keypair
    };

    // Now interact with the program
    // Create an instruction to call the program
    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        vec![AccountMeta::new(greeting_account.pubkey(), false)],
    );

    // Create a transaction with the instruction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash()?,
    );

    // Send and confirm the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);

    // Read the greeting account data
    let account_data = client.get_account_data(&greeting_account.pubkey())?;
    let greeting_account_data = GreetingAccount::try_from_slice(&account_data)?;
    println!("Greeting counter: {}", greeting_account_data.counter);

    // Ask user if they want to increment the counter again
    println!("\nDo you want to increment the counter again? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() == "y" {
        // Call the program again to increment the counter
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![AccountMeta::new(greeting_account.pubkey(), false)],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash()?,
        );

        let signature = client.send_and_confirm_transaction(&transaction)?;
        println!("Second transaction signature: {}", signature);

        // Read the greeting account data again
        let account_data = client.get_account_data(&greeting_account.pubkey())?;
        let greeting_account_data = GreetingAccount::try_from_slice(&account_data)?;
        println!(
            "Greeting counter after second call: {}",
            greeting_account_data.counter
        );
    }

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

    // Calculate rent for the program
    let program_rent = client.get_minimum_balance_for_rent_exemption(program_len)?;
    println!("Program rent: {} lamports", program_rent);

    // Create a buffer account
    let buffer_keypair = Keypair::new();
    println!("Creating buffer account: {}", buffer_keypair.pubkey());

    // Calculate rent for the buffer
    let buffer_data_len = program_len;
    let buffer_balance = client.get_minimum_balance_for_rent_exemption(
        buffer_data_len + UpgradeableLoaderState::size_of_buffer_metadata(),
    )?;

    // Create buffer account
    let create_buffer_ix = bpf_loader_upgradeable::create_buffer(
        &payer.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        buffer_balance,
        buffer_data_len,
    )?;

    // Create and send transaction
    let create_buffer_tx = Transaction::new_signed_with_payer(
        &create_buffer_ix,
        Some(&payer.pubkey()),
        &[payer, &buffer_keypair],
        client.get_latest_blockhash()?,
    );

    let signature = client.send_and_confirm_transaction(&create_buffer_tx)?;
    println!("Buffer account created: {}", signature);

    // Write program data to the buffer account
    // We need to split the program into chunks to avoid transaction size limits
    const CHUNK_SIZE: usize = 900; // Slightly less than 1KB to account for transaction overhead
    let mut offset = 0;

    while offset < program_data.len() {
        let chunk_end = std::cmp::min(offset + CHUNK_SIZE, program_data.len());
        let chunk = &program_data[offset..chunk_end];

        let write_ix = bpf_loader_upgradeable::write(
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            offset as u32,
            chunk.to_vec(),
        );

        let write_tx = Transaction::new_signed_with_payer(
            &[write_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash()?,
        );

        let signature = client.send_and_confirm_transaction(&write_tx)?;
        println!("Wrote chunk at offset {}: {}", offset, signature);

        offset = chunk_end;
    }

    // Create program account
    let program_account_keypair = program_keypair;
    println!(
        "Creating program account: {}",
        program_account_keypair.pubkey()
    );

    // Calculate rent for the program
    let programdata_len = program_len;
    let programdata_balance = client.get_minimum_balance_for_rent_exemption(
        programdata_len + UpgradeableLoaderState::size_of_programdata_metadata(),
    )?;

    // Create deploy instruction
    let deploy_ix = bpf_loader_upgradeable::deploy_with_max_program_len(
        &payer.pubkey(),
        &program_account_keypair.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        programdata_balance,
        programdata_len,
    )?;

    // Create and send transaction
    let deploy_tx = Transaction::new_signed_with_payer(
        &deploy_ix,
        Some(&payer.pubkey()),
        &[payer, program_account_keypair],
        client.get_latest_blockhash()?,
    );

    let signature = client.send_and_confirm_transaction(&deploy_tx)?;
    println!("Program deployed: {}", signature);

    Ok(())
}

fn read_keypair_file(path: &str) -> Result<Keypair> {
    solana_read_keypair_file(path).map_err(|e| anyhow!("Failed to read keypair file: {}", e))
}

fn write_keypair_file(keypair: &Keypair, path: &str) -> Result<()> {
    fs::write(path, keypair.to_bytes())?;
    Ok(())
}
