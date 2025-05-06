#![allow(deprecated)]

// This file contains general utility functions that don't fit in other modules.
// Most functionality has been moved to more specific modules.

// Re-export common types for backward compatibility
pub use crate::account::{
    read_keypair_file, request_and_confirm_airdrop, setup_payer, write_keypair_file,
};
pub use crate::client::initialize_client;
pub use crate::deployment::{deploy_program, setup_program, write_program_to_buffer};
pub use crate::greeting::{interact_with_program, setup_greeting_account};
pub use crate::scheduler::{
    execute_all_tasks, execute_task, initialize_scheduler, push_task, setup_scheduler_account,
};
pub use crate::transaction::{
    confirm_transaction_with_retries, create_and_send_transaction, send_and_confirm_transaction,
};
