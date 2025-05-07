use utils::AccountCast;
/// Define the type of state stored in accounts
#[derive(Debug)]
pub struct VerifierAccount {
    /// number of greetings
    pub counter: u32,

    pub double_counter: u8,

    pub data: [u8; 1048576],
}

impl AccountCast for VerifierAccount {}
