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
