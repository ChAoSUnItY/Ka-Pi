use std::collections::HashMap;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::asm::node::access_flag::ClassAccessFlag;
use crate::asm::node::attribute::AttributeInfo;
use crate::asm::node::constant::{Constant, ConstantPool};
use crate::asm::node::field::Field;
use crate::asm::node::method::Method;
use crate::asm::node::ConstantRearrangeable;
use crate::asm::visitor::class::ClassVisitor;
use crate::asm::visitor::constant::ConstantVisitor;
use crate::asm::visitor::field::FieldVisitor;
use crate::asm::visitor::method::MethodVisitor;
use crate::asm::visitor::Visitable;
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

    pub fn this_class_mut(&mut self) -> Option<&mut crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self.constant_pool.get_mut(self.this_class) {
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

    /// Get super class from constant pool.
    pub fn super_class_mut(&mut self) -> Option<&mut crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self.constant_pool.get_mut(self.super_class) {
            Some(class)
        } else {
            None
        }
    }

    /// Get interface from constant pool at given index.
    pub fn interface(&self, index: u16) -> Option<&crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self
            .interfaces
            .get(index as usize)
            .and_then(|interface_index| self.constant_pool.get(*interface_index))
        {
            Some(class)
        } else {
            None
        }
    }

    /// Get interface from constant pool at given index.
    pub fn interface_mut(&mut self, index: u16) -> Option<&mut crate::asm::node::constant::Class> {
        if let Some(Constant::Class(class)) = self
            .interfaces
            .get(index as usize)
            .and_then(|interface_index| self.constant_pool.get_mut(*interface_index))
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

impl<CPV, TCCV, SCCV, ICV, MV, FV, CV> Visitable<CV> for Class
where
    CPV: ConstantVisitor,
    TCCV: ConstantVisitor,
    SCCV: ConstantVisitor,
    ICV: ConstantVisitor,
    MV: MethodVisitor,
    FV: FieldVisitor,
    CV: ClassVisitor<CPV = CPV, TCCV = TCCV, SCCV = SCCV, ICV = ICV, MV = MV, FV = FV>,
{
    fn visit(&mut self, visitor: &mut CV) {
        visitor.visit_version(&mut self.java_version);

        visitor.visit_constant_pool(&self.constant_pool);

        for (index, constant) in self.constant_pool.iter() {
            visitor.visit_constant(index, constant);
        }

        visitor.visit_access_flags(&mut self.access_flags);

        let mut this_class_visitor = visitor.visit_this_class();

        if let Some(this_class) = self.this_class_mut() {
            this_class.visit(&mut this_class_visitor)
        }

        let mut super_class_visitor = visitor.visit_super_class();

        if let Some(super_class) = self.super_class_mut() {
            super_class.visit(&mut super_class_visitor);
        }

        visitor.visit_interfaces(&self.interfaces);

        if self.interfaces.len() != self.interfaces_count as usize {
            self.interfaces_count = self.interfaces.len() as u16;
        }

        for index in 0..self.interfaces.len() {
            let mut interface_visitor = visitor.visit_interface();

            if let Some(interface_constant) = self.interface_mut(index as u16) {
                interface_constant.visit(&mut interface_visitor);
            }
        }

        visitor.visit_fields(&mut self.fields);

        if self.fields.len() != self.fields_count as usize {
            self.fields_count = self.fields.len() as u16;
        }

        for field in &mut self.fields {
            let name = field
                .name(&self.constant_pool)
                .and_then(|utf8| utf8.string().ok());
            let descriptor = field
                .descriptor(&self.constant_pool)
                .and_then(|utf8| utf8.string().ok());

            if let (Some(name), Some(descriptor)) = (name, descriptor) {
                let mut field_visitor =
                    visitor.visit_field(&mut field.access_flags, &name, &descriptor);

                field.visit(&mut field_visitor);
            }
        }

        visitor.visit_methods(&mut self.methods);

        if self.methods.len() != self.methods_count as usize {
            self.methods_count = self.methods.len() as u16;
        }

        for method in &mut self.methods {
            let name = method
                .name(&self.constant_pool)
                .and_then(|utf8| utf8.string().ok());
            let descriptor = method
                .descriptor(&self.constant_pool)
                .and_then(|utf8| utf8.string().ok());

            if let (Some(name), Some(descriptor)) = (name, descriptor) {
                let mut method_visitor =
                    visitor.visit_method(&mut method.access_flags, &name, &descriptor);

                method.visit(&mut method_visitor);
            }
        }
    }
}

/// Represents java version documented in specification (combines `major_version` and `minor_version`).
///
/// See [Table 4.1-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=83).
#[repr(u32)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    TryFromPrimitive,
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
