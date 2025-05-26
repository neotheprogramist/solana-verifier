use std::path::Path;

use borsh::BorshSerialize;
use client::{initialize_client, setup_payer, setup_program, ClientError, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use stark::{
    felt::Felt,
    funvec::FunVec,
    stark_proof::{HashPublicInputs, VerifyPublicInput},
    swiftness::{
        air::public_memory::{Page, PublicInput},
        stark::{
            config::StarkConfig,
            types::{cast_struct_to_slice, StarkProof},
        },
    },
};
use utils::BidirectionalStack;
use utils::{AccountCast, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};
pub const CHUNK_SIZE: usize = 500;
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

    let init_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::Initialize,
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );
    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash()?,
    );
    let init_signature = client.send_and_confirm_transaction(&init_tx)?;
    println!("Account initialized: {}", init_signature);

    let account_data_after_init = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast(&account_data_after_init);

    println!("\nSet Proof on Solana");
    println!("====================");

    let mut proof = StarkProof {
        public_input: PublicInput {
            log_n_steps: Felt::from(1),
            range_check_min: Felt::from(2),
            range_check_max: Felt::from(3),
            layout: Felt::from(4),
            segments: FunVec::default(),
            padding_addr: Felt::from(5),
            padding_value: Felt::from(6),
            main_page: Page::default(),
            continuous_page_headers: FunVec::default(),
        },
        config: StarkConfig::default(),
    };
    let proof_bytes = cast_struct_to_slice(&mut proof);
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
    for instruction in instructions {
        let set_proof_tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash()?,
        );
        let set_proof_signature = client.send_and_confirm_transaction(&set_proof_tx)?;
        println!("Set proof: {}", set_proof_signature);
    }

    let account_data_after_set_proof = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast(&account_data_after_set_proof);

    let verify_public_input_task = VerifyPublicInput::new().to_vec_with_type_tag();
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(verify_public_input_task),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );
    let push_tx = Transaction::new_signed_with_payer(
        &[push_task_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash()?,
    );
    let push_signature = client.send_and_confirm_transaction(&push_tx)?;
    println!("Push task: {}", push_signature);

    let mut steps = 0;
    loop {
        let execute_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::Execute,
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let execute_tx = Transaction::new_signed_with_payer(
            &[execute_ix],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash()?,
        );
        let _execute_signature = client.send_and_confirm_transaction(&execute_tx)?;
        println!(".");
        steps += 1;
        if stack.is_empty_back() {
            println!("\nExecution complete after {} steps", steps);
            break;
        }
    }

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("\nProgram Hash: {:?}", result_program_hash);
    println!("Output Hash: {:?}", result_output_hash);
    println!("\n Verify Public Inputs successfully executed on Solana!");
    Ok(())
}
