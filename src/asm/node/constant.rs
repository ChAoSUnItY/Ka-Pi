use std::collections::BTreeMap;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstantPool {
    len: usize,
    entries: BTreeMap<usize, Constant>,
}

impl ConstantPool {
    pub(crate) fn add(&mut self, constant: Constant) {
        let is_2 = matches!(constant, Constant::Long { .. } | Constant::Double { .. });

        self.entries.insert(self.len, constant);

        if is_2 {
            self.len += 2;
        } else {
            self.len += 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<&Constant> {
        self.entries.get(&index)
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self {
            len: 1,
            entries: BTreeMap::default(),
        }
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum ConstantTag {
    /** The tag value of CONSTANT_Class_info JVMS structures. */
    Class = 7,
    /** The tag value of CONSTANT_Fieldref_info JVMS structures. */
    FieldRef = 9,
    /** The tag value of CONSTANT_Methodref_info JVMS structures. */
    MethodRef = 10,
    /** The tag value of CONSTANT_InterfaceMethodref_info JVMS structures. */
    InterfaceMethodRef = 11,
    /** The tag value of CONSTANT_String_info JVMS structures. */
    String = 8,
    /** The tag value of CONSTANT_Integer_info JVMS structures. */
    Integer = 3,
    /** The tag value of CONSTANT_Float_info JVMS structures. */
    Float = 4,
    /** The tag value of CONSTANT_Long_info JVMS structures. */
    Long = 5,
    /** The tag value of CONSTANT_Double_info JVMS structures. */
    Double = 6,
    /** The tag value of CONSTANT_NameAndType_info JVMS structures. */
    NameAndType = 12,
    /** The tag value of CONSTANT_Utf8_info JVMS structures. */
    Utf8 = 1,
    /** The tag value of CONSTANT_MethodHandle_info JVMS structures. */
    MethodHandle = 15,
    /** The tag value of CONSTANT_MethodType_info JVMS structures. */
    MethodType = 16,
    /** The tag value of CONSTANT_MethodType_info JVMS structures. */
    Dynamic = 17,
    /** The tag value of CONSTANT_Dynamic_info JVMS structures. */
    InvokeDynamic = 18,
    /** The tag value of CONSTANT_InvokeDynamic_info JVMS structures. */
    Module = 19,
    /** The tag value of CONSTANT_Module_info JVMS structures. */
    Package = 20,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Constant {
    Class(Class),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    String(String),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    NameAndType(NameAndType),
    Utf8(Utf8),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    Dynamic(Dynamic),
    InvokeDynamic(InvokeDynamic),
    Module(Module),
    Package(Package),
}

impl Constant {
    pub const fn tag(&self) -> ConstantTag {
        match self {
            Constant::Class(..) => ConstantTag::Class,
            Constant::FieldRef { .. } => ConstantTag::FieldRef,
            Constant::MethodRef { .. } => ConstantTag::MethodRef,
            Constant::InterfaceMethodRef { .. } => ConstantTag::InterfaceMethodRef,
            Constant::String { .. } => ConstantTag::String,
            Constant::Integer { .. } => ConstantTag::Integer,
            Constant::Float { .. } => ConstantTag::Float,
            Constant::Long { .. } => ConstantTag::Long,
            Constant::Double { .. } => ConstantTag::Double,
            Constant::NameAndType { .. } => ConstantTag::NameAndType,
            Constant::Utf8 { .. } => ConstantTag::Utf8,
            Constant::MethodHandle { .. } => ConstantTag::MethodHandle,
            Constant::MethodType { .. } => ConstantTag::MethodType,
            Constant::Dynamic { .. } => ConstantTag::Dynamic,
            Constant::InvokeDynamic { .. } => ConstantTag::InvokeDynamic,
            Constant::Module { .. } => ConstantTag::Module,
            Constant::Package { .. } => ConstantTag::Package,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Class {
    pub name_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FieldRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InterfaceMethodRef {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct String {
    pub string_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Integer {
    pub bytes: [u8; 4],
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Float {
    pub bytes: [u8; 4],
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Long {
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Double {
    pub high_bytes: [u8; 4],
    pub low_bytes: [u8; 4],
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NameAndType {
    pub name_index: u16,
    pub type_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Utf8 {
    /*  Implementation note: This has been merged into a single String type for later table
     *  implementation usage.
     */
    pub data: std::string::String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodType {
    pub descriptor_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Module {
    pub name_index: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Package {
    pub name_index: u16,
}
