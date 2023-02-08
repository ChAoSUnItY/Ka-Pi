use std::rc::Rc;

use crate::{constants, edge::Edge, error::RasmError, frame::Frame, opcodes, utils::{replace, Rev}};

pub(crate) trait Label {
    fn flags(&self) -> u8;
    fn bytecode_offset(&self) -> i32;
    fn input_stack_size(&self) -> u16;
    fn output_stack_size(&self) -> u16;
    fn output_stack_max(&self) -> u16;
    fn subroutine_id(&self) -> u16;
    fn frame(&self) -> Option<Rc<dyn Frame>>;
    fn next_basic_block(&self) -> Option<Rc<Self>>
    where
        Self: Sized;
    fn outgoing_edges(&self) -> Option<Rc<Edge>>;
    fn next_list_element(&self) -> Option<Rc<Self>>
    where
        Self: Sized;
}

#[allow(unused)]
impl dyn Label {
    pub(crate) const FLAG_DEBUG_ONLY: u8 = 0b00000001;
    pub(crate) const FLAG_JUMP_TARGET: u8 = 0b00000010;
    pub(crate) const FLAG_RESOLVED: u8 = 0b00000100;
    pub(crate) const FLAG_REACHABLE: u8 = 0b00001000;
    pub(crate) const FLAG_SUBROUTINE_CALLER: u8 = 0b00010000;
    pub(crate) const FLAG_SUBROUTINE_START: u8 = 0b00100000;
    pub(crate) const FLAG_SUBROUTINE_END: u8 = 0b01000000;
    pub(crate) const FLAG_LINE_NUMBER: u8 = 0b10000000;

    pub(crate) const LINE_NUMBERS_CAPACITY_INCREMENT: u32 = 4;
    pub(crate) const FORWARD_REFERENCES_CAPACITY_INCREMENT: u32 = 6;

    pub(crate) const FORWARD_REFERENCE_TYPE_MASK: i32 = 0xF0000000i64 as i32; // Force overflow
    pub(crate) const FORWARD_REFERENCE_TYPE_SHORT: i32 = 0x10000000;
    pub(crate) const FORWARD_REFERENCE_TYPE_WIDE: i32 = 0x20000000;
    pub(crate) const FORWARD_REFERENCE_HANDLE_MASK: i32 = 0x0FFFFFFF;

    pub(crate) const EMPTY_LIST: LabelImpl = LabelImpl::new();
}

pub(crate) struct LabelImpl {
    pub(crate) flags: u8,
    line_number: u32,
    other_line_numbers: Option<Vec<u32>>,
    pub(crate) bytecode_offset: i32,
    forward_references: Option<Vec<i32>>,
    pub(crate) input_stack_size: u16,
    pub(crate) output_stack_size: u16,
    pub(crate) output_stack_max: u16,
    pub(crate) subroutine_id: u16,
    pub(crate) frame: Option<Rc<dyn Frame>>,
    pub(crate) next_basic_block: Option<Rc<Self>>,
    pub(crate) outgoing_edges: Option<Rc<Edge>>,
    pub(crate) next_list_element: Option<Rc<Self>>,
}

impl LabelImpl {
    pub(crate) const fn new() -> Self {
        Self {
            flags: 0,
            line_number: 0,
            other_line_numbers: None,
            bytecode_offset: 0,
            forward_references: None,
            input_stack_size: 0,
            output_stack_size: 0,
            output_stack_max: 0,
            subroutine_id: 0,
            frame: None,
            next_basic_block: None,
            outgoing_edges: None,
            next_list_element: None,
        }
    }
}

impl LabelImpl {
    pub fn getOffset(&self) -> Result<i32, RasmError> {
        if self.flags() & <dyn Label>::FLAG_RESOLVED == 0 {
            Err(RasmError::StateError {
                cause: "Label offset position has not been resolved yet",
            })
        } else {
            Ok(self.bytecode_offset())
        }
    }

    pub(crate) fn get_canonical_instance(self: Rc<Self>) -> Option<Rc<dyn Label>> {
        if self.frame.is_none() {
            Some(self)
        } else {
            self.frame().map(|f| f.owner())
        }
    }

