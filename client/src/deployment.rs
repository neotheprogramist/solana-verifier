use solana_client::rpc_client::RpcClient;
use solana_program::{bpf_loader_upgradeable, system_instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{fs, thread::sleep};

use crate::{
    account::{read_keypair_file, write_keypair_file},
    transaction::confirm_transaction_with_retries,
    ClientError, Config, Result,
};

// Constants for program and buffer sizes
const PROGRAM_DATA_OFFSET: usize = 45; // Offset of the actual program data in the buffer
const BUFFER_METADATA_SIZE: usize = 45; // Size of the buffer metadata

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

/// Deploy a program to the Solana blockchain
pub fn deploy_program(
    client: &RpcClient,
    payer: &Keypair,
    program_keypair: &Keypair,
    program_data: &[u8],
    config: &Config,
) -> Result<()> {
    // Create a buffer account to hold the program data
    let buffer_keypair = Keypair::new();
    println!(
        "Creating program buffer account: {}",
        buffer_keypair.pubkey()
    );

    // Write the program to the buffer account
    write_program_to_buffer(client, payer, &buffer_keypair, program_data, config)?;

    // Create the program account if it doesn't exist
    let program_id = program_keypair.pubkey();
    let program_account = client.get_account(&program_id);

    if program_account.is_err() {
        println!("Creating program account...");

        // Calculate rent for the program account
        let program_len = 36; // Fixed size for program account
        let program_rent = client
            .get_minimum_balance_for_rent_exemption(program_len)
            .map_err(|err| {
                ClientError::DeploymentError(format!("Failed to get rent exemption: {}", err))
            })?;

        // Deploy program instructions
        let deploy_program_ixs = bpf_loader_upgradeable::deploy_with_max_program_len(
            &payer.pubkey(),
            &program_id,
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            program_rent,
            config.buffer_chunk_size,
        )
        .map_err(|err| {
            ClientError::DeploymentError(format!("Failed to create deploy instruction: {}", err))
        })?;

        // Send each instruction in a separate transaction
        for (i, ix) in deploy_program_ixs.iter().enumerate() {
            let recent_blockhash = client.get_latest_blockhash().map_err(|err| {
                ClientError::DeploymentError(format!("Failed to get recent blockhash: {}", err))
            })?;

            let signers = if i == 0 {
                // First instruction (create_account) needs program_keypair
                vec![payer, program_keypair]
            } else {
                // Other instructions just need payer
                vec![payer]
            };

            let transaction = Transaction::new_signed_with_payer(
                &[ix.clone()],
                Some(&payer.pubkey()),
                &signers,
                recent_blockhash,
            );

            // Send and confirm transaction
            let signature = client.send_transaction(&transaction).map_err(|err| {
                ClientError::DeploymentError(format!("Failed to send transaction {}: {}", i, err))
            })?;

            println!(
                "Deploying program (step {}), signature: {}",
                i + 1,
                signature
            );
            confirm_transaction_with_retries(
                client,
                &signature,
                config.transaction_retry_count,
                config,
            )?;
        }
    } else {
        println!("Program account already exists, updating program...");

        // Upgrade program instruction
        let upgrade_ix = bpf_loader_upgradeable::upgrade(
            &program_id,
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            &payer.pubkey(),
        );

        // Create and send transaction
        let recent_blockhash = client.get_latest_blockhash().map_err(|err| {
            ClientError::DeploymentError(format!("Failed to get recent blockhash: {}", err))
        })?;

        let transaction = Transaction::new_signed_with_payer(
            &[upgrade_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let signature = client.send_transaction(&transaction).map_err(|err| {
            ClientError::DeploymentError(format!("Failed to send upgrade transaction: {}", err))
        })?;

        println!("Upgrading program, signature: {}", signature);
        confirm_transaction_with_retries(
            client,
            &signature,
            config.transaction_retry_count,
            config,
        )?;
    }

    Ok(())
}

/// Write program data to a buffer account
pub fn write_program_to_buffer(
    client: &RpcClient,
    payer: &Keypair,
    buffer_keypair: &Keypair,
    program_data: &[u8],
    config: &Config,
) -> Result<()> {
    // Calculate rent for the buffer account
    let buffer_size = program_data.len() + BUFFER_METADATA_SIZE;
    let buffer_rent = client
        .get_minimum_balance_for_rent_exemption(buffer_size)
        .map_err(|err| {
            ClientError::DeploymentError(format!("Failed to get rent exemption: {}", err))
        })?;

    // Create buffer account instruction
    let create_buffer_ix = system_instruction::create_account(
        &payer.pubkey(),
        &buffer_keypair.pubkey(),
        buffer_rent,
        buffer_size as u64,
        &bpf_loader_upgradeable::id(),
    );

    // Create transaction with create buffer instruction
    let recent_blockhash = client.get_latest_blockhash().map_err(|err| {
        ClientError::DeploymentError(format!("Failed to get recent blockhash: {}", err))
    })?;

    // Create instruction to initialize the buffer
    let init_buffer_ix = bpf_loader_upgradeable::write(
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        0,
        vec![0; BUFFER_METADATA_SIZE], // Write zeros to initialize the buffer metadata
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_buffer_ix, init_buffer_ix],
        Some(&payer.pubkey()),
        &[payer, buffer_keypair],
        recent_blockhash,
    );

    // Send and confirm transaction
    let signature = client.send_transaction(&transaction).map_err(|err| {
        ClientError::DeploymentError(format!("Failed to send buffer transaction: {}", err))
    })?;

    println!("Creating buffer account, signature: {}", signature);
    confirm_transaction_with_retries(client, &signature, config.transaction_retry_count, config)?;

    // Write program data to buffer in chunks
    let chunk_size = config.buffer_chunk_size;
    for (i, chunk) in program_data.chunks(chunk_size).enumerate() {
        let write_ix = bpf_loader_upgradeable::write(
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            (i * chunk_size + PROGRAM_DATA_OFFSET) as u32,
            chunk.to_vec(),
        );

        let recent_blockhash = client.get_latest_blockhash().map_err(|err| {
            ClientError::DeploymentError(format!("Failed to get recent blockhash: {}", err))
        })?;

        let transaction = Transaction::new_signed_with_payer(
            &[write_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let signature = client.send_transaction(&transaction).map_err(|err| {
            ClientError::DeploymentError(format!("Failed to send write transaction: {}", err))
        })?;

        println!(
            "Writing program chunk {}/{}, signature: {}",
            i + 1,
            (program_data.len() + chunk_size - 1) / chunk_size,
            signature
        );

        confirm_transaction_with_retries(
            client,
            &signature,
            config.transaction_retry_count,
            config,
        )?;

        // Small delay between chunks to avoid rate limiting
        sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
