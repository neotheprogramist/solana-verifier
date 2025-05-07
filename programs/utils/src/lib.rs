/// Trait for safely casting between account data and Rust types
pub trait AccountCast: Sized {
    /// Cast a slice to an immutable reference of Self
    fn cast(slice: &[u8]) -> &Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    /// Cast a mutable slice to a mutable reference of Self
    fn cast_mut(slice: &mut [u8]) -> &mut Self {
        assert_eq!(slice.len(), std::mem::size_of::<Self>());
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }
}

pub trait BidirectionalStack {
    type Error;

    fn push_front(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn push_back(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn pop_front(&mut self) -> Result<(), Self::Error>;
    fn pop_back(&mut self) -> Result<(), Self::Error>;
    fn borrow_front(&self) -> Result<&[u8], Self::Error>;
    fn borrow_back(&self) -> Result<&[u8], Self::Error>;
    fn borrow_mut_front(&mut self) -> Result<&mut [u8], Self::Error>;
    fn borrow_mut_back(&mut self) -> Result<&mut [u8], Self::Error>;
}
