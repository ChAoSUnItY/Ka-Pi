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
    }
}

impl Attribute {
    pub const fn name(&self) -> &'static str {
        match self {
            Attribute::ConstantValue { .. } => constants::CONSTANT_VALUE,
            Attribute::Code { .. } => constants::CODE,
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
                        * *attributes_length as u32
            }
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
