use std::ops::{Deref, DerefMut};

pub(crate) trait SizeComputable {
    /// Gets total size of current class, method, or field.
    fn compute_size(&self) -> usize;

    /// Gets current class, method, or field's attribute count.
    fn attributes_count(&self) -> usize;
}

pub(crate) trait ToBytes {
    fn put_bytes(&self, vec: &mut ByteVec);
}

#[derive(Debug, Default)]
pub(crate) struct ByteVec(Vec<u8>);

impl ByteVec {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn as_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn push_u8(&mut self, byte: u8) -> &mut Self {
        self.push(byte);
        self
    }

    pub fn push_u8s(&mut self, bytes: &[u8]) -> &mut Self {
        self.extend_from_slice(bytes);
        self
    }

    pub fn push_u16(&mut self, val: u16) -> &mut Self {
        self.push_u8s(&val.to_be_bytes());
        self
    }

    pub fn push_u32(&mut self, val: u32) -> &mut Self {
        self.push_u8s(&val.to_be_bytes());
        self
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
