
use crate::blocks::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        Blockchain { chain: vec![genesis_block], difficulty }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
