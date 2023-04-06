// Tag values for the constant pool entries (using the same order as in the JVMS).

use crate::asm::symbol::ConstantTag::*;

#[repr(u8)]
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

pub(crate) enum Symbol {
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        bytes: [u8; 4],
    },
    Float {
        bytes: [u8; 4],
    },
    Long {
        high_bytes: [u8; 4],
        low_bytes: [u8; 4],
    },
    Double {
        high_bytes: [u8; 4],
        low_bytes: [u8; 4],
    },
    NameAndType {
        name_index: u16,
        type_index: u16,
    },
    Utf8 {
        /* Due to rust's safety, len and bytes information are merged as Vec<u8> type */
        bytes: Vec<u8>,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

impl Symbol {
    pub const fn tag(&self) -> ConstantTag {
        match self {
            Symbol::Class { .. } => Class,
            Symbol::FieldRef { .. } => FieldRef,
            Symbol::MethodRef { .. } => MethodRef,
            Symbol::InterfaceMethodRef { .. } => InterfaceMethodRef,
            Symbol::String { .. } => String,
            Symbol::Integer { .. } => Integer,
            Symbol::Float { .. } => Float,
            Symbol::Long { .. } => Long,
            Symbol::Double { .. } => Double,
            Symbol::NameAndType { .. } => NameAndType,
            Symbol::Utf8 { .. } => Utf8,
            Symbol::MethodHandle { .. } => MethodHandle,
            Symbol::MethodType { .. } => MethodType,
            Symbol::Dynamic { .. } => Dynamic,
            Symbol::InvokeDynamic { .. } => InvokeDynamic,
            Symbol::Module { .. } => Module,
            Symbol::Package { .. } => Package,
        }
    }
}

pub struct SymbolTable {
    symbols: Vec<Symbol>,
}
