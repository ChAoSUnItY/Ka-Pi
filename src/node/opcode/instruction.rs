use serde::{Deserialize, Serialize};

use crate::node::constant::{
    Class, Constant, ConstantPool, FieldRef, InterfaceMethodRef, MethodRef,
};
use crate::node::Node;

/// Represents a `ldc` instruction.
///
/// See [6.5.ldc](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=563).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ldc {
    pub index: Node<u8>,
}

impl Ldc {
    /// Get target constant loaded by `ldc` instruction from constant pool.
    pub fn constant<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get_constant(*self.index as u16)
    }
}

/// Represents a `ldc_w` instruction.
///
/// See [6.5.ldc_w](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=566).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc_W {
    pub index: Node<u16>,
}

impl Ldc_W {
    /// Get target constant loaded by `ldc_w` instruction from constant pool.
    pub fn constant<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get_constant(*self.index)
    }
}

/// Represents a `ldc2_w` instruction.
///
/// See [6.5.ldc2_w](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=568).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc2_W {
    pub index: Node<u16>,
}

impl Ldc2_W {
    /// Get target constant loaded by `ldc2_w` instruction from constant pool.
    pub fn constant<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Constant> {
        constant_pool.get_constant(*self.index)
    }
}

/// Represents a `getstatic` instruction.
///
/// See [6.5.getstatic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=491)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetStatic {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl GetStatic {
    /// Get target field's reference loaded by `getstatic` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
        constant_pool.get_field_ref(*self.index)
    }
}

/// Represents a `putstatic` instruction.
///
/// See [6.5.putstatic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=602)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PutStatic {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl PutStatic {
    /// Get target field's reference loaded by `putstatic` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
        constant_pool.get_field_ref(*self.index)
    }
}

/// Represents a `getfield` instruction.
///
/// See [6.5.getfield](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=490)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetField {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl GetField {
    /// Get target field's reference loaded by `getfield` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
        constant_pool.get_field_ref(*self.index)
    }
}

/// Represents a `putfield` instruction.
///
/// See [6.5.putfield](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=600)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PutField {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl PutField {
    /// Get target field's reference loaded by `putfield` instruction from constant pool.
    pub fn field_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool FieldRef> {
        constant_pool.get_field_ref(*self.index)
    }
}

/// Represents a `invokevirtual` instruction.
///
/// See [6.5.invokevirtual](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=535)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeVirtual {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl InvokeVirtual {
    /// Get target method's reference invoked by `invokevirtual` instruction from constant pool.
    pub fn method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodRef> {
        constant_pool.get_method_ref(*self.index)
    }
}

/// Represents a `invokespecial` instruction.
///
/// See [6.5.invokespecial](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=527)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeSpecial {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl InvokeSpecial {
    /// Get target method's reference invoked by `invokespecial` instruction from constant pool.
    pub fn method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodRef> {
        constant_pool.get_method_ref(*self.index)
    }
}

/// Represents a `invokestatic` instruction.
///
/// See [6.5.invokestatic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=532)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeStatic {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl InvokeStatic {
    /// Get target method's reference invoked by `invokestatic` instruction from constant pool.
    pub fn method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool MethodRef> {
        constant_pool.get_method_ref(*self.index)
    }
}

/// Represents a `invokeinterface` instruction.
///
/// See [6.5.invokeinterface](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=523)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeInterface {
    pub index: Node<u16>,
    pub count: Node<u8>,
}

//noinspection DuplicatedCode
impl InvokeInterface {
    /// Get target interface method's reference invoked by `invokeinterface` instruction from constant pool.
    pub fn interface_method_ref<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool InterfaceMethodRef> {
        constant_pool.get_interface_method_ref(*self.index)
    }
}

/// Represents a `invokedynamic` instruction.
///
/// See [6.5.invokedynamic](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=521)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl InvokeDynamic {
    /// Get target constant invoke dynamic invoked by `invokeinterface` instruction from constant pool.
    pub fn invoke_dynamic<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool crate::node::constant::InvokeDynamic> {
        constant_pool.get_invoke_dynamic(*self.index)
    }
}

/// Represents a `new` instruction.
///
/// See [6.5.new](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=593)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct New {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl New {
    /// Get target class created by `new` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.index)
    }
}

/// Represents a `anewarray` instruction.
///
/// See [6.5.anewarray](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=542)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ANewArray {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl ANewArray {
    /// Get target class' based class created by `anewarray` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.index)
    }
}

/// Represents a `checkcast` instruction.
///
/// See [6.5.checkcast](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=437)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CheckCast {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl CheckCast {
    /// Get target cast class checked by `checkcast` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.index)
    }
}

/// Represents a `instanceof` instruction.
///
/// See [6.5.instanceof](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=519)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InstanceOf {
    pub index: Node<u16>,
}

//noinspection DuplicatedCode
impl InstanceOf {
    /// Get target class checked by `instanceof` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.index)
    }
}

//noinspection SpellCheckingInspection
/// Represents a `wide` instruction.
///
/// See [6.5.wide](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=612)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Wide {
    ILOAD(Node<u16>),
    FLOAD(Node<u16>),
    ALOAD(Node<u16>),
    LLOAD(Node<u16>),
    DLOAD(Node<u16>),
    ISTORE(Node<u16>),
    FSTORE(Node<u16>),
    ASTORE(Node<u16>),
    LSTORE(Node<u16>),
    DSTORE(Node<u16>),
    RET(Node<u16>),
    IINC(Node<u16>, Node<u16>),
}

/// Represents a `multianewarray` instruction.
///
/// See [6.5.multianewarray](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=591)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MultiANewArray {
    pub index: Node<u16>,
    pub dimensions: Node<u8>,
}

//noinspection DuplicatedCode
impl MultiANewArray {
    /// Get target class' based class created by `multianewarray` instruction from constant pool.
    pub fn class<'instruction, 'constant_pool: 'instruction>(
        &'instruction self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Class> {
        constant_pool.get_class(*self.index)
    }
}
