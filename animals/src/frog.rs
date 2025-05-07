use utils::Executable;

#[repr(C)]
pub struct Frog {
    name: [u8; 32],   // Fixed-size array for name
    is_jumping: bool, // Status flag for jumping
}

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
    const TYPE_TAG: u8 = 4; // Different from Dog (1), Cat (0), Mouse (2), and Bird (3)
    fn execute(&mut self) {
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
