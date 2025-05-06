/// Define the type of state stored in accounts
#[derive(Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,

    pub double_counter: u8,

    pub data: [u8; 1048576],
}
impl GreetingAccount {
    pub fn cast_mut(slice: &mut [u8]) -> &mut Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }
}
