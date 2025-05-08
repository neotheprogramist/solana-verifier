# Dynamic Type Execution System

This project implements a system for handling multiple types that implement the `Executable` trait, allowing for serialization and execution based on type tags.

## How It Works

1. The `build.rs` file automatically scans the source directory for types that implement the `Executable` trait.
2. It generates code for a generic `execute` function that can handle any type implementing `Executable`.
3. The generated code is included in `main.rs` using Rust's `include!` macro.

## Adding New Types

To add a new type that can be executed:

1. Create a new file in the `src` directory for your type (e.g., `src/mouse.rs`).
2. Define your struct and implement the `Executable` trait:

```rust
use crate::traits::Executable;

#[repr(C)]
pub struct Mouse {
    name: [u8; 32],
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
    const TYPE_TAG: u8 = 2; // Use a unique type tag
    fn execute(&mut self) {
        println!("Squeak! I'm {}.", self.get_name());
    }
}
```

3. Add the module to `main.rs`:

```rust
pub mod mouse;
```

4. Import your type in `main.rs`:

```rust
use mouse::Mouse;
```

5. That's it! The `build.rs` script will automatically detect your new type and update the generated code.

## Important Notes

- Each type must have a unique `TYPE_TAG` value.
- Structs must be `#[repr(C)]` to ensure consistent memory layout for serialization.
- The `build.rs` script runs before compilation, so new types are automatically recognized without changing any execution code.

## Usage Example

```rust
// Create instances of executable types
let mouse = Mouse::new("Jerry");

// Add to the stack using the generic helper
push_executable(&mut stack, mouse);

// Execute (will dispatch to the correct type's execute method)
execute(&mut stack);
```

## How the System Works

1. The `build.rs` script scans all `.rs` files in the project.
2. It identifies structs that implement the `Executable` trait.
3. It generates a dispatch function that handles the execution based on the type tag.
4. It also generates a helper function for pushing executables to the stack.
5. This means that the main execution code never needs to change, even as new types are added. 