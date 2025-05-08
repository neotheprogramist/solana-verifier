use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Bird {
    species: [u8; 32], // Fixed-size array for species
    is_flying: bool,   // Status flag
}

impl_type_identifiable!(Bird);

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
    // No need to specify TYPE_TAG, it's automatically derived from TypeIdentifiable
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) {
        if self.is_flying {
            println!("Tweet! I'm a {} flying high!", self.get_species());
        } else {
            println!("Tweet! I'm a {} resting on a branch.", self.get_species());
        }
        // Toggle flying state each time it executes
        self.is_flying = !self.is_flying;
    }
}
