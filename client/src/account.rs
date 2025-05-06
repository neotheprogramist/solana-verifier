use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use std::{fs, path::Path, thread::sleep};

use crate::{transaction::confirm_transaction_with_retries, ClientError, Config, Result};

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

/// Read a keypair from a file
pub fn read_keypair_file<P: AsRef<Path>>(path: P) -> Result<Keypair> {
    let file_data = fs::read(&path).map_err(|_| {
        ClientError::KeypairError(format!("Failed to read keypair file: {:?}", path.as_ref()))
    })?;
    let keypair = solana_sdk::signer::keypair::Keypair::from_bytes(&file_data)
        .map_err(|e| ClientError::KeypairError(format!("Invalid keypair file: {}", e)))?;
    Ok(keypair)
}

/// Write a keypair to a file
pub fn write_keypair_file<P: AsRef<Path>>(keypair: &Keypair, path: P) -> Result<()> {
    let dir = path.as_ref().parent().unwrap_or_else(|| Path::new("./"));
    fs::create_dir_all(dir).map_err(ClientError::IoError)?;
    fs::write(path, keypair.to_bytes()).map_err(ClientError::IoError)
}
