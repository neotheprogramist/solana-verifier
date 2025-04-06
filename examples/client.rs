use serde::Deserialize;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::{EncodableKey, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_verifier::{Entrypoint, PROGRAM_ID, ProofAccount};
use std::{path::PathBuf, str::FromStr, thread::sleep, time::Duration};
use tokio::fs;

const CHUNK_SIZE: usize = 500;

async fn send_transactions(config: &SolanaConfig, instructions: &[Instruction]) {
    let (client, payer) = config.get_client();

    for (i, instruction) in instructions.iter().enumerate() {
        // Process each instruction one by one
        loop {
            let result = create_and_send_transaction_without_confirmation(
                &client,
                &payer,
                &[instruction.clone()],
            )
            .await;

            if result.is_ok() {
                break;
            }

            println!("Failed to send transaction: {i}, repeating.");
            sleep(Duration::from_millis(100));
        }
    }

    println!(
        "Sent all {} publish instructions sequentially",
        instructions.len()
    );
}

async fn create_and_send_transaction_without_confirmation(
    client: &RpcClient,
    payer: &Keypair,
    instructions: &[Instruction],
) -> Result<Signature, Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash().await?;
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    let signature = client.send_transaction(&transaction).await?;
    Ok(signature)
}

async fn create_and_send_transaction_with_confirmation(
    client: &RpcClient,
    payer: &Keypair,
    instructions: &[Instruction],
) -> Result<Signature, Box<dyn std::error::Error>> {
    let blockhash = client.get_latest_blockhash().await?;
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction).await?;
    Ok(signature)
}

pub async fn read_proof_account() -> Box<ProofAccount> {
    let stark_proof = fs::read("resources/proof.bin").await.unwrap();
    Box::new(*bytemuck::from_bytes::<ProofAccount>(&stark_proof))
}

/// Creates a `Transaction` to create an account with rent exemption
async fn create_proof_data_account(
    client: &RpcClient,
    payer: &Keypair,
    proof_data_account: &Keypair,
    proof_size: usize,
    owner: &Pubkey,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let rent_exemption_amount = client
        .get_minimum_balance_for_rent_exemption(proof_size)
        .await?;

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &proof_data_account.pubkey(),
        rent_exemption_amount,
        proof_size as u64,
        owner,
    );

    let blockhash = client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[payer, proof_data_account],
        blockhash,
    );

    Ok(tx)
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
struct SolanaConfig {
    json_rpc_url: String,
    keypair_path: PathBuf,
}

impl SolanaConfig {
    pub fn get_client(&self) -> (RpcClient, Keypair) {
        let client = RpcClient::new(self.json_rpc_url.clone());
        let payer = Keypair::read_from_file(self.keypair_path.clone()).unwrap();
        (client, payer)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let config =
        PathBuf::from(std::env::var("HOME").unwrap()).join(".config/solana/cli/config.yml");

    let config: SolanaConfig = serde_yaml::from_reader(std::fs::File::open(config)?)?;
    let (client, payer) = config.get_client();

    println!("Using keypair {}, at {}", payer.pubkey(), client.url());

    let account = read_proof_account().await;
    let stark_proof = bytemuck::bytes_of(&*account);

    let needed_tx = get_needed_tx(&stark_proof);
    println!("needed_tx: {}", needed_tx);

    let proof_data_account = Keypair::new();
    let program_id = Pubkey::from_str(PROGRAM_ID)?;

    println!("account pubkey: {:?}", proof_data_account.pubkey());
    client
        .send_and_confirm_transaction(
            &create_proof_data_account(
                &client,
                &payer,
                &proof_data_account,
                stark_proof.len() + 8, // +1 for the `stage` and 7 as padding
                &program_id,
            )
            .await?,
        )
        .await?;

    // for (section, section_data) in stark_proof.chunks(10000).enumerate() {
    // Allocate data instructions

    let instructions = stark_proof
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, data)| Instruction {
            program_id,
            accounts: vec![AccountMeta::new(proof_data_account.pubkey(), false)],
            data: bincode::serialize(&Entrypoint::PublishFragment {
                offset: i * CHUNK_SIZE,
                data,
            })
            .unwrap(),
        })
        .collect::<Vec<_>>();

    send_transactions(&config, &instructions).await;

    println!("Prepared instructions");

    loop {
        let data = client
            .get_account_data(&proof_data_account.pubkey())
            .await?;

        if data[8..].eq(stark_proof) {
            println!("proof_data_account correct!");
            break;
        } else {
            println!("waiting proof_data_account data to match!");
            sleep(Duration::from_secs(1));
        }
    }

    let schedule_ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(proof_data_account.pubkey(), false)],
        data: bincode::serialize(&Entrypoint::Schedule).unwrap(),
    };

    let verify_ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(proof_data_account.pubkey(), false)],
        data: bincode::serialize(&Entrypoint::VerifyProof).unwrap(),
    };

    let mut verify_ixs = (0..needed_tx)
        .map(|_| verify_ix.clone())
        .collect::<Vec<_>>();
    verify_ixs.insert(0, schedule_ix);

    let max_instructions_per_tx = 1;
    let mut batches: Vec<Vec<Instruction>> = Vec::new();

    let mut current_batch: Vec<Instruction> = Vec::new();

    for ix in verify_ixs {
        current_batch.push(ix);

        // Create a new batch when we reach the limit
        if current_batch.len() >= max_instructions_per_tx {
            batches.push(current_batch);
            current_batch = Vec::new();
        }
    }

    // Add any remaining instructions to the last batch
    if !current_batch.is_empty() {
        batches.push(current_batch);
    }

    println!("Split into {} transactions", batches.len());

    // Send each batch as a separate transaction
    for (i, batch) in batches.iter().enumerate() {
        println!(
            "Sending transaction {}/{} with {} instructions",
            i + 1,
            batches.len(),
            batch.len()
        );

        match create_and_send_transaction_with_confirmation(&client, &payer, batch).await {
            Ok(signature) => println!(
                "Transaction {}/{} confirmed, signature: {}",
                i + 1,
                batches.len(),
                signature
            ),
            Err(err) => {
                println!("Transaction {}/{} failed: {}", i + 1, batches.len(), err);
                return Err(format!("Transaction failed: {}", err).into());
            }
        }
    }

    Ok(())
}

fn get_needed_tx(proof: &[u8]) -> usize {
    let mut proof = proof.to_vec();
    let proof_account = bytemuck::from_bytes_mut::<ProofAccount>(&mut proof);
    proof_account.flow()
}
