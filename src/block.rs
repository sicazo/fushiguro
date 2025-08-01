use crate::{transaction::Transaction, util::byte_builder::ByteBuilder};
use serde::{Deserialize, Serialize};
use sha256::digest;

const MINING_DIFFICULTY: usize = 4;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: i64,
    // unix style timestamp
    pub created: i64,
    pub data: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: i64,
        created: i64,
        data: Vec<Transaction>,
        previous_hash: String,
        nonce: u64,
    ) -> Self {
        let mut byte_builder = ByteBuilder::new();
        byte_builder.add_i64(index);
        byte_builder.add_i64(created);
        byte_builder.add_transactions(&data);
        byte_builder.add_string(previous_hash.as_str());
        byte_builder.add_u64(nonce);

        let hash = digest(byte_builder.build());

        Block {
            index,
            created,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn mine_block(
        index: i64,
        created: i64,
        data: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let mut nonce: u64 = 0;

        loop {
            let mut builder = ByteBuilder::new();
            builder.add_i64(index);
            builder.add_i64(created);
            builder.add_transactions(&data);
            builder.add_string(previous_hash.as_str());
            builder.add_u64(nonce);

            let hash = digest(builder.build());

            if hash.starts_with(&"0".repeat(MINING_DIFFICULTY)) {
                return Block {
                    index,
                    created,
                    data,
                    previous_hash,
                    hash,
                    nonce,
                };
            }

            nonce += 1;
        }
    }
}
