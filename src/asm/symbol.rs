// Tag values for the constant pool entries (using the same order as in the JVMS).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::opcodes::RefKind;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Default, Serialize, Deserialize)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
    #[serde(skip_serializing)]
    utf8_cache: HashMap<String, u16>,
    #[serde(skip_serializing)]
    single_index_cache: HashMap<u16, u16>,
    #[serde(skip_serializing)]
    double_index_cache: HashMap<(u16, u16), u16>,
    #[serde(skip_serializing)]
    integer_cache: HashMap<i32, u16>,
    #[serde(skip_serializing)]
    float_cache: HashMap<[u8; 4], u16>,
    #[serde(skip_serializing)]
    long_cache: HashMap<i64, u16>,
    #[serde(skip_serializing)]
    double_cache: HashMap<[u8; 8], u16>,
    #[serde(skip_serializing)]
    method_handle_cache: HashMap<(u8, u16), u16>,
}

impl SymbolTable {
    fn len(&self) -> u16 {
        self.symbols.len() as u16
    }

    fn add_utf8(&mut self, string: &str) -> u16 {
        if let Some(index) = self.utf8_cache.get(string) {
            *index
        } else {
            self.symbols.push(Symbol::Utf8 {
                data: string.to_owned(),
            });
            self.utf8_cache
                .insert(string.to_owned(), self.len())
                .unwrap()
        }
    }

    fn add_class(&mut self, class: &str) -> u16 {
        let name_index = self.add_utf8(class);

        if let Some(index) = self.single_index_cache.get(&name_index) {
            *index
        } else {
            self.symbols.push(Symbol::Class { name_index });
            self.single_index_cache
                .insert(name_index, self.len())
                .unwrap()
        }
    }

    fn add_string(&mut self, string: &str) -> u16 {
        let string_index = self.add_utf8(string);

        if let Some(index) = self.single_index_cache.get(&string_index) {
            *index
        } else {
            self.symbols.push(Symbol::String { string_index });
            self.single_index_cache
                .insert(string_index, self.len())
                .unwrap()
        }
    }

    fn add_integer(&mut self, integer: i32) -> u16 {
        if let Some(index) = self.integer_cache.get(&integer) {
            *index
        } else {
            self.symbols.push(Symbol::Integer {
                bytes: integer.to_be_bytes(),
            });
            self.integer_cache.insert(integer, self.len()).unwrap()
        }
    }

    fn add_float(&mut self, float: f32) -> u16 {
        let be_bytes = float.to_be_bytes();

        if let Some(index) = self.float_cache.get(&be_bytes) {
            *index
        } else {
            self.symbols.push(Symbol::Float { bytes: be_bytes });
            self.float_cache.insert(be_bytes, self.len()).unwrap()
        }
    }

    fn add_long(&mut self, long: i64) -> u16 {
        if let Some(index) = self.long_cache.get(&long) {
            *index
        } else {
            let [high_bytes, low_bytes] =
                unsafe { std::mem::transmute::<[u8; 8], [[u8; 4]; 2]>(long.to_be_bytes()) };
            self.symbols.push(Symbol::Long {
                high_bytes,
                low_bytes,
            });
            self.long_cache.insert(long, self.len()).unwrap()
        }
    }

    fn add_field_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);

        if let Some(index) = self
            .double_index_cache
            .get(&(class_index, name_and_type_index))
        {
            *index
        } else {
            self.symbols.push(Symbol::FieldRef {
                class_index,
                name_and_type_index,
            });
            self.double_index_cache
                .insert((class_index, name_and_type_index), self.len())
                .unwrap()
        }
    }

    fn add_method_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);

        if let Some(index) = self
            .double_index_cache
            .get(&(class_index, name_and_type_index))
        {
            *index
        } else {
            self.symbols.push(Symbol::MethodRef {
                class_index,
                name_and_type_index,
            });
            self.double_index_cache
                .insert((class_index, name_and_type_index), self.len())
                .unwrap()
        }
    }

    fn add_interface_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);

        if let Some(index) = self
            .double_index_cache
            .get(&(class_index, name_and_type_index))
        {
            *index
        } else {
            self.symbols.push(Symbol::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            });
            self.double_index_cache
                .insert((class_index, name_and_type_index), self.len())
                .unwrap()
        }
    }

    fn add_double(&mut self, double: f64) -> u16 {
        let be_bytes = double.to_be_bytes();

        if let Some(index) = self.double_cache.get(&be_bytes) {
            *index
        } else {
            let [high_bytes, low_bytes] =
                unsafe { std::mem::transmute::<[u8; 8], [[u8; 4]; 2]>(be_bytes) };

            self.symbols.push(Symbol::Double {
                high_bytes,
                low_bytes,
            });
            self.double_cache.insert(be_bytes, self.len()).unwrap()
        }
    }

    fn add_name_and_type(&mut self, name: &str, typ: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let type_index = self.add_utf8(typ);

        if let Some(index) = self.double_index_cache.get(&(name_index, type_index)) {
            *index
        } else {
            self.symbols.push(Symbol::NameAndType {
                name_index,
                type_index,
            });
            self.double_index_cache
                .insert((name_index, type_index), self.len())
                .unwrap()
        }
    }

    fn add_method_handle(
        &mut self,
        reference_kind: RefKind,
        class: &str,
        name: &str,
        typ: &str,
    ) -> u16 {
        let reference_index = match reference_kind {
            RefKind::GetField | RefKind::GetStatic | RefKind::PutField | RefKind::PutStatic => {
                self.add_field_ref(class, name, typ)
            }
            RefKind::InvokeVirtual | RefKind::NewInvokeSpecial => {
                self.add_method_ref(class, name, typ)
            }
            RefKind::InvokeStatic | RefKind::InvokeSpecial | RefKind::InvokeInterface => {
                self.add_interface_ref(class, name, typ)
            }
        };

        if let Some(index) = self
            .method_handle_cache
            .get(&(reference_kind as u8, reference_index))
        {
            *index
        } else {
            self.symbols.push(Symbol::MethodHandle {
                reference_kind: reference_kind as u8,
                reference_index,
            });
            self.method_handle_cache
                .insert((reference_kind as u8, reference_index), self.len())
                .unwrap()
        }
    }

    // TODO: fn add_dynamic()
    // TODO: fn add_invoke_dynamic()

    fn add_module(&mut self, name: &str) -> u16 {
        let name_index = self.add_utf8(name);

        if let Some(index) = self.single_index_cache.get(&name_index) {
            *index
        } else {
            self.symbols.push(Symbol::Module { name_index });
            self.single_index_cache
                .insert(name_index, self.len())
                .unwrap()
        }
    }

    fn add_package(&mut self, name: &str) -> u16 {
        let name_index = self.add_utf8(name);

        if let Some(index) = self.single_index_cache.get(&name_index) {
            *index
        } else {
            self.symbols.push(Symbol::Package { name_index });
            self.single_index_cache
                .insert(name_index, self.len())
                .unwrap()
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
    
    // More tests?
}
