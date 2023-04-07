// Tag values for the constant pool entries (using the same order as in the JVMS).

use std::collections::HashMap;

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
        /*  Implementation note: This has been merged into a single String type for later table
         *   implementation usage.
         */
        data: String,
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
            Symbol::Class { .. } => ConstantTag::Class,
            Symbol::FieldRef { .. } => ConstantTag::FieldRef,
            Symbol::MethodRef { .. } => ConstantTag::MethodRef,
            Symbol::InterfaceMethodRef { .. } => ConstantTag::InterfaceMethodRef,
            Symbol::String { .. } => ConstantTag::String,
            Symbol::Integer { .. } => ConstantTag::Integer,
            Symbol::Float { .. } => ConstantTag::Float,
            Symbol::Long { .. } => ConstantTag::Long,
            Symbol::Double { .. } => ConstantTag::Double,
            Symbol::NameAndType { .. } => ConstantTag::NameAndType,
            Symbol::Utf8 { .. } => ConstantTag::Utf8,
            Symbol::MethodHandle { .. } => ConstantTag::MethodHandle,
            Symbol::MethodType { .. } => ConstantTag::MethodType,
            Symbol::Dynamic { .. } => ConstantTag::Dynamic,
            Symbol::InvokeDynamic { .. } => ConstantTag::InvokeDynamic,
            Symbol::Module { .. } => ConstantTag::Module,
            Symbol::Package { .. } => ConstantTag::Package,
        }
    }
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
    utf8_cache: HashMap<String, usize>,
    name_and_type_cache: HashMap<(String, String), usize>,
}

impl SymbolTable {
    fn len(&self) -> usize {
        self.symbols.len()
    }

    fn add_utf8(&mut self, string: &str) -> usize {
        if let Some(registered_string_index) = self.utf8_cache.get(string) {
            *registered_string_index
        } else {
            let index = self.symbols.len();
            self.symbols.push(Symbol::Utf8 {
                data: string.to_owned(),
            });
            self.utf8_cache.insert(string.to_owned(), index);
            index
        }
    }

    fn add_name_and_type(&mut self, name: &str, typ: &str) -> usize {
        if let Some(registered_name_and_type_index) = self
            .name_and_type_cache
            .get(&(name.to_owned(), typ.to_owned()))
        {
            *registered_name_and_type_index
        } else {
            let name_index = self.add_utf8(name) as u16;
            let type_index = self.add_utf8(typ) as u16;
            let name_and_type_index = self.symbols.len();
            self.symbols.push(Symbol::NameAndType {
                name_index,
                type_index,
            });
            self.name_and_type_cache
                .insert((name.to_owned(), typ.to_owned()), name_and_type_index);
            name_and_type_index
        }
    }
}

#[cfg(test)]
mod test {
    use crate::asm::symbol::SymbolTable;

    #[test]
    pub fn test_symbol_table_utf8() {
        let mut table = SymbolTable::default();

        let index = table.add_utf8("ClassName");
        let cached_index = table.add_utf8("ClassName");

        assert_eq!(index, cached_index);
        assert_eq!(table.len(), 1);
    }

    #[test]
    pub fn test_symbol_table_name_and_type() {
        let mut table = SymbolTable::default();

        let index = table.add_name_and_type("clazz", "java.lang.Class");
        let cached_index = table.add_name_and_type("clazz", "java.lang.Class");

        assert_eq!(index, cached_index);
        assert_eq!(table.len(), 3);
    }
}
