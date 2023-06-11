use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::asm::node::access_flag::ClassAccessFlag;
use crate::asm::node::attribute::AttributeInfo;
use crate::asm::node::constant::{Constant, ConstantPool};
use crate::asm::node::field::Field;
use crate::asm::node::method::Method;
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

/// Represents a class file.
///
/// See [4.1 The ClassFile Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=82).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Class {
    pub java_version: JavaVersion,
    pub constant_pool_count: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: Vec<ClassAccessFlag>,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<Field>,
    pub methods_count: u16,
    pub methods: Vec<Method>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl Class {
    /// Get current class from constant pool.
    pub fn this_class(&self) -> Option<&crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self.constant_pool.get(self.this_class) {
            Some(class)
        } else {
            None
        }
    }

    /// Get super class from constant pool.
    pub fn super_class(&self) -> Option<&crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self.constant_pool.get(self.super_class) {
            Some(class)
        } else {
            None
        }
    }

    /// Get interface from constant pool at given index.
    pub fn interface(&self, index: usize) -> Option<&crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self
            .interfaces
            .get(index)
            .map(|interface_index| self.constant_pool.get(*interface_index))
            .flatten()
        {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for Class {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.this_class, rearrangements);
        Self::rearrange_index(&mut self.super_class, rearrangements);

        self.constant_pool.rearrange(rearrangements)?;

        for interface in &mut self.interfaces {
            Self::rearrange_index(interface, rearrangements);
        }

        for field in &mut self.fields {
            field.rearrange(rearrangements)?;
        }

        for method in &mut self.methods {
            method.rearrange(rearrangements)?;
        }

        for attribute in &mut self.attributes {
            attribute.rearrange(rearrangements)?;
        }

        Ok(())
    }
}

/// Represents java version documented in specification (combines `major_version` and `minor_version`).
///
/// See [Table 4.1-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=83).
#[repr(u32)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum JavaVersion {
    V1_1 = 3 << 16 | 45,
    V1_2 = 0 << 16 | 46,
    V1_3 = 0 << 16 | 47,
    V1_4 = 0 << 16 | 48,
    V1_5 = 0 << 16 | 49,
    V1_6 = 0 << 16 | 50,
    V1_7 = 0 << 16 | 51,
    V1_8 = 0 << 16 | 52,
    V9 = 0 << 16 | 53,
    V10 = 0 << 16 | 54,
    V11 = 0 << 16 | 55,
    V12 = 0 << 16 | 56,
    V13 = 0 << 16 | 57,
    V14 = 0 << 16 | 58,
    V15 = 0 << 16 | 59,
    V16 = 0 << 16 | 60,
    V17 = 0 << 16 | 61,
    V18 = 0 << 16 | 62,
    V19 = 0 << 16 | 63,
    V20 = 0 << 16 | 64,
    V21 = 0 << 16 | 65,
}
