// Tag values for the constant pool entries (using the same order as in the JVMS).

use crate::asm::byte_vec::ByteVec;

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

pub(crate) trait Symbol {
    fn index(&self) -> usize;
    fn tag(&self) -> u8;
    fn owner(&self) -> &str;
    fn name(&self) -> &str;
    fn value(&self) -> &str;
    fn data(&self) -> i64;
    fn info(&self) -> i32;

    fn new(
        index: usize,
        tag: u8,
        owner: String,
        name: String,
        value: String,
        data: i64,
        info: i32,
    ) -> Self;
}

pub(crate) struct SymbolTable<BV> where BV: ByteVec {
    major_version: u16,
    class_name: String,
    constant_pool: BV,
    entry_count: usize,
}

struct Entry {
    index: usize,
    tag: u8,
    owner: String,
    name: String,
    value: String,
    data: i64,
    info: i32,
}

impl Symbol for Entry {
    fn index(&self) -> usize {
        self.index
    }

    fn tag(&self) -> u8 {
        self.tag
    }

    fn owner(&self) -> &str {
        &self.owner
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn data(&self) -> i64 {
        self.data
    }

    fn info(&self) -> i32 {
        todo!()
    }

    fn new(index: usize, tag: u8, owner: String, name: String, value: String, data: i64, info: i32) -> Self {
        todo!()
    }
}
