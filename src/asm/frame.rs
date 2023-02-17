use std::rc::Rc;

use crate::asm::label::Label;

pub(crate) const SAME_FRAME: i32 = 0;
pub(crate) const SAME_LOCALS_1_STACK_ITEM_FRAME: i32 = 64;
pub(crate) const RESERVED: i32 = 128;
pub(crate) const SAME_LOCALS_1_STACK_ITEM_FRAME_EXTENDED: i32 = 247;
pub(crate) const CHOP_FRAME: i32 = 248;
pub(crate) const SAME_FRAME_EXTENDED: i32 = 251;
pub(crate) const APPEND_FRAME: i32 = 252;
pub(crate) const FULL_FRAME: i32 = 255;

pub(crate) const ITEM_TOP: u8 = 0;
pub(crate) const ITEM_INTEGER: u8 = 1;
pub(crate) const ITEM_FLOAT: u8 = 2;
pub(crate) const ITEM_DOUBLE: u8 = 3;
pub(crate) const ITEM_LONG: u8 = 4;
pub(crate) const ITEM_NULL: u8 = 5;
pub(crate) const ITEM_UNINITIALIZED_THIS: u8 = 6;
pub(crate) const ITEM_OBJECT: u8 = 7;
pub(crate) const ITEM_UNINITIALIZED: u8 = 8;

pub(crate) const ITEM_ASM_BOOLEAN: i32 = 9;
pub(crate) const ITEM_ASM_BYTE: i32 = 10;
pub(crate) const ITEM_ASM_CHAR: i32 = 11;
pub(crate) const ITEM_ASM_SHORT: i32 = 12;

pub(crate) const DIM_SIZE: i32 = 6;
pub(crate) const KIND_SIZE: i32 = 4;
pub(crate) const FLAGS_SIZE: i32 = 2;
pub(crate) const VALUE_SIZE: i32 = 32 - DIM_SIZE - KIND_SIZE - FLAGS_SIZE;

pub(crate) const DIM_SHIFT: i32 = KIND_SIZE + FLAGS_SIZE + VALUE_SIZE;
pub(crate) const KIND_SHIFT: i32 = FLAGS_SIZE + VALUE_SIZE;
pub(crate) const FLAGS_SHIFT: i32 = VALUE_SIZE;

pub(crate) const DIM_MASK: i32 = ((1 << DIM_SIZE) - 1) << DIM_SHIFT;
pub(crate) const KIND_MASK: i32 = ((1 << KIND_SIZE) - 1) << KIND_SHIFT;
pub(crate) const VALUE_MASK: i32 = (1 << VALUE_SIZE) - 1;

pub(crate) const ARRAY_OF: i32 = 1 << DIM_SHIFT;

pub(crate) const ELEMENT_OF: i32 = -1 << DIM_SHIFT as i8;

pub(crate) const CONSTANT_KIND: i32 = 1 << KIND_SHIFT;
pub(crate) const REFERENCE_KIND: i32 = 2 << KIND_SHIFT;
pub(crate) const UNINITIALIZED_KIND: i32 = 3 << KIND_SHIFT;
pub(crate) const LOCAL_KIND: i32 = 4 << KIND_SHIFT;
pub(crate) const STACK_KIND: i32 = 5 << KIND_SHIFT;

pub(crate) const TOP_IF_LONG_OR_DOUBLE_FLAG: i32 = 1 << FLAGS_SHIFT;

pub(crate) const TOP: i32 = CONSTANT_KIND | ITEM_TOP as i32;
pub(crate) const BOOLEAN: i32 = CONSTANT_KIND | ITEM_ASM_BOOLEAN;
pub(crate) const BYTE: i32 = CONSTANT_KIND | ITEM_ASM_BYTE;
pub(crate) const CHAR: i32 = CONSTANT_KIND | ITEM_ASM_CHAR;
pub(crate) const SHORT: i32 = CONSTANT_KIND | ITEM_ASM_SHORT;
pub(crate) const INTEGER: i32 = CONSTANT_KIND | ITEM_INTEGER as i32;
pub(crate) const FLOAT: i32 = CONSTANT_KIND | ITEM_FLOAT as i32;
pub(crate) const LONG: i32 = CONSTANT_KIND | ITEM_LONG as i32;
pub(crate) const DOUBLE: i32 = CONSTANT_KIND | ITEM_DOUBLE as i32;
pub(crate) const NULL: i32 = CONSTANT_KIND | ITEM_NULL as i32;
pub(crate) const UNINITIALIZED_THIS: i32 = CONSTANT_KIND | ITEM_UNINITIALIZED_THIS as i32;

pub(crate) trait Frame {
    fn owner(&self) -> Rc<dyn Label>;
}

pub(crate) struct FrameImpl {
    owner: Rc<dyn Label>,
    input_locals: Vec<i32>,
    input_stack: Vec<i32>,
    output_locals: Vec<i32>,
    output_stack: Vec<i32>,
    output_stack_start: i8,
    output_stack_top: i8,
    initialization_count: i32,
    initializations: Vec<i32>,
}

impl FrameImpl {
    pub(crate) const fn new(owner: Rc<dyn Label>) -> Self {
        Self {
            owner,
            input_locals: vec![],
            input_stack: vec![],
            output_locals: vec![],
            output_stack: vec![],
            output_stack_start: 0,
            output_stack_top: 0,
            initialization_count: 0,
            initializations: vec![],
        }
    }

    pub(crate) fn from(&mut self, frame: &FrameImpl) {
        self.input_locals.clone_from(&frame.input_locals);
        self.input_stack.clone_from(&frame.input_stack);
        self.output_locals.clone_from(&frame.output_locals);
        self.output_stack.clone_from(&frame.output_stack);
        self.output_stack_start = 0;
        self.output_stack_top = frame.output_stack_top;
        self.initialization_count = frame.initialization_count;
        self.initializations.clone_from(&frame.initializations);
    }
}

impl Frame for FrameImpl {
    fn owner(&self) -> Rc<dyn Label> {
        self.owner.clone()
    }
}
