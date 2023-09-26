use std::{
  cell::RefCell,
  rc::Rc,
};

use crate::{
  access_flag::MethodAccessFlag,
  byte_vec::{
    ByteVec,
    ByteVector,
    SizeComputable,
    ToBytes,
  },
  symbol::SymbolTable,
  types::compute_method_descriptor_sizes,
};

pub trait MethodVisitor {
  fn inner(&mut self) -> Option<&mut dyn MethodVisitor> {
    None
  }

  fn visit_code(&mut self) {
    if let Some(inner) = self.inner() {
      inner.visit_code();
    }
  }
}

#[derive(Debug)]
pub struct MethodWriter {
  constant_pool: Rc<RefCell<SymbolTable>>,
  access: MethodAccessFlag,
  name_index: u16,
  descriptor_index: u16,
  signature_index: Option<u16>,
  exception_indicies: Vec<u16>,
  code: ByteVec,
  max_locals: u8,
  max_stacks: u8,
  // Dynamic computing properties
  current_locals: u8,
  current_stacks: u8,
}

impl MethodWriter {
  pub(crate) fn new(
    constant_pool: Rc<RefCell<SymbolTable>>,
    access: MethodAccessFlag,
    name: &str,
    descriptor: &str,
    signature: Option<&str>,
    exceptions: &[&str],
  ) -> Self {
    let cp = constant_pool.clone();
    let mut cp = cp.borrow_mut();
    let name_index = cp.put_utf8(name);
    let descriptor_index = cp.put_utf8(descriptor);
    let signature_index = signature.map(|signature| cp.put_utf8(signature));
    let exception_indicies = exceptions
      .iter()
      .map(|exception| cp.put_class(exception))
      .collect();

    let (max_locals, _) =
      compute_method_descriptor_sizes(descriptor, access.contains(MethodAccessFlag::Static));

    Self {
      constant_pool,
      access,
      name_index,
      descriptor_index,
      signature_index,
      exception_indicies,
      code: ByteVec::default(),
      max_locals,
      max_stacks: 0,
      current_locals: max_locals,
      current_stacks: 0,
    }
  }
}

impl MethodVisitor for MethodWriter {
  fn visit_code(&mut self) {}
}

impl ToBytes for MethodWriter {
  fn put_bytes(&self, vec: &mut ByteVec) {
    let attributes_count = self.attributes_count();

    vec.push_u16(self.access.bits());
    vec.push_u16(self.name_index);
    vec.push_u16(self.descriptor_index);
    vec.push_u16(attributes_count as u16);
  }
}

impl SizeComputable for MethodWriter {
  fn compute_size(&self) -> usize {
    let mut size = 8;

    if self.signature_index.is_some() {
      size += 8;
    }

    if !self.exception_indicies.is_empty() {
      size += 8 + 2 * self.exception_indicies.len();
    }

    if !self.code.is_empty() {
      size += 16 + self.code.len();
    }

    size
  }

  fn attributes_count(&self) -> usize {
    let mut size = 0;

    if self.signature_index.is_some() {
      size += 1;
    }

    if !self.exception_indicies.is_empty() {
      size += 1;
    }

    if !self.code.is_empty() {
      size += 1;
    }

    size
  }
}
