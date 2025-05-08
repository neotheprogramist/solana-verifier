use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Mouse {
    name: [u8; 32], // Fixed-size array for name
}

impl_type_identifiable!(Mouse);

impl Mouse {
    pub fn new(name: &str) -> Self {
        let mut name_bytes = [0u8; 32];
        let bytes = name.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        Self { name: name_bytes }
    }

    fn get_name(&self) -> String {
        let null_pos = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        String::from_utf8_lossy(&self.name[..null_pos]).to_string()
    }
}

impl Executable for Mouse {
    // No need to specify TYPE_TAG, it's automatically derived from TypeIdentifiable
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        println!("Squeak! I'm {}.", self.get_name());
        Vec::new()
    }
}
