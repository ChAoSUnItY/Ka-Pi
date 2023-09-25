use bitflags::bitflags;

use crate::byte_vec::{
  ByteVec,
  ByteVector,
};

bitflags! {
  #[derive(Debug)]
  pub(crate) struct LabelFlag: u8 {
    const DebugOnly = 1;
    const JumpTarget = 2;
    const Resolved = 4;
    const Reachable = 8;
  }
}

#[repr(u32)]
#[derive(Debug)]
enum FowardRefType {
  Wide = 0x10000000,
  Short = 0x20000000,
}

impl FowardRefType {
  pub(crate) const MASK: u32 = 0xF0000000;
}

impl Default for LabelFlag {
  fn default() -> Self {
    Self::empty()
  }
}

#[derive(Debug, Default)]
pub struct Label {
  flags: LabelFlag,
  line_numbers: Vec<u16>,
  bytecode_offset: u32,
  foward_reference: Vec<u32>,
  input_stack_size: u16,
  output_stack_size: u16,
  output_stack_max: u16,
}

impl Label {
  pub fn new() -> Self {
    Self::default()
  }

  fn offset(&self) -> u32 {
    if !self.flags.contains(LabelFlag::Resolved) {
      panic!("Label offset position has not been resolved yet")
    }

    self.bytecode_offset
  }

  fn add_line_number(&mut self, line_number: u16) {
    self.line_numbers.push(line_number);
  }

  fn put(&mut self, code: &mut ByteVec, source_inst_bytecode_offset: u32, wide_ref: bool) {
    if !self.flags.contains(LabelFlag::Resolved) {
      if wide_ref {
        self.add_foward_ref(
          source_inst_bytecode_offset,
          FowardRefType::Wide,
          code.len() as u32,
        );
        code.push_u32(0);
      } else {
        self.add_foward_ref(
          source_inst_bytecode_offset,
          FowardRefType::Short,
          code.len() as u32,
        );
        code.push_u16(0);
      }
    } else {
      if wide_ref {
        code
          .push_u32((self.bytecode_offset as u32).wrapping_sub(source_inst_bytecode_offset as u32));
      } else {
        code
          .push_u16((self.bytecode_offset as u16).wrapping_sub(source_inst_bytecode_offset as u16));
      }
    }
  }

  fn add_foward_ref(
    &mut self,
    source_inst_bytecode_offset: u32,
    ref_type: FowardRefType,
    ref_handle: u32,
  ) {
    self.foward_reference.push(source_inst_bytecode_offset);
    self.foward_reference.push(ref_type as u32 | ref_handle);
  }
}
