use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::node::constant::{
    Class, Constant, ConstantPool, FieldRef, InterfaceMethodRef, MethodRef,
};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

/// Represents a `ldc` instruction.
///
/// See [6.5.ldc](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=563).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ldc {
    pub index: u8,
}

impl Ldc {
    /// Get target constant loaded by `ldc` instruction from constant pool.
    pub fn constant<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get(self.index as u16)
    }
}

impl ConstantRearrangeable for Ldc {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_narrow_index(&mut self.index, rearrangements)
    }
}

/// Represents a `ldc_w` instruction.
///
/// See [6.5.ldc_w](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=566).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc_W {
    pub index: u16,
}

impl Ldc_W {
    /// Get target constant loaded by `ldc_w` instruction from constant pool.
    pub fn constant<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get(self.index)
    }
}

impl ConstantRearrangeable for Ldc_W {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

/// Represents a `ldc2_w` instruction.
///
/// See [6.5.ldc2_w](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=568).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc2_W {
    pub index: u16,
}

impl Ldc2_W {
    /// Get target constant loaded by `ldc2_w` instruction from constant pool.
    pub fn constant<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get(self.index)
    }
}

impl ConstantRearrangeable for Ldc2_W {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

/// Represents a `getstatic` instruction.
///
/// See [6.5.getstatic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=491)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetStatic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl GetStatic {
    /// Get target field's reference loaded by `getstatic` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
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

/// Represents a `putstatic` instruction.
///
/// See [6.5.putstatic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=602)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PutStatic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl PutStatic {
    /// Get target field's reference loaded by `putstatic` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
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

/// Represents a `getfield` instruction.
///
/// See [6.5.getfield](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=490)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetField {
    pub index: u16,
}

//noinspection DuplicatedCode
impl GetField {
    /// Get target field's reference loaded by `getfield` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
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

/// Represents a `putfield` instruction.
///
/// See [6.5.putfield](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=600)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PutField {
    pub index: u16,
}

//noinspection DuplicatedCode
impl PutField {
    /// Get target field's reference loaded by `putfield` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
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

/// Represents a `invokevirtual` instruction.
///
/// See [6.5.invokevirtual](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=535)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeVirtual {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeVirtual {
    /// Get target method's reference invoked by `invokevirtual` instruction from constant pool.
    pub fn method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodRef> {
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

/// Represents a `invokespecial` instruction.
///
/// See [6.5.invokespecial](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=527)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeSpecial {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeSpecial {
    /// Get target method's reference invoked by `invokespecial` instruction from constant pool.
    pub fn method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodRef> {
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

/// Represents a `invokestatic` instruction.
///
/// See [6.5.invokestatic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=532)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeStatic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeStatic {
    /// Get target method's reference invoked by `invokestatic` instruction from constant pool.
    pub fn method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodRef> {
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

/// Represents a `invokeinterface` instruction.
///
/// See [6.5.invokeinterface](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=523)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeInterface {
    pub index: u16,
    pub count: u8,
}

//noinspection DuplicatedCode
impl InvokeInterface {
    /// Get target interface method's reference invoked by `invokeinterface` instruction from constant pool.
    pub fn interface_method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool InterfaceMethodRef> {
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

/// Represents a `invokedynamic` instruction.
///
/// See [6.5.invokedynamic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=521)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InvokeDynamic {
    /// Get target constant invoke dynamic invoked by `invokeinterface` instruction from constant pool.
    pub fn invoke_dynamic<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool crate::asm::node::constant::InvokeDynamic> {
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

/// Represents a `new` instruction.
///
/// See [6.5.new](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=593)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct New {
    pub index: u16,
}

//noinspection DuplicatedCode
impl New {
    /// Get target class created by `new` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
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

/// Represents a `anewarray` instruction.
///
/// See [6.5.anewarray](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=542)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ANewArray {
    pub index: u16,
}

//noinspection DuplicatedCode
impl ANewArray {
    /// Get target class' based class created by `anewarray` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
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

/// Represents a `checkcast` instruction.
///
/// See [6.5.checkcast](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=437)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CheckCast {
    pub index: u16,
}

//noinspection DuplicatedCode
impl CheckCast {
    /// Get target cast class checked by `checkcast` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
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

/// Represents a `instanceof` instruction.
///
/// See [6.5.instanceof](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=519)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InstanceOf {
    pub index: u16,
}

//noinspection DuplicatedCode
impl InstanceOf {
    /// Get target class checked by `instanceof` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
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
/// Represents a `wide` instruction.
///
/// See [6.5.wide](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=612)
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

/// Represents a `multianewarray` instruction.
///
/// See [6.5.multianewarray](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=591)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MultiANewArray {
    pub index: u16,
    pub dimensions: u8,
}

//noinspection DuplicatedCode
impl MultiANewArray {
    /// Get target class' based class created by `multianewarray` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
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
