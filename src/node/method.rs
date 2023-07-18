use serde::{Deserialize, Serialize};

use crate::node::access_flag::MethodAccessFlag;
use crate::node::attribute::AttributeInfo;
use crate::node::constant::{ConstantPool, Utf8};
use crate::node::{Node, Nodes};
use crate::visitor::method::MethodVisitor;
use crate::visitor::Visitable;

/// Represents a class method.
///
/// See [4.6 Methods](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=111).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Method {
    pub access_flags: Node<Vec<MethodAccessFlag>>,
    pub name_index: Node<u16>,
    pub descriptor_index: Node<u16>,
    pub attribute_infos_len: Node<u16>,
    pub attribute_infos: Nodes<AttributeInfo>,
}

//noinspection DuplicatedCode
impl Method {
    /// Get name of method from constant pool.
    pub fn name<'method, 'constant_pool: 'method>(
        &'method self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.name_index)
    }

    /// Get descriptor of method from constant pool.
    pub fn descriptor<'method, 'constant_pool: 'method>(
        &'method self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(*self.descriptor_index)
    }
}

impl<MV> Visitable<MV> for Method
where
    MV: MethodVisitor,
{
    fn visit(&self, visitor: &mut MV) {
        // TODO: Implement with MethodVisitor here
    }
}
