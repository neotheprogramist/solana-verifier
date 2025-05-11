# Solana Verifier

This project demonstrates how to build, deploy, and interact with Solana programs using Rust, with a focus on on-chain verification of various computational tasks.

## Project Structure

- `client/`: A Rust client application that deploys and interacts with programs
- `programs/`: Solana programs written in Rust
  - `greeting/`: Simple greeting program for demonstration
  - `verifier/`: Core verification program for executing tasks
  - `utils/`: Shared utilities for Solana programs
- `tasks/`: Task implementations for the verifier
  - `arithmetic/`: Basic arithmetic operations
  - `stark/`: STARK verification tasks

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
- Deploy Solana programs programmatically using the Solana SDK
- Create program accounts
- Send transactions to interact with the program
- Read account data from the blockchain

## Programmatic Deployment

This example showcases how to deploy a Solana program using the Solana SDK directly in Rust code, without relying on the Solana CLI. This approach:

- Gives you more control over the deployment process
- Allows you to integrate program deployment into your application workflow
- Demonstrates how to work with the BPF loader at a lower level

## Project Components

### Verifier Program
The core Solana program (`programs/verifier/`) implements a verification system that can execute and validate different types of tasks on-chain. It includes:
- An instruction processor for handling program commands
- A scheduler system for task execution
- State management for task data and execution results
- Error handling specific to verification operations

### Task Implementations
The project includes several task types in the `tasks/` directory:
- `arithmetic/`: Basic arithmetic operations (addition, multiplication, exponentiation, Fibonacci)
- `stark/`: STARK (Scalable Transparent ARguments of Knowledge) verification tasks

### Client Examples
The `client/examples/` directory contains example applications that demonstrate how to:
- Deploy and interact with the programs (greeting, verifier)
- Execute different types of tasks:
  - Simple arithmetic (add, mul, exp, fib)
  - Cryptographic operations (poseidon, hades)
  - Type casting and conversion
