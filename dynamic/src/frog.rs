use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Frog {
    name: [u8; 32],   // Fixed-size array for name
    is_jumping: bool, // Status flag for jumping
}

impl_type_identifiable!(Frog);

impl Frog {
    pub fn new(name: &str, is_jumping: bool) -> Self {
        let mut name_bytes = [0u8; 32];
        let bytes = name.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        Self {
            name: name_bytes,
            is_jumping,
        }
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

impl Executable for Frog {
    // No need to specify TYPE_TAG, it's automatically derived from TypeIdentifiable
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) {
        if self.is_jumping {
            println!("Ribbit! I'm {} and I'm jumping!", self.get_name());
        } else {
            println!(
                "Ribbit! I'm {} and I'm resting on a lily pad.",
                self.get_name()
            );
        }
        // Toggle jumping state each time it executes
        self.is_jumping = !self.is_jumping;
    }
}
