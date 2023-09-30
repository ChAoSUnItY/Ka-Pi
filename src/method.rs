use std::{
  cell::RefCell,
  collections::HashMap,
  rc::Rc,
};

use crate::{
  access_flag::MethodAccessFlag,
  attrs,
  byte_vec::{
    ByteVec,
    ByteVector,
    SizeComputable,
    ToBytes,
  },
  label::{
    Label,
    LabelFlag,
  },
  opcodes,
  constant::ConstantPool,
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

  fn visit_inst(&mut self, inst: u8) {
    if let Some(inner) = self.inner() {
      inner.visit_inst(inst);
    }
  }

  fn visit_label(&mut self, label: &mut Label) {
    if let Some(inner) = self.inner() {
      inner.visit_label(label);
    }
  }

  fn visit_jump_inst(&mut self, opcode: u8, label: &mut Label) {
    if let Some(inner) = self.inner() {
      inner.visit_jump_inst(opcode, label);
    }
  }
}

#[derive(Debug)]
pub struct MethodWriter {
  constant_pool: Rc<RefCell<ConstantPool>>,
  access: MethodAccessFlag,
  name_index: u16,
  descriptor_index: u16,
  signature_index: Option<u16>,
  exception_indicies: Vec<u16>,
  code: ByteVec,
  max_locals: u16,
  max_stacks: u16,
  // Dynamic computing properties
  current_locals: u16,
  current_stacks: u16,
  labels: HashMap<u32, Label>,
}

impl MethodWriter {
  pub(crate) fn new(
    constant_pool: Rc<RefCell<ConstantPool>>,
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
      labels: HashMap::new(),
    }
  }

  fn code_attributes_count(&self) -> u16 {
    // TODO
    let mut count = 0;
    count
  }

  fn compute_exception_table_size(&self) -> u32 {
    2 /* TODO: + 8 * exceptions */
  }
}

impl MethodVisitor for MethodWriter {
  fn visit_code(&mut self) {
    let mut cp = self.constant_pool.borrow_mut();

    cp.put_utf8(attrs::CODE);

    drop(cp);

    let mut label = Label::default();

    self.visit_label(&mut label);

    self.labels.insert(label.offset(), label);
  }

  fn visit_inst(&mut self, inst: u8) {
      self.code.push_u8(inst);
  }

  fn visit_label(&mut self, label: &mut Label) {
    let bytecode_len = self.code.len() as u32;

    label.resolve(&mut self.code, bytecode_len);
  }

  fn visit_jump_inst(&mut self, opcode: u8, label: &mut Label) {
    let bytecode_len = self.code.len() as u32;
    let base_opcode = if opcode >= opcodes::GOTO_W {
      opcode - 33
    } else {
      opcode
    };

    if label.flags().contains(LabelFlag::Resolved)
      && (label.offset().wrapping_sub(bytecode_len) as i32) < (i16::MIN as i32)
    {
      match base_opcode {
        opcodes::GOTO => {
          self.code.push_u8(opcodes::GOTO_W);
        }
        opcodes::JSR => {
          self.code.push_u8(opcodes::JSR_W);
        }
        _ => {
          let flipped_branch_opcode = if base_opcode >= opcodes::IFNULL {
            base_opcode ^ 1
          } else {
            ((base_opcode + 1) ^ 1) - 1
          };

          self
            .code
            .push_u8(flipped_branch_opcode)
            .push_u16(8)
            .push_u8(opcodes::GOTO_W);
        }
      }

      let bytecode_len = self.code.len() as u32;

      label.put(&mut self.code, bytecode_len - 1, true);
    } else if base_opcode != opcode {
      self.code.push_u8(opcode);

      let bytecode_len = self.code.len() as u32;

      label.put(&mut self.code, bytecode_len - 1, true);
    } else {
      self.code.push_u8(base_opcode);

      let bytecode_len = self.code.len() as u32;

      label.put(&mut self.code, bytecode_len - 1, false);
    }
  }
}

impl ToBytes for MethodWriter {
  fn put_bytes(&self, vec: &mut ByteVec) {
    let cp = self.constant_pool.borrow();
    let attributes_count = self.attributes_count();

    vec.push_u16(self.access.bits());
    vec.push_u16(self.name_index);
    vec.push_u16(self.descriptor_index);
    vec.push_u16(attributes_count as u16);

    if !self.code.is_empty() {
      let code_attr_size = 10 + self.code.len() as u32 + self.compute_exception_table_size();

      vec
        .push_u16(cp.get_utf8(attrs::CODE).unwrap())
        .push_u32(code_attr_size)
        .push_u16(self.max_stacks)
        .push_u16(self.max_locals)
        .push_u32(self.code.len() as u32)
        .push_u8s(&self.code);

      // TODO: Compute exception table
      vec.push_u16(0);

      // TODO: Compute attributes
      vec.push_u16(self.code_attributes_count());
    }
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
