use serde::{Deserialize, Serialize};

use crate::node::access_flag::FieldAccessFlag;
use crate::node::attribute::{Attribute, AttributeInfo};
use crate::node::constant::{ConstantPool, Utf8};
use crate::visitor::field::FieldVisitor;
use crate::visitor::Visitable;

/// Represents a class field.
///
/// See [4.5 Fields](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=109).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
        constant_pool.get_utf8(self.name_index)
    }

    /// Get descriptor of field from constant pool.
    pub fn descriptor<'method, 'constant_pool: 'method>(
        &'method self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.descriptor_index)
    }
}

impl<FV> Visitable<FV> for Field
where
    FV: FieldVisitor,
{
    fn visit(&self, visitor: &mut FV) {
        for attribute_info in &self.attribute_infos {
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
