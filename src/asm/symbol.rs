// Tag values for the constant pool entries (using the same order as in the JVMS).

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::asm::attribute::{Attribute, BootstrapMethod, ConstantValue};
use crate::asm::handle::Handle;
use crate::asm::opcodes::{ConstantObject, RefKind};

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) enum Constant {
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

impl Constant {
    pub const fn tag(&self) -> ConstantTag {
        match self {
            Constant::Class { .. } => ConstantTag::Class,
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

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct SymbolTable {
    pub(crate) constants: IndexSet<Constant>,
    pub(crate) attributes: IndexSet<Attribute>,
    // Symbol caching fields
    // Attribute caching fields
    #[serde(skip_serializing)]
    bootstrap_methods: IndexSet<BootstrapMethod>,
}

impl SymbolTable {
    #[inline]
    fn constants_len(&self) -> u16 {
        self.constants.len() as u16
    }

    #[inline]
    fn attributes_len(&self) -> u16 {
        self.attributes.len() as u16
    }

    #[inline]
    fn bootstrap_methods_len(&self) -> u16 {
        self.bootstrap_methods.len() as u16
    }

    #[inline]
    fn insert_constant(&mut self, constant: Constant) -> u16 {
        if let Some(index) = self.constants.get_index_of(&constant) {
            index as u16 + 1
        } else {
            self.constants.insert(constant);
            self.constants_len()
        }
    }

    #[inline]
    fn insert_attr(&mut self, attr: Attribute) -> u16 {
        if let Some(index) = self.attributes.get_index_of(&attr) {
            index as u16
        } else {
            self.attributes.insert(attr);
            self.attributes_len() - 1
        }
    }

    fn insert_bootstrap_method(&mut self, boostrap_method: BootstrapMethod) -> u16 {
        if let Some(index) = self.bootstrap_methods.get_index_of(&boostrap_method) {
            index as u16
        } else {
            self.bootstrap_methods.insert(boostrap_method);
            self.bootstrap_methods.len() as u16 - 1
        }
    }

    pub(crate) fn add_utf8(&mut self, string: &str) -> u16 {
        let constant = Constant::Utf8 {
            data: string.to_owned(),
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_class(&mut self, class: &str) -> u16 {
        let name_index = self.add_utf8(class);
        let constant = Constant::Class { name_index };

        self.insert_constant(constant)
    }

    pub(crate) fn add_string(&mut self, string: &str) -> u16 {
        let string_index = self.add_utf8(string);
        let constant = Constant::String { string_index };

        self.insert_constant(constant)
    }

    pub(crate) fn add_integer(&mut self, integer: i32) -> u16 {
        let constant = Constant::Integer {
            bytes: integer.to_be_bytes(),
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_float(&mut self, float: f32) -> u16 {
        let constant = Constant::Float {
            bytes: float.to_be_bytes(),
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_long(&mut self, long: i64) -> u16 {
        let bytes = long.to_be_bytes();
        let (high_bytes, low_bytes) = bytes.split_at(4);
        let constant = Constant::Long {
            high_bytes: high_bytes.try_into().unwrap(),
            low_bytes: low_bytes.try_into().unwrap(),
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_double(&mut self, double: f64) -> u16 {
        let bytes = double.to_be_bytes();
        let (high_bytes, low_bytes) = bytes.split_at(4);
        let constant = Constant::Double {
            high_bytes: high_bytes.try_into().unwrap(),
            low_bytes: low_bytes.try_into().unwrap(),
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_field_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);
        let constant = Constant::FieldRef {
            class_index,
            name_and_type_index,
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_method_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);
        let constant = Constant::MethodRef {
            class_index,
            name_and_type_index,
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_interface_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);
        let constant = Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_name_and_type(&mut self, name: &str, typ: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let type_index = self.add_utf8(typ);
        let constant = Constant::NameAndType {
            name_index,
            type_index,
        };

        self.insert_constant(constant)
    }

    pub(crate) fn add_method_handle(
        &mut self,
        reference_kind: &RefKind,
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
        let constant = Constant::MethodHandle {
            reference_kind: *reference_kind as u8,
            reference_index,
        };

        self.insert_constant(constant)
    }

    // TODO: fn add_dynamic()
    // TODO: fn add_invoke_dynamic()

    pub(crate) fn add_module(&mut self, name: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let constant = Constant::Module { name_index };

        self.insert_constant(constant)
    }

    pub(crate) fn add_package(&mut self, name: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let constant = Constant::Package { name_index };

        self.insert_constant(constant)
    }

    pub(crate) fn add_constant_attribute<CV>(&mut self, constant_value: CV) -> u16
    where
        CV: Into<ConstantValue>,
    {
        let constant_value = constant_value.into();
        let constant_value_index = match &constant_value {
            ConstantValue::Int(val) => self.add_integer(*val),
            ConstantValue::Float(val) => self.add_float(*val),
            ConstantValue::Long(val) => self.add_long(*val),
            ConstantValue::Double(val) => self.add_double(*val),
            ConstantValue::String(val) => self.add_string(val),
        };
        let attribute = Attribute::ConstantValue {
            constant_value_index,
        };

        self.insert_attr(attribute)
    }

    pub(crate) fn add_constant_object(&mut self, constant_object: &ConstantObject) -> u16 {
        match constant_object {
            ConstantObject::String(val) => self.add_string(val),
            ConstantObject::Int(val) => self.add_integer(*val),
            ConstantObject::Float(val) => self.add_float(*val),
            ConstantObject::Long(val) => self.add_long(*val),
            ConstantObject::Double(val) => self.add_double(*val),
            ConstantObject::Class(val) => self.add_class(val),
            ConstantObject::MethodHandle(ref_kind, class, name, descriptor) => {
                self.add_method_handle(ref_kind, class, name, descriptor)
            }
            ConstantObject::MethodType(val) => self.add_utf8(val),
            ConstantObject::ConstantDynamic(name, descriptor, handle, arguments) => {
                self.add_constant_dynamic(name, descriptor, handle, arguments)
            }
        }
    }

    pub(crate) fn add_constant_dynamic(
        &mut self,
        name: &str,
        descriptor: &str,
        handle: &Handle,
        arguments: &Vec<ConstantObject>,
    ) -> u16 {
        let boostrap_method_index = self.add_bootstrap_method(handle, arguments);
        let name_and_type_index = self.add_name_and_type(name, descriptor);
        let constant = Constant::Dynamic {
            bootstrap_method_attr_index: boostrap_method_index,
            name_and_type_index,
        };

        self.insert_constant(constant)
    }

    fn add_bootstrap_method(&mut self, handle: &Handle, arguments: &Vec<ConstantObject>) -> u16 {
        let bootstrap_arguments_indices = arguments
            .iter()
            .map(|constant_object| self.add_constant_object(constant_object))
            .collect::<Vec<_>>();
        let Handle {
            tag,
            owner,
            name,
            descriptor,
        } = handle;
        let boostrap_method_handle = self.add_method_handle(tag, owner, name, descriptor);
        let boostrap_method =
            BootstrapMethod::new(boostrap_method_handle, bootstrap_arguments_indices);

        self.insert_bootstrap_method(boostrap_method)
    }
}

#[cfg(test)]
mod test {
    use crate::asm::symbol::SymbolTable;

    #[test]
    fn test_symbol_table_utf8() {
        let mut table = SymbolTable::default();

        let index = table.add_utf8("ClassName");
        let cached_index = table.add_utf8("ClassName");

        assert_eq!(index, 1);
        assert_eq!(index, cached_index);
        assert_eq!(table.constants_len(), 1);
    }

    #[test]
    fn test_symbol_table_name_and_type() {
        let mut table = SymbolTable::default();

        let index = table.add_name_and_type("clazz", "java.lang.Class");
        let cached_index = table.add_name_and_type("clazz", "java.lang.Class");

        // NameAndType is registered with 3 entries (utf8 & utf8 -> name_and_type),
        // therefore the index of NameAndType should be 3
        assert_eq!(index, 3);
        assert_eq!(index, cached_index);
        assert_eq!(table.constants_len(), 3);
    }

    #[test]
    fn test_symbol_table_long_double() {
        let mut table = SymbolTable::default();

        let index = table.add_long(i64::MAX);
        let cached_index = table.add_long(i64::MAX);

        assert_eq!(index, 1);
        assert_eq!(index, cached_index);
        assert_eq!(table.constants_len(), 1);

        let index = table.add_double(f64::MAX);
        let cached_index = table.add_double(f64::MAX);

        assert_eq!(index, 2);
        assert_eq!(index, cached_index);
        assert_eq!(table.constants_len(), 2);
    }

    // More tests?
}
