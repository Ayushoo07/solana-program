# Solana To-Do List Program

This is a simple To-Do List program built using the **Anchor framework** on Solana. It allows users to:
- Initialize a To-Do account
- Create new To-Do items
- Update task descriptions
- Mark tasks as completed or incomplete
- Delete tasks

## Prerequisites

Ensure you have the following installed before proceeding:
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- Rust and Cargo ([Install Rust](https://www.rust-lang.org/tools/install))
- Node.js and Yarn ([Install Node.js](https://nodejs.org/en/download/))

## Build and Deploy on Solana Devnet

### 1. Configure Solana CLI

Set the network to **Devnet**:
```sh
solana config set --url devnet
```

Create a new keypair (if not already created):
```sh
solana-keygen new --outfile ~/.config/solana/id.json
```

Airdrop some SOL to your account for transaction fees:
```sh
solana airdrop 2
```

### 2. Build the Program

Compile the program using Anchor:
```sh
anchor build
```

### 3. Deploy to Devnet

```sh
anchor deploy
```

After successful deployment, note the **program ID** displayed in the output. If needed, update `declare_id!()` in your `lib.rs` file with the correct program ID.

## Running Tests

### Local Testing

To run tests locally using Solana's built-in test validator:
```sh
anchor test
```

### On-Chain Testing (Devnet)

1. Start a local Solana validator:
   ```sh
   solana-test-validator
   ```
2. Deploy the program to Devnet:
   ```sh
   anchor deploy
   ```
3. Run the tests:
   ```sh
   anchor test --provider.cluster devnet
   ```

## Program Structure

- **programs/todo_list/src/lib.rs**: Contains the main program logic.
- **tests/**: Contains integration tests.
- **migrations/**: Stores migration scripts.
- **Anchor.toml**: Anchor configuration file.

## Troubleshooting

- If you encounter **`AccountNotFound`** errors, make sure the **program ID** is correctly updated in `declare_id!()`.
- Use `solana logs` to check for runtime errors:
  ```sh
  solana logs
  ```
- Ensure you have enough **SOL** for transactions:
  ```sh
  solana balance# Solana To-Do List Program

This is a simple To-Do List program built using the **Anchor framework** on Solana. It allows users to:
- Initialize a To-Do account
- Create new To-Do items
- Update task descriptions
- Mark tasks as completed or incomplete
- Delete tasks

## Prerequisites

Ensure you have the following installed before proceeding:
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- Rust and Cargo ([Install Rust](https://www.rust-lang.org/tools/install))
- Node.js and Yarn ([Install Node.js](https://nodejs.org/en/download/))

## Build and Deploy on Solana Devnet

### 1. Configure Solana CLI

Set the network to **Devnet**:
```sh
solana config set --url devnet
```

Create a new keypair (if not already created):
```sh
solana-keygen new --outfile ~/.config/solana/id.json
```

Airdrop some SOL to your account for transaction fees:
```sh
solana airdrop 2
```

### 2. Build the Program

Compile the program using Anchor:
```sh
anchor build
```

### 3. Deploy to Devnet

```sh
anchor deploy
```

After successful deployment, note the **program ID** displayed in the output. If needed, update `declare_id!()` in your `lib.rs` file with the correct program ID.

## Running Tests

### Local Testing

To run tests locally using Solana's built-in test validator:
```sh
anchor test
```

### On-Chain Testing (Devnet)

1. Start a local Solana validator:
   ```sh
   solana-test-validator
   ```
2. Deploy the program to Devnet:
   ```sh
   anchor deploy
   ```
3. Run the tests:
   ```sh
   anchor test --provider.cluster devnet
   ```

## Program Structure

- **programs/todo_list/src/lib.rs**: Contains the main program logic.
- **tests/**: Contains integration tests.
- **migrations/**: Stores migration scripts.
- **Anchor.toml**: Anchor configuration file.

## Troubleshooting

- If you encounter **`AccountNotFound`** errors, make sure the **program ID** is correctly updated in `declare_id!()`.
- Use `solana logs` to check for
- runtime errors:
  ```sh
  solana logs
  ```
- Ensure you have enough **SOL** for transactions:
  ```sh
  solana balance
  ```



