use serde::{Deserialize, Serialize};

use crate::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(
            0,
            1753948889,
            String::from("Hello World!"),
            String::from(""),
            0,
        );
        Blockchain {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, timestamp: i64, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let block = Block::mine_block(
            prev_block.index + 1,
            timestamp,
            data,
            prev_block.hash.clone(),
        );
        self.blocks.push(block);
    }

    pub fn validate(&self) -> bool {
        let mut right = 1;
        let mut left = 0;
        while right != self.blocks.len() - 1 {
            if self.blocks[right].previous_hash != self.blocks[left].hash {
                return false;
            }
            right += 1;
            left += 1;
        }
        true
    }
}
