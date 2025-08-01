use bincode::Encode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Encode)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: usize,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: usize) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}
