use crate::transaction::{self, Transaction};

pub struct ByteBuilder {
    bytes: Vec<u8>,
}

impl ByteBuilder {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn add_string(&mut self, s: &str) -> &mut Self {
        self.bytes.extend_from_slice(s.as_bytes());
        self
    }

    pub fn add_i64(&mut self, val: i64) -> &mut Self {
        self.bytes.extend_from_slice(&val.to_le_bytes());
        self
    }

    pub fn add_u64(&mut self, val: u64) -> &mut Self {
        self.bytes.extend_from_slice(&val.to_le_bytes());
        self
    }

    pub fn add_transactions(&mut self, transactions: &Vec<Transaction>) -> &mut Self {
        let serialized =
            bincode::encode_to_vec(transactions, bincode::config::standard()).unwrap_or_default();
        self.bytes.extend_from_slice(&serialized);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.bytes
    }
}
