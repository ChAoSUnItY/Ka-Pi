// Tag values for the constant pool entries (using the same order as in the JVMS).

use crate::asm::symbol::ConstantTag::*;
use crate::error::KapiError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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

/// Top level constant tag redefinitions for low level usage

/** The tag value of CONSTANT_Class_info JVMS structures. */
pub(crate) const CONSTANT_CLASS_TAG: u8 = 7;

/** The tag value of CONSTANT_Fieldref_info JVMS structures. */
pub(crate) const CONSTANT_FIELDREF_TAG: u8 = 9;

/** The tag value of CONSTANT_Methodref_info JVMS structures. */
pub(crate) const CONSTANT_METHODREF_TAG: u8 = 10;

/** The tag value of CONSTANT_InterfaceMethodref_info JVMS structures. */
pub(crate) const CONSTANT_INTERFACE_METHODREF_TAG: u8 = 11;

/** The tag value of CONSTANT_String_info JVMS structures. */
pub(crate) const CONSTANT_STRING_TAG: u8 = 8;

/** The tag value of CONSTANT_Integer_info JVMS structures. */
pub(crate) const CONSTANT_INTEGER_TAG: u8 = 3;

/** The tag value of CONSTANT_Float_info JVMS structures. */
pub(crate) const CONSTANT_FLOAT_TAG: u8 = 4;

/** The tag value of CONSTANT_Long_info JVMS structures. */
pub(crate) const CONSTANT_LONG_TAG: u8 = 5;

/** The tag value of CONSTANT_Double_info JVMS structures. */
pub(crate) const CONSTANT_DOUBLE_TAG: u8 = 6;

/** The tag value of CONSTANT_NameAndType_info JVMS structures. */
pub(crate) const CONSTANT_NAME_AND_TYPE_TAG: u8 = 12;

/** The tag value of CONSTANT_Utf8_info JVMS structures. */
pub(crate) const CONSTANT_UTF8_TAG: u8 = 1;

/** The tag value of CONSTANT_MethodHandle_info JVMS structures. */
pub(crate) const CONSTANT_METHOD_HANDLE_TAG: u8 = 15;

/** The tag value of CONSTANT_MethodType_info JVMS structures. */
pub(crate) const CONSTANT_METHOD_TYPE_TAG: u8 = 16;

/** The tag value of CONSTANT_Dynamic_info JVMS structures. */
pub(crate) const CONSTANT_DYNAMIC_TAG: u8 = 17;

/** The tag value of CONSTANT_InvokeDynamic_info JVMS structures. */
pub(crate) const CONSTANT_INVOKE_DYNAMIC_TAG: u8 = 18;

/** The tag value of CONSTANT_Module_info JVMS structures. */
pub(crate) const CONSTANT_MODULE_TAG: u8 = 19;

/** The tag value of CONSTANT_Package_info JVMS structures. */
pub(crate) const CONSTANT_PACKAGE_TAG: u8 = 20;

// Tag values for the BootstrapMethods attribute entries (ASM specific tag).

/** The tag value of the BootstrapMethods attribute entries. */
pub(crate) const BOOTSTRAP_METHOD_TAG: u8 = 64;

// Tag values for the type table entries (ASM specific tags).

/** The tag value of a normal type entry in the (ASM specific) type table of a class. */
pub(crate) const TYPE_TAG: u8 = 128;

/**
 * The tag value of an {@link Frame#ITEM_UNINITIALIZED} type entry in the type table of a class.
 */
pub(crate) const UNINITIALIZED_TYPE_TAG: u8 = 129;

/** The tag value of a merged type entry in the (ASM specific) type table of a class. */
pub(crate) const MERGED_TYPE_TAG: u8 = 130;

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
