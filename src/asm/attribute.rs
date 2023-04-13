use serde::{Deserialize, Serialize};

use crate::asm::constants;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}

impl Eq for ConstantValue {}

impl From<i32> for ConstantValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for ConstantValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for ConstantValue {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<f64> for ConstantValue {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<String> for ConstantValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ConstantValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<Exception>,
        attributes_length: u16,
        attributes: Vec<Attribute>,
    },
    StackMapTable {
        number_of_entries: u16,
        entries: Vec<StackMapEntry>,
    },
    Exceptions {
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        number_of_classes: u16,
        class: Vec<InnerClass>,
    },
}

impl Attribute {
    pub const fn name(&self) -> &'static str {
        match self {
            Attribute::ConstantValue { .. } => constants::CONSTANT_VALUE,
            Attribute::Code { .. } => constants::CODE,
            Attribute::StackMapTable { .. } => constants::STACK_MAP_TABLE,
            Attribute::Exceptions { .. } => constants::EXCEPTIONS,
            Attribute::InnerClasses { .. } => constants::INNER_CLASSES,
        }
    }

    pub fn attribute_len(&self) -> u32 {
        match self {
            Attribute::ConstantValue { .. } => 2,
            Attribute::Code {
                max_stack: _,
                max_locals: _,
                code_length,
                code: _,
                exception_table_length,
                exception_table: _,
                attributes_length,
                attributes,
            } => {
                // max_stack: 2
                // max_locals: 2
                // code_length: 4
                // code: code_length
                // exception_table_length: 2
                // exception_table: [Exception] (8) * exception_table_length
                //     start_pc: 2
                //     end_pc: 2
                //     handler_pc: 2
                //     catch_type: 2
                //     `total`: 8
                // attributes_length: 2
                // attributes: [Attribute] * attributes_length
                // `total`: 12 + code_length + 8 * exception_table_length + [Attribute] * attribute_length
                12 + *code_length as u32
                    + 8 * *exception_table_length as u32
                    + attributes.iter().map(Attribute::attribute_len).sum::<u32>()
            }
            Attribute::StackMapTable {
                number_of_entries: _,
                entries,
            } => 2 + entries.iter().map(StackMapEntry::len).sum::<u32>(),
            Attribute::Exceptions {
                number_of_exceptions,
                exception_index_table: _,
            } => 2 + 2 * *number_of_exceptions as u32,
            Attribute::InnerClasses {
                number_of_classes,
                class: _,
            } => 8 * *number_of_classes as u32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackMapEntry {
    Same,
    SameLocal1StackItem {
        stack: VerificationType,
    },
    SameLocal1StackItemExtended {
        offset_delta: u16,
        stack: VerificationType,
    },
    Chop {
        offset_delta: u16,
    },
    SameExtended {
        frame_type: AppendType,
        offset_delta: u16,
    },
    Full {
        offset_delta: u16,
        numbers_of_locals: u16,
        locals: Vec<VerificationType>,
        number_of_stack_items: u16,
        stack: Vec<VerificationType>,
    },
}

impl StackMapEntry {
    pub fn len(&self) -> u32 {
        // frame_type: 1
        1 + match self {
            StackMapEntry::Same => 0,
            StackMapEntry::SameLocal1StackItem { stack } => stack.len(),
            StackMapEntry::SameLocal1StackItemExtended {
                offset_delta: _,
                stack,
            } => 2 + stack.len(),
            StackMapEntry::Chop { .. } => 2,
            StackMapEntry::SameExtended { .. } => 4,
            StackMapEntry::Full {
                offset_delta: _,
                numbers_of_locals: _,
                locals,
                number_of_stack_items: _,
                stack,
            } => {
                2 + locals.iter().map(VerificationType::len).sum::<u32>()
                    + stack.iter().map(VerificationType::len).sum::<u32>()
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppendType {
    One,
    Two,
    Three,
}

// noinspection ALL
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    NullVariable,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized,
}

impl VerificationType {
    pub const fn len(&self) -> u32 {
        match self {
            Self::Object { .. } => 2,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerClass {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}
