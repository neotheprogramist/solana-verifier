use client::{initialize_client, setup_payer, setup_program, ClientError, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use stark::felt::Felt;
use stark::poseidon::hades::HadesPermutation;
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

/// Main entry point for the Solana program client
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Define program path
    let program_path = Path::new("target/deploy/verifier.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path)?;

    println!("Using program ID: {}", program_id);

    // Create a new account that's owned by our program
    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    // Calculate the space needed for our account
    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {} bytes", space);

    // Create account instruction
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space)?,
        space as u64,
        &program_id,
    );

    // Create and send the transaction
    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &stack_account],
        client.get_latest_blockhash()?,
    );

    let signature = client.send_and_confirm_transaction(&create_account_tx)?;
    println!("Account created successfully: {}", signature);

    // Initialize the account
    let init_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::Initialize,
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    // Send initialize transaction
    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash()?,
    );

    let init_signature = client.send_and_confirm_transaction(&init_tx)?;
    println!("Account initialized: {}", init_signature);

    // Cast to stack account to see if initialized correctly
    let account_data_after_init = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast(&account_data_after_init);
    println!("Stack front_index: {}", stack.front_index);
    println!("Stack back_index: {}", stack.back_index);

    println!("\nHades Permutation on Solana");
    println!("=========================");

    // Print information about the Hades operation
    println!(
        "Using HadesPermutation with TYPE_TAG: {}",
        HadesPermutation::TYPE_TAG
    );

    // Create initial state for Hades permutation
    let state = [
        Felt::from_hex("0x9").unwrap(),
        Felt::from_hex("0xb").unwrap(),
        Felt::from_hex("0x2").unwrap(),
    ];

    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(HadesPermutation::new(state).to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let push_tx = Transaction::new_signed_with_payer(
        &[push_task_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash()?,
    );

    let push_signature = client.send_and_confirm_transaction(&push_tx)?;
    println!("\nHades task pushed: {}", push_signature);

    // Check stack state after pushing
    let account_data_after_push = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack_after_push = BidirectionalStackAccount::cast(&account_data_after_push);
    println!("Stack front index: {}", stack_after_push.front_index);
    println!("Stack back index: {}", stack_after_push.back_index);

    // Execute until task is complete
    let mut steps = 0;
    loop {
        // Execute the task
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
        print!(".");
        steps += 1;
        if steps % 10 == 0 {
            println!(" {}", steps);
        }

        // Check stack state
        let account_data = client
            .get_account_data(&stack_account.pubkey())
            .map_err(ClientError::SolanaClientError)?;
        let stack = BidirectionalStackAccount::cast(&account_data);
        if stack.is_empty_back() {
            println!("\nExecution complete after {} steps", steps);
            break;
        }
    }

    // Read and display the result
    let account_data = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast(&account_data);
    let result_bytes = stack.borrow_front();
    let felt = Felt::from_bytes_be(&result_bytes.try_into().unwrap());
    println!("\nHades permutation result: {}", felt);

    println!("\nHades permutation successfully executed on Solana!");

    Ok(())
}
