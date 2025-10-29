// Import the Block struct from the block module
use crate::blocks::block::Block;

// Define the Blockchain struct
pub struct Blockchain {
    pub chain: Vec<Block>,   // A vector storing the sequence of blocks
    pub difficulty: usize,   // Difficulty for proof-of-work (number of leading zeros)
}

// Implement methods for Blockchain
impl Blockchain {
    // Constructor to create a new Blockchain with a genesis block
    pub fn new(difficulty: usize) -> Self {
        // Create the first block (genesis block) with index 0, data "Genesis Block" and previous hash "0"
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        // Initialize the blockchain with the genesis block and store difficulty
        Blockchain { chain: vec![genesis_block], difficulty }
    }

    // Add a new block to the blockchain
    pub fn add_block(&mut self, data: String) {
        // Get the last block in the chain to link the new block
        let last_block = self.chain.last().unwrap(); // unwrap() is safe here because chain always has at least genesis block
        // Create a new block with incremented index, provided data, previous hash from last block, and difficulty
        let new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(), // clone because hash is a String
            self.difficulty,
        );
        // Append the new block to the blockchain
        self.chain.push(new_block);
    }

    // Print all blocks in the blockchain
    pub fn print_chain(&self) {
        // Iterate over each block in the chain
        for block in &self.chain {
            println!("{:?}", block); // Print block using Debug formatting
        }
    }
}
