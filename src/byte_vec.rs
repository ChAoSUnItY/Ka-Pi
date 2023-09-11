use std::ops::{Deref, DerefMut};

pub(crate) trait ToBytes {
    fn put_bytes(&self, vec: &mut ByteVec);
}

#[derive(Debug, Default)]
pub(crate) struct ByteVec(Vec<u8>);

impl ByteVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn as_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.push(byte);
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    pub fn push_11(&mut self, byte1: u8, byte2: u8) {
        self.push(byte1);
        self.push(byte2);
    }

    pub fn push_u16(&mut self, val: u16) {
        self.push_bytes(&val.to_be_bytes());
    }

    pub fn push_u32(&mut self, val: u32) {
        self.push_bytes(&val.to_be_bytes());
    }
}

impl Deref for ByteVec {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
