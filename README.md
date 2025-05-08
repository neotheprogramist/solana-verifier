# Solana Scheduler Example

This project demonstrates how to build, deploy, and interact with a Solana program using Rust.

## Project Structure

- `client/`: A Rust client application that deploys and interacts with the program
- `programs/`: Solana programs written in Rust
- `tasks/`: Task implementations for the scheduler
- `scheduler/`: Core scheduler implementation for task execution
- `dynamic/`: Dynamic type execution system for handling multiple task types

## Manual Setup

1. Start a Solana test validator:
```bash
solana-test-validator
```

2. Build the Solana program:
```bash
cargo build-sbf --workspace
```

3. Build and run the greeting example:
```bash
cargo run --example greeting
```

## Client Features

The client demonstrates how to:
- Create and manage Solana keypairs
- Request airdrops of SOL for testing
- Deploy a Solana program programmatically using the Solana SDK
- Create program accounts
- Send transactions to interact with the program
- Read account data from the blockchain

## Programmatic Deployment

This example showcases how to deploy a Solana program using the Solana SDK directly in Rust code, without relying on the Solana CLI. This approach:

- Gives you more control over the deployment process
- Allows you to integrate program deployment into your application workflow
- Demonstrates how to work with the BPF loader at a lower level

See the [client README](client/README.md) for more details.

## Project Internals

### Verifier Program
The core Solana program (`programs/verifier/`) implements a verification system that can execute and validate different types of tasks on-chain. It includes:
- An instruction processor for handling program commands
- State management for task data and execution results
- Error handling specific to verification operations

### Scheduler System
The scheduler (`scheduler/`) provides a framework for defining, executing, and managing tasks:
- Tasks implement the `SchedulerTask` trait
- The scheduler maintains data and execution stacks
- Tasks can be executed sequentially with results passed between them

### Task Implementations
The project includes several task types in the `tasks/` directory:
- `arithmetic/`: Basic arithmetic operations (addition, exponentiation, Fibonacci)
- `example/`: Example task implementations for demonstration
- `stark/`: STARK (Scalable Transparent ARguments of Knowledge) verification tasks

### Dynamic Type System
The `dynamic/` module provides a system for handling multiple types that implement the `Executable` trait:
- Automatically detects types implementing the trait
- Generates code for type-based dispatch
- Allows for serialization and execution based on type tags

### Client Examples
The `client/examples/` directory contains example applications that demonstrate how to:
- Deploy and interact with the verifier program
- Execute different types of tasks
- Work with the dynamic type system
- Handle program accounts and transaction signing
