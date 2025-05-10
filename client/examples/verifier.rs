use arithmetic::add::Add;
use client::{initialize_client, setup_payer, ClientError, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::{mem::size_of, str::FromStr};
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

// Helper function to print byte array
fn print_bytes(bytes: &[u8]) {
    print!("[");
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", byte);
    }
    println!("]");
}

/// Main entry point for the Solana program client
fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config)?;

    // Setup the payer account
    let payer = setup_payer(&client, &config)?;

    // Get program ID from environment variable or use the default
    let program_id = if let Ok(id_str) = std::env::var("PROGRAM_ID") {
        Pubkey::from_str(&id_str).unwrap_or_else(|_| {
            // Default program ID from the deployed program
            Pubkey::from_str("F2G4q7fGoPAagN59euVMYSCttUTDrST85wck6Rk6CDd6").unwrap()
        })
    } else {
        // Default program ID from the deployed program
        Pubkey::from_str("F2G4q7fGoPAagN59euVMYSCttUTDrST85wck6Rk6CDd6").unwrap()
    };

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

    println!("\nDynamic Arithmetic Operations on Solana");
    println!("======================================");

    // Print information about the Add operation
    println!("Using Add operation with TYPE_TAG: {}", Add::TYPE_TAG);

    // Create Add task (same as in arithmetic example)
    let add_task = Add::new(48, 52);
    let add_data = add_task.to_vec_with_type_tag();
    println!("Add task data size: {} bytes", add_data.len());

    // Print the task data in a readable format
    println!("Task data:");
    print_bytes(&add_data);

    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(add_data),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let push_tx = Transaction::new_signed_with_payer(
        &[push_task_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash()?,
    );

    let push_signature = client.send_and_confirm_transaction(&push_tx)?;
    println!("\nTask pushed: {}", push_signature);

    // Check stack state after pushing
    let account_data_after_push = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let stack_after_push = BidirectionalStackAccount::cast(&account_data_after_push);
    println!("Stack front index: {}", stack_after_push.front_index);
    println!("Stack back index: {}", stack_after_push.back_index);

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

    let execute_signature = client.send_and_confirm_transaction(&execute_tx)?;
    println!("\nTask executed: {}", execute_signature);

    // Check final stack state
    let final_account_data = client
        .get_account_data(&stack_account.pubkey())
        .map_err(ClientError::SolanaClientError)?;
    let final_stack = BidirectionalStackAccount::cast(&final_account_data);
    println!("Stack front index: {}", final_stack.front_index);
    println!("Stack back index: {}", final_stack.back_index);

    // Read and display the result
    let result_bytes = final_stack.borrow_front();
    println!("Result bytes (length: {}): ", result_bytes.len());
    print_bytes(result_bytes);

    if result_bytes.len() >= 16 {
        let mut result_array = [0u8; 16];
        result_array.copy_from_slice(&result_bytes[0..16]);
        let result = u128::from_be_bytes(result_array);

        println!("\nAdd result (48 + 52): {}", result);
        if result == 100 {
            println!("Success! Computation matches expected result.");
        } else {
            println!("ERROR: Result does not match expected value of 100!");
        }
    } else {
        println!(
            "\nERROR: Result data is too short: {} bytes",
            result_bytes.len()
        );
    }

    println!("\nArithmetic operation successfully executed on Solana!");

    Ok(())
}
