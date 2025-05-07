use utils::Executable;

#[repr(C)]
pub struct Bird {
    species: [u8; 32], // Fixed-size array for species
    is_flying: bool,   // Status flag
}

impl Bird {
    pub fn new(species: &str, is_flying: bool) -> Self {
        let mut species_bytes = [0u8; 32];
        let bytes = species.as_bytes();
        let len = std::cmp::min(bytes.len(), 32);
        species_bytes[..len].copy_from_slice(&bytes[..len]);
        Self {
            species: species_bytes,
            is_flying,
        }
    }

    fn get_species(&self) -> String {
        let null_pos = self
            .species
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.species.len());
        String::from_utf8_lossy(&self.species[..null_pos]).to_string()
    }
}

impl Executable for Bird {
    const TYPE_TAG: u8 = 3; // Different from Dog (1), Cat (0), and Mouse (2)
    fn execute(&mut self) {
        if self.is_flying {
            println!("Tweet! I'm a {} flying high!", self.get_species());
        } else {
            println!("Tweet! I'm a {} resting on a branch.", self.get_species());
        }
        // Toggle flying state each time it executes
        self.is_flying = !self.is_flying;
    }
}
