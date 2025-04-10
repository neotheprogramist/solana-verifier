#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

macro_rules! import_module_if_layout {
    ($mod_name:ident) => {
        #[cfg(any(
            feature = "recursive_with_poseidon",
        ))]
        pub mod $mod_name;
    };
}

import_module_if_layout!(commit);
import_module_if_layout!(oods);
import_module_if_layout!(stark);
import_module_if_layout!(verify);

pub mod config;
pub mod queries;
pub mod types;

#[cfg(any(test, feature = "test_fixtures"))]
pub mod fixtures;
#[cfg(test)]
pub mod tests;

pub use funvec;
