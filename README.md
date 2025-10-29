Rust Blockchain

A simple interactive blockchain implemented in Rust with proof-of-work.

Features:
- Blocks with index, data, previous hash, hash, and nonce.
- Proof-of-Work (PoW): hashes must start with a configurable number of zeros.
- Interactive terminal menu:
    - Add new blocks
    - Print the blockchain
    - Exit the program
- Simple and lightweight structure, easy to extend.

Folder Structure:

rust_blockchain/
│
├── Cargo.toml           # Cargo configuration (dependencies, package info)
├── Cargo.lock           # Auto-generated lock file
├── src/
│   ├── main.rs          # Entry point, interactive terminal logic
│   ├── blocks/
│   │   ├── block.rs     # Block struct, hashing, proof-of-work
│   │   └── blockchain.rs # Blockchain struct, add block, print chain
└── README.txt           # This documentation

Dependencies:

This project uses the "sha2" crate for hashing.

In Cargo.toml:
[dependencies]
sha2 = "0.10"

Getting Started:

1. Clone or download the repository:

git clone <your-repo-url>
cd rust_blockchain

2. Build and run the project:

cargo run

3. Interact with the blockchain via the terminal menu:

- 1 → Add a new block
- 2 → Print the entire blockchain
- 3 → Exit the program

Example:

--- Rust Blockchain ---
1. Add Block
2. Print Chain
3. Exit
Choose an option: 1
Enter block data: Alice pays Bob 10 coins
Block added!

Notes:

- The difficulty controls the mining speed (number of leading zeros in hash).
- The blockchain is entirely local and for educational purposes.
- Blocks include a nonce to satisfy the proof-of-work condition.
