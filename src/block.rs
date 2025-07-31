use crate::util::byte_builder::ByteBuilder;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: i64,
    // unix style timestamp
    pub created: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: Option<String>,
}

impl Block {
    pub fn new(
        index: i64,
        created: i64,
        data: String,
        previous_hash: String,
        nonce: Option<String>,
    ) -> Self {
        let nonce_cpy = nonce.clone();
        let mut byte_builder = ByteBuilder::new();
        byte_builder = byte_builder.add_i64(index);
        byte_builder = byte_builder.add_i64(created);
        byte_builder = byte_builder.add_string(data.as_str());
        byte_builder = byte_builder.add_string(previous_hash.as_str());
        byte_builder = byte_builder.add_string(nonce_cpy.unwrap_or_else(|| String::new()).as_str());

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
}
