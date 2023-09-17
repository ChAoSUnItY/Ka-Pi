use std::ops::{
  Deref,
  DerefMut,
};

pub(crate) trait SizeComputable {
  /// Gets total size of current class, method, or field.
  fn compute_size(&self) -> usize;

  /// Gets current class, method, or field's attribute count.
  fn attributes_count(&self) -> usize;
}

pub(crate) trait ToBytes {
  fn put_bytes(&self, vec: &mut ByteVec);
}

pub trait ByteVector {
  fn push_u8(&mut self, u8: u8) -> &mut Self;

  fn push_u8s(&mut self, u8s: &[u8]) -> &mut Self;

  fn push_u16(&mut self, u16: u16) -> &mut Self;

  fn push_u32(&mut self, u32: u32) -> &mut Self;
}

pub type ByteVec = Vec<u8>;

impl ByteVector for ByteVec {
  fn push_u8(&mut self, u8: u8) -> &mut Self {
    self.push(u8);
    self
  }

  fn push_u8s(&mut self, u8s: &[u8]) -> &mut Self {
    self.extend_from_slice(u8s);
    self
  }

  fn push_u16(&mut self, u16: u16) -> &mut Self {
    self.push_u8s(&u16.to_be_bytes());
    self
  }

  fn push_u32(&mut self, u32: u32) -> &mut Self {
    self.push_u8s(&u32.to_be_bytes());
    self
  }
}
