use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::node::access_flag::MethodAccessFlag;
use crate::asm::node::attribute::AttributeInfo;
use crate::asm::node::constant::{Constant, ConstantPool, Utf8};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

/// Represents a class method.
///
/// See [4.6 Methods](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=111).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Method {
    pub access_flags: Vec<MethodAccessFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_infos_len: u16,
    pub attribute_infos: Vec<AttributeInfo>,
}

//noinspection DuplicatedCode
impl Method {
    /// Get name of method from constant pool.
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

    /// Get descriptor of method from constant pool.
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
impl ConstantRearrangeable for Method {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.name_index, rearrangements);
        Self::rearrange_index(&mut self.descriptor_index, rearrangements);

        for attribute in &mut self.attribute_infos {
            attribute.rearrange(rearrangements)?;
        }

        Ok(())
    }
}
