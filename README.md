# Solana Verifier Example

This project demonstrates how to build, deploy, and interact with a Solana program using Rust.

## Project Structure

- `verifier/`: The Solana program written in Rust
- `client/`: A Rust client application that deploys and interacts with the program


## Manual Setup

1. Start a Solana test validator:
```bash
solana-test-validator
```

2. Build the Solana program:
```bash
cargo build-sbf -- -p verifier
```

3. Build and run the client:
```bash
cargo run -p client
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
