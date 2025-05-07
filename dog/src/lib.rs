use utils::Executable;

#[repr(C)]
pub struct Dog {
    name: [u8; 32], // Fixed-size array for name
}

impl Dog {
    pub fn new(name: &str) -> Self {
        let mut name_bytes = [0u8; 32];
        let bytes = name.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        Self { name: name_bytes }
    }

    pub fn get_name(&self) -> String {
        let null_pos = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        String::from_utf8_lossy(&self.name[..null_pos]).to_string()
    }
}

impl Executable for Dog {
    const TYPE_TAG: u8 = 1;
    fn execute(&mut self) {
        println!("Woof! I'm {}.", self.get_name());
    }
}
