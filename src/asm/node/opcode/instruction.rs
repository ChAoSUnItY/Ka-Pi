use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::node::constant::{
    Class, Constant, ConstantPool, FieldRef, InterfaceMethodRef, MethodRef,
};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ldc {
    pub index: u8,
}

impl Ldc {
    pub fn constant<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Constant> {
        constant_pool.get(self.index as u16)
    }
}

impl ConstantRearrangeable for Ldc {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_narrow_index(&mut self.index, rearrangements)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc_W {
    pub index: u16,
}

impl Ldc_W {
    pub fn constant<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Constant> {
        constant_pool.get(self.index)
    }
}

impl ConstantRearrangeable for Ldc_W {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc2_W {
    pub index: u16,
}

impl Ldc2_W {
    pub fn constant<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Constant> {
        constant_pool.get(self.index)
    }
}

impl ConstantRearrangeable for Ldc2_W {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetStatic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl GetStatic {
    pub fn field_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant FieldRef> {
        if let Some(Constant::FieldRef(field_ref)) = constant_pool.get(self.index) {
            Some(field_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for GetStatic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PutStatic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl PutStatic {
    pub fn field_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant FieldRef> {
        if let Some(Constant::FieldRef(field_ref)) = constant_pool.get(self.index) {
            Some(field_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for PutStatic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetField {
    pub index: u16,
}

//noinspection DuplicatedCode
impl GetField {
    pub fn field_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant FieldRef> {
        if let Some(Constant::FieldRef(field_ref)) = constant_pool.get(self.index) {
            Some(field_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for GetField {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PutField {
    pub index: u16,
}

//noinspection DuplicatedCode
impl PutField {
    pub fn field_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant FieldRef> {
        if let Some(Constant::FieldRef(field_ref)) = constant_pool.get(self.index) {
            Some(field_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for PutField {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeVirtual {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeVirtual {
    pub fn method_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant MethodRef> {
        if let Some(Constant::MethodRef(method_ref)) = constant_pool.get(self.index) {
            Some(method_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InvokeVirtual {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeSpecial {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeSpecial {
    pub fn method_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant MethodRef> {
        if let Some(Constant::MethodRef(method_ref)) = constant_pool.get(self.index) {
            Some(method_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InvokeSpecial {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeStatic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeStatic {
    pub fn method_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant MethodRef> {
        if let Some(Constant::MethodRef(method_ref)) = constant_pool.get(self.index) {
            Some(method_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InvokeStatic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeInterface {
    pub index: u16,
    pub count: u8,
}

//noinspection DuplicatedCode
impl InvokeInterface {
    pub fn interface_method_ref<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant InterfaceMethodRef> {
        if let Some(Constant::InterfaceMethodRef(interface_method_ref)) =
            constant_pool.get(self.index)
        {
            Some(interface_method_ref)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InvokeInterface {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeDynamic {
    pub fn invoke_dynamic<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant crate::asm::node::constant::InvokeDynamic> {
        if let Some(Constant::InvokeDynamic(invoke_dynamic)) = constant_pool.get(self.index) {
            Some(invoke_dynamic)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InvokeDynamic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct New {
    pub index: u16,
}

//noinspection DuplicatedCode
impl New {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for New {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ANewArray {
    pub index: u16,
}

//noinspection DuplicatedCode
impl ANewArray {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for ANewArray {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CheckCast {
    pub index: u16,
}

//noinspection DuplicatedCode
impl CheckCast {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for CheckCast {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InstanceOf {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InstanceOf {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for InstanceOf {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Wide {
    ILOAD(u16),
    FLOAD(u16),
    ALOAD(u16),
    LLOAD(u16),
    DLOAD(u16),
    ISTORE(u16),
    FSTORE(u16),
    ASTORE(u16),
    LSTORE(u16),
    DSTORE(u16),
    RET(u16),
    IINC(u16, i16),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MultiANewArray {
    pub index: u16,
    pub dimensions: u8,
}

//noinspection DuplicatedCode
impl MultiANewArray {
    pub fn class<'constant, 'constant_pool: 'constant>(
        &'constant self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant Class> {
        if let Some(Constant::Class(class)) = constant_pool.get(self.index) {
            Some(class)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for MultiANewArray {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}
