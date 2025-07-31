pub struct ByteBuilder {
    bytes: Vec<u8>,
}

impl ByteBuilder {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn add_string(mut self, s: &str) -> Self {
        self.bytes.extend_from_slice(s.as_bytes());
        self
    }

    pub fn add_i64(mut self, val: i64) -> Self {
        self.bytes.extend_from_slice(&val.to_le_bytes());
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.bytes
    }
}
