use utils::impl_type_identifiable;
use utils::{Executable, TypeIdentifiable};

#[repr(C)]
pub struct Op {
    x: u32,
    y: u32,
}

impl_type_identifiable!(Op);

impl Op {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn compute(&self) -> u32 {
        self.x.saturating_add(self.y)
    }
}
