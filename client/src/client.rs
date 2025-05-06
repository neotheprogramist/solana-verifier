use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::{ClientError, Config, Result};

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
