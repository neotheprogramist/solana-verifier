use crate::traits::Executable;

#[repr(C)]
pub struct Mouse {
    name: [u8; 32], // Fixed-size array for name
}

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
    const TYPE_TAG: u8 = 2; // Different from Dog (1) and Cat (0)
    fn execute(&mut self) {
        println!("Squeak! I'm {}.", self.get_name());
    }
}
