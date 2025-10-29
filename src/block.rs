// Import the Sha256 hashing function and the Digest trait from the sha2 crate
use sha2::{Sha256, Digest};

// Derive the Debug trait so that instances of Block can be printed with {:?}
#[derive(Debug)]
// Define the Block struct to represent a single block in the blockchain
pub struct Block {
    pub index: u32,           // The position of the block in the blockchain
    pub data: String,         // The arbitrary data stored in the block
    pub previous_hash: String,// The hash of the previous block in the chain
    pub hash: String,         // The hash of this block (current block)
    pub nonce: u64,           // The nonce used for proof-of-work mining
}

// Implement methods for the Block struct
impl Block {
    // Constructor to create a new block
    pub fn new(index: u32, data: String, previous_hash: String, difficulty: usize) -> Self {
        let mut nonce = 0; // Start nonce at 0 for proof-of-work
        // Calculate the initial hash of the block with the current nonce
        let mut hash = Block::calculate_hash(index, &data, &previous_hash, nonce);

        // Proof-of-work loop: keep changing nonce until hash starts with 'difficulty' zeros
        while &hash[0..difficulty] != &"0".repeat(difficulty) {
            nonce += 1; // Increment nonce for next attempt
            hash = Block::calculate_hash(index, &data, &previous_hash, nonce); // Recalculate hash
        }

        // Return a new Block instance with all fields populated
        Block { index, data, previous_hash, hash, nonce }
    }

    // Function to calculate the hash of a block
    pub fn calculate_hash(index: u32, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new(); // Create a new Sha256 hasher
        hasher.update(index.to_string()); // Include index in the hash
        hasher.update(data);             // Include data in the hash
        hasher.update(previous_hash);    // Include previous hash in the hash
        hasher.update(nonce.to_string());// Include nonce in the hash
        let result = hasher.finalize();  // Finalize hashing and get byte array
        format!("{:x}", result)          // Convert hash bytes to hexadecimal string
    }
}
