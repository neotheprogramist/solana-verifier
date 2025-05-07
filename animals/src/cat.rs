use utils::impl_type_identifiable;
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Cat {
    color: [u8; 32], // Fixed-size array for color
}

impl_type_identifiable!(Cat);

impl Cat {
    pub fn new(color: &str) -> Self {
        let mut color_bytes = [0u8; 32];
        let bytes = color.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        color_bytes[..len].copy_from_slice(&bytes[..len]);
        Self { color: color_bytes }
    }

    fn get_color(&self) -> String {
        let null_pos = self
            .color
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.color.len());
        String::from_utf8_lossy(&self.color[..null_pos]).to_string()
    }
}

impl Executable for Cat {
    fn execute(&mut self) {
        println!("Meow! I am {}.", self.get_color());
    }
}
