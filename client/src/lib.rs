pub mod account;
pub mod client;
pub mod config;
pub mod deployment;
pub mod error;
pub mod greeting;
pub mod scheduler;
pub mod transaction;
pub mod utils;

// Re-export the main types and functions for backward compatibility
pub use config::Config;
pub use error::{ClientError, Result};

// Client module exports
pub use client::initialize_client;

// Account module exports
pub use account::{
    read_keypair_file, request_and_confirm_airdrop, setup_payer, write_keypair_file,
};

// Transaction module exports
pub use transaction::{
    confirm_transaction_with_retries, create_and_send_transaction, send_and_confirm_transaction,
};

// Deployment module exports
pub use deployment::{deploy_program, setup_program, write_program_to_buffer};

// Greeting module exports
pub use greeting::{interact_with_program, setup_greeting_account};

// Scheduler module exports
pub use scheduler::{
    execute_all_tasks, execute_task, initialize_scheduler, push_task, setup_scheduler_account,
};
