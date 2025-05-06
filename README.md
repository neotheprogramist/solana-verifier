# Solana Verifier Example

This project demonstrates how to build, deploy, and interact with a Solana program using Rust.

## Project Structure

- `verifier/`: The Solana program written in Rust
- `client/`: A Rust client application that deploys and interacts with the program
- `scheduler/`: A task scheduler library for serialization and execution of tasks
- `tasks/`: Task implementations for the scheduler


## Manual Setup

1. Start a Solana test validator:
```bash
solana-test-validator
```

2. Build the Solana program:
```bash
cargo build-sbf -- -p verifier
```

3. Build and run the greeting example:
```bash
cargo run --example greeting
```

4. Build and run the scheduler example:
```bash
cargo run --example scheduler
```

## Client Features

The client demonstrates how to:
- Create and manage Solana keypairs
- Request airdrops of SOL for testing
- Deploy a Solana program programmatically using the Solana SDK
- Create program accounts
- Send transactions to interact with the program
- Read account data from the blockchain
- Use the scheduler to execute tasks on-chain

## Scheduler Integration

The verifier program now integrates with a task scheduler that can:
- Schedule and execute arithmetic tasks
- Store task results in a persistent account
- Demonstrate how to use the scheduler pattern in Solana programs

The scheduler example shows how to:
- Create a scheduler account
- Schedule an Add task with two operands
- Execute the task on-chain
- Get the result of the computation

## Programmatic Deployment

This example showcases how to deploy a Solana program using the Solana SDK directly in Rust code, without relying on the Solana CLI. This approach:

- Gives you more control over the deployment process
- Allows you to integrate program deployment into your application workflow
- Demonstrates how to work with the BPF loader at a lower level

See the [client README](client/README.md) for more details.
