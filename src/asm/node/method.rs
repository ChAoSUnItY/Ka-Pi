use serde::{Deserialize, Serialize};

use crate::asm::node::access_flag::MethodAccessFlag;
use crate::asm::node::attribute::AttributeInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Method {
    pub access_flags: Vec<MethodAccessFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_infos_len: u16,
    pub attribute_infos: Vec<AttributeInfo>,
}
