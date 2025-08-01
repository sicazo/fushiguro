use crate::{
    block::Block,
    transaction::{self, Transaction},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub balances: HashMap<String, usize>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, 1753948889, Vec::new(), String::from("genesis"), 0);
        let creator_transaction = Transaction::new(
            "genesis".into(),
            "32a81ba1-34c5-4665-b1e8-1437e388138c".into(),
            10000000,
        );

        let mut balances: HashMap<String, usize> = HashMap::new();
        balances.insert(creator_transaction.sender, creator_transaction.amount);
        balances.insert(creator_transaction.receiver, creator_transaction.amount);

        Blockchain {
            blocks: vec![genesis],
            balances: balances,
        }
    }

    pub fn add_block(&mut self, timestamp: i64, data: Vec<Transaction>) -> Result<(), String> {
        // Validate all transactions first
        for transaction in &data {
            let sender_balance = self.balances.get(&transaction.sender).unwrap_or(&0);
            if *sender_balance < transaction.amount {
                return Err(format!(
                    "Insufficient balance for sender: {}. Has: {}, Needs: {}",
                    transaction.sender, sender_balance, transaction.amount
                ));
            }
        }

        let prev_block = self.blocks.last().unwrap();
        let block = Block::mine_block(
            prev_block.index + 1,
            timestamp,
            data.clone(),
            prev_block.hash.clone(),
        );

        self.blocks.push(block);

        // Apply transactions after successful block addition
        for transaction in data {
            // Safely update sender balance
            let sender_balance = self.balances.entry(transaction.sender.clone()).or_insert(0);
            *sender_balance -= transaction.amount;

            // Safely update receiver balance
            let receiver_balance = self
                .balances
                .entry(transaction.receiver.clone())
                .or_insert(0);
            *receiver_balance += transaction.amount;
        }

        Ok(())
    }

    pub fn validate(&self) -> bool {
        if self.blocks.len() <= 1 {
            return true; // Genesis block or empty blockchain is valid
        }

        for i in 1..self.blocks.len() {
            if self.blocks[i].previous_hash != self.blocks[i - 1].hash {
                return false;
            }
        }
        true
    }

    // Helper method to add initial balance (e.g., for mining rewards or initial distribution)
    pub fn add_balance(&mut self, address: String, amount: usize) {
        let balance = self.balances.entry(address).or_insert(0);
        *balance += amount;
    }

    // Helper method to get balance safely
    pub fn get_balance(&self, address: &str) -> usize {
        *self.balances.get(address).unwrap_or(&0)
    }
}
