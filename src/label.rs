use bitflags::bitflags;

use crate::byte_vec::{
  ByteVec,
  ByteVector,
};

bitflags! {
  #[derive(Debug, Clone)]
  pub(crate) struct LabelFlag: u8 {
    const DebugOnly = 1;
    const JumpTarget = 2;
    const Resolved = 4;
    const Reachable = 8;
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FowardRefType {
  Short,
  Wide,
}

impl Default for LabelFlag {
  fn default() -> Self {
    Self::empty()
  }
}

#[derive(Debug, Default, Clone)]
pub struct Label {
  flags: LabelFlag,
  line_numbers: Vec<u16>,
  bytecode_offset: u32,
  foward_reference: Vec<(u32, FowardRefType, u32)>,
  input_stack_size: u16,
  output_stack_size: u16,
  output_stack_max: u16,
}

impl Label {
  pub fn new() -> Self {
    Self::default()
  }

  pub(crate) fn offset(&self) -> u32 {
    if !self.flags.contains(LabelFlag::Resolved) {
      panic!("Label offset position has not been resolved yet")
    }

    self.bytecode_offset
  }

  pub(crate) fn flags(&self) -> &LabelFlag {
    &self.flags
  }

  pub(crate) fn add_line_number(&mut self, line_number: u16) {
    self.line_numbers.push(line_number);
  }

  pub(crate) fn put(&mut self, code: &mut ByteVec, source_inst_bytecode_offset: u32, wide_ref: bool) {
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
          .push_u32(self.bytecode_offset.wrapping_sub(source_inst_bytecode_offset));
      } else {
        code
          .push_u16(self.bytecode_offset.wrapping_sub(source_inst_bytecode_offset) as u16);
      }
    }
  }

  fn add_foward_ref(
    &mut self,
    source_inst_bytecode_offset: u32,
    ref_type: FowardRefType,
    ref_handle: u32,
  ) {
    self.foward_reference.push((source_inst_bytecode_offset, ref_type, ref_handle));
  }

  pub(crate) fn resolve(&mut self, code: &mut ByteVec, bytecode_offset: u32) {
    self.flags |= LabelFlag::Resolved;
    self.bytecode_offset = bytecode_offset;

    for (source_inst_bytecode_offset, ref_type, ref_handle) in &self.foward_reference {
      let relative_offset = bytecode_offset.wrapping_sub(*source_inst_bytecode_offset);

      if ((relative_offset as i32) < (i16::MIN as i32) || (relative_offset as i32 > (i16::MAX as i32))) && *ref_type == FowardRefType::Short {
        // reserves 2 more bytes for offset to store when we previously
        // only allocated 2 bytes
        for _ in 0..2 {
          code.insert(*ref_handle as usize, 0);
        }

        // TODO: This algorithm requires the remaining jump insts to check
        // if their offset interval intersects with current foward reference,
        // if so, intersected offset intervals must compute
      } 

      match ref_type {
        FowardRefType::Short => {
          let relative_offset_bytes = (relative_offset as u16).to_be_bytes();

          for i in 0..2 {
            code[*ref_handle as usize + i] = relative_offset_bytes[i]; 
          }
        }
        FowardRefType::Wide => {
          let relative_offset_bytes = (relative_offset as u32).to_be_bytes();

          for i in 0..4 {
            code[*ref_handle as usize + i] = relative_offset_bytes[i]; 
          }
        }
    }
    }
  }
}
