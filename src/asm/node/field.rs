use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::node::access_flag::FieldAccessFlag;
use crate::asm::node::attribute::AttributeInfo;
use crate::asm::node::constant::{Constant, ConstantPool, Utf8};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub access_flags: Vec<FieldAccessFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_infos_len: u16,
    pub attribute_infos: Vec<AttributeInfo>,
}

//noinspection DuplicatedCode
impl Field {
    pub fn name<'method, 'constant_pool: 'method>(
        &'method self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.name_index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn descriptor<'method, 'constant_pool: 'method>(
        &'method self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.descriptor_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

//noinspection DuplicatedCode
impl ConstantRearrangeable for Field {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);
        Self::rearrange_index(&mut self.descriptor_index, rearrangements);

        for attribute in &mut self.attribute_infos {
            attribute.rearrange(rearrangements)?;
        }

        Ok(())
    }
}
