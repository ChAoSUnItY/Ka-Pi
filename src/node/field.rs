use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::KapiResult;
use crate::node::access_flag::FieldAccessFlag;
use crate::node::attribute::{Attribute, AttributeInfo};
use crate::node::constant::{Constant, ConstantPool, Utf8};
use crate::node::ConstantRearrangeable;
use crate::visitor::field::FieldVisitor;
use crate::visitor::Visitable;

/// Represents a class field.
///
/// See [4.5 Fields](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=109).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Field {
    pub access_flags: Vec<FieldAccessFlag>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_infos_len: u16,
    pub attribute_infos: Vec<AttributeInfo>,
}

//noinspection DuplicatedCode
impl Field {
    /// Get name of field from constant pool.
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

    /// Get descriptor of field from constant pool.
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

impl<FV> Visitable<FV> for Field
where
    FV: FieldVisitor,
{
    fn visit(&mut self, visitor: &mut FV) {
        for attribute_info in &mut self.attribute_infos {
            if let AttributeInfo {
                attribute: Some(Attribute::ConstantValue(constant_value)),
                ..
            } = attribute_info
            {
                visitor.visit_constant(constant_value);
            }
        }
    }
}
