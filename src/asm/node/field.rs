use serde::{Serialize, Deserialize};
use crate::asm::node::access_flag::FieldAccessFlag;
use crate::asm::node::attribute::AttributeInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub access_flags: Vec<FieldAccessFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_infos_len: u16,
    pub attribute_infos: Vec<AttributeInfo>,
}
