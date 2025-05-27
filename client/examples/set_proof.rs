use std::{path::Path, time::Duration};

use client::{initialize_client, setup_payer, setup_program, ClientError, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use stark::swiftness::stark::types::cast_struct_to_slice;
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::AccountCast;
use utils::BidirectionalStack;
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;
fn main() -> client::Result<()> {
    let config = Config::parse_args();
    let client = initialize_client(&config)?;

    let payer = setup_payer(&client, &config)?;

    let program_path = Path::new("target/deploy/verifier.so");

    let program_id = setup_program(&client, &payer, &config, program_path)?;

    println!("Using program ID: {}", program_id);

    let stack_account = Keypair::new();

    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {} bytes", space);

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space)?,
        space as u64,
        &program_id,
    );

    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &stack_account],
        client.get_latest_blockhash()?,
    );

    let signature = client.send_and_confirm_transaction(&create_account_tx)?;
    println!("Account created successfully: {}", signature);
    println!("\nSet Proof on Solana");
    println!("====================");
    let input = include_str!("../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();
    let proof_verifier = proof.transform_to();

    let mut bidirectional_stack = BidirectionalStackAccount {
        proof: proof_verifier,
        front_index: 0,
        back_index: 0,
        buffer: [0; 65536],
    };

    let proof_bytes = cast_struct_to_slice(&mut bidirectional_stack);
    println!("Proof bytes in kb: {:?}", proof_bytes.len() / 1024);
    let instructions = proof_bytes
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetProof(i * CHUNK_SIZE, chunk.to_vec()),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect::<Vec<_>>();

    println!("Instructions number: {:?}", instructions.len());
    std::thread::sleep(Duration::from_secs(10));
    for (i, instruction) in instructions.iter().enumerate() {
        let set_proof_tx = Transaction::new_signed_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash()?,
        );
        let set_proof_signature: solana_sdk::signature::Signature =
            client.send_and_confirm_transaction(&set_proof_tx)?;
        println!("Set proof: {}: {}", i, set_proof_signature);
    }

    let account_data_after_set_proof = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast(&account_data_after_set_proof);
    println!("Proof: {:?}", stack.proof);
    // println!("Stack: {:?}", stack);
    println!("Stack front: {:?}", stack.borrow_front());
    println!("Stack back: {:?}", stack.borrow_back());
    println!("Stack front index: {:?}", stack.front_index);
    println!("Stack back index: {:?}", stack.back_index);
    println!("Stack is empty front: {:?}", stack.is_empty_front());
    println!("Stack is empty back: {:?}", stack.is_empty_back());
    Ok(())
}