    pub(crate) fn add_line_number(&mut self, line_number: u32) {
        if self.flags() & <dyn Label>::FLAG_LINE_NUMBER == 0 {
            self.flags |= <dyn Label>::FLAG_LINE_NUMBER;
            self.line_number = line_number;
            return;
        }

        if self.other_line_numbers.is_none() {
            let _ = self.other_line_numbers.insert(vec![]);
        }

        if let Some(other_line_numbers) = &mut self.other_line_numbers {
            other_line_numbers.push(line_number);
        }
    }

    // TODO: accept

    // TODO: put

    pub(crate) fn add_foward_reference(
        &mut self,
        source_inst_bytecode_offset: i32,
        reference_type: u32,
        reference_handle: u32,
    ) {
        if self.forward_references.is_none() {
            let _ = self.forward_references.insert(vec![]);
        }

        if let Some(foward_references) = &mut self.forward_references {
            foward_references.push(source_inst_bytecode_offset);
            foward_references.push((reference_type | reference_handle).try_into().unwrap());
        }
    }

    pub(crate) fn resolve(&mut self, code: &mut Vec<u8>, bytecode_offset: i32) -> bool {
        self.flags |= <dyn Label>::FLAG_RESOLVED;
        self.bytecode_offset = bytecode_offset;

        if self.forward_references.is_none() {
            return false;
        }

        let mut has_asm_instructions = false;

        if let Some(forward_references) = &self.forward_references {
            for i in 1..forward_references.len() {
                let source_inst_bytecode_offset = forward_references[i - 1];
                let reference = forward_references[i];
                let relative_offset = bytecode_offset - source_inst_bytecode_offset;
                let offset = relative_offset.as_rev();
                let handle = (reference & <dyn Label>::FORWARD_REFERENCE_HANDLE_MASK) as usize;

                if reference & <dyn Label>::FORWARD_REFERENCE_TYPE_MASK
                    == <dyn Label>::FORWARD_REFERENCE_TYPE_SHORT
                {
                    if relative_offset < i16::MIN as i32 && relative_offset > i16::MAX as i32 {
                        let opcode = code[source_inst_bytecode_offset as usize] & 0xFF;

                        if opcode < opcodes::IFNULL {
                            // Change IFEQ ... JSR to ASM_IFEQ ... ASM_JSR.
                            code[source_inst_bytecode_offset as usize] =
                                opcode + constants::ASM_OPCODE_DELTA;
                        } else {
                            // Change IFNULL and IFNONNULL to ASM_IFNULL and ASM_IFNONNULL.
                            code[source_inst_bytecode_offset as usize] =
                                opcode + constants::ASM_IFNULL_OPCODE_DELTA;
                        }

                        has_asm_instructions = true;
                    }

                    replace(handle, code, offset, 2);
                } else {
                    replace(handle, code, offset, 4);
                }
            }
        }

        has_asm_instructions
    }
}

impl Label for LabelImpl {
    fn flags(&self) -> u8 {
        self.flags
    }

    fn bytecode_offset(&self) -> i32 {
        self.bytecode_offset
    }

    fn input_stack_size(&self) -> u16 {
        self.input_stack_size
    }

    fn output_stack_size(&self) -> u16 {
        self.output_stack_size
    }

    fn output_stack_max(&self) -> u16 {
        self.output_stack_max
    }

    fn subroutine_id(&self) -> u16 {
        self.subroutine_id
    }

    fn frame(&self) -> Option<Rc<dyn Frame>> {
        self.frame.clone()
    }

    fn next_basic_block(&self) -> Option<Rc<Self>>
    where
        Self: Sized,
    {
        self.next_basic_block.clone()
    }

    fn outgoing_edges(&self) -> Option<Rc<Edge>> {
        self.outgoing_edges.clone()
    }

    fn next_list_element(&self) -> Option<Rc<Self>>
    where
        Self: Sized,
    {
        self.next_list_element.clone()
    }
}
