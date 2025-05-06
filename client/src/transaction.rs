use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::thread::sleep;

use crate::{ClientError, Config, Result};

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

/// Send and confirm a transaction
pub fn send_and_confirm_transaction(
    client: &RpcClient,
    transaction: &Transaction,
    retries: usize,
    config: &Config,
) -> Result<solana_sdk::signature::Signature> {
    let signature = client.send_transaction(transaction).map_err(|err| {
        ClientError::TransactionError(format!("Failed to send transaction: {}", err))
    })?;

    confirm_transaction_with_retries(client, &signature, retries, config)?;

    Ok(signature)
}

/// Create and send a transaction with the given instruction
pub fn create_and_send_transaction(
    client: &RpcClient,
    instruction: Instruction,
    signers: &[&Keypair],
    config: &Config,
) -> Result<solana_sdk::signature::Signature> {
    let recent_blockhash = client.get_latest_blockhash().map_err(|err| {
        ClientError::TransactionError(format!("Failed to get recent blockhash: {}", err))
    })?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signers[0].pubkey()),
        signers,
        recent_blockhash,
    );

    send_and_confirm_transaction(client, &transaction, config.transaction_retry_count, config)
}
