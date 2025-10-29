
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String, difficulty: usize) -> Self {
        let mut nonce = 0;
        let mut hash = Block::calculate_hash(index, &data, &previous_hash, nonce);

        while &hash[0..difficulty] != &"0".repeat(difficulty) {
            nonce += 1;
            hash = Block::calculate_hash(index, &data, &previous_hash, nonce);
        }

        Block { index, data, previous_hash, hash, nonce }
    }

    pub fn calculate_hash(index: u32, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        hasher.update(nonce.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
