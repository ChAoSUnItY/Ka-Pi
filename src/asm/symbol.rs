// Tag values for the constant pool entries (using the same order as in the JVMS).

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

use indexmap::IndexSet;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::asm::node::attribute::{constant_value, Attribute, BootstrapMethod, ConstantValue};
use crate::asm::node::constant::{
    Class, Constant, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef, Long,
    MethodHandle, MethodRef, Module, NameAndType, Package, Utf8,
};
use crate::asm::node::handle::Handle;
use crate::asm::node::opcode::{ConstantObject, RefKind};
use crate::asm::node::{constant, ConstantRearrangeable};

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

    /// Merges another [SymbolTable] into current one. This function eliminates other table's constants
    /// if constant is already existed in current one. Otherwise, clone other table's non-duplicated
    /// constants to current table. This function will also relocate the constant index for [Constant]s
    /// and [Attribute]s which depends on it.
    ///
    /// # Finalization
    /// Notice that this function is meant for finalize and optimize the other table's constant entries,
    /// therefore further modification will cause errors. Other table's constant entries will be cleared,
    ///
    /// # Rearrangement Map
    /// The returned [HashMap]<u16, u16> indicates the transformation of constant pool index from key
    /// as original position to value as modified position.
    pub(crate) fn merge(&mut self, other: &mut SymbolTable) -> HashMap<u16, u16> {
        let SymbolTable {
            constants,
            attributes,
            bootstrap_methods,
        } = other;
        let mut rearrangements = HashMap::with_capacity(constants.len());
        let mut rearranged_attrs = IndexSet::with_capacity(bootstrap_methods.len());

        for (index, constant) in constants.iter().enumerate() {
            let new_index = self.insert_constant(constant.to_owned());

            rearrangements.insert((index + 1) as u16, new_index);
        }

        for attribute in attributes.iter() {
            let mut attribute = attribute.clone();

            attribute.rearrange(&rearrangements);
            rearranged_attrs.insert(attribute);
        }

        other.attributes = rearranged_attrs;

        rearrangements
    }

    pub(crate) fn get_utf8(&self, index: u16) -> Option<String> {
        match self.constants.index((index - 1) as usize) {
            Constant::Utf8(constant) => {
                if let Ok(string) = constant.string() {
                    Some(string)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn add_constant(&mut self, constant: Constant) -> u16 {
        self.insert_constant(constant)
    }

    pub(crate) fn add_utf8(&mut self, string: &str) -> u16 {
        let bytes = cesu8::to_java_cesu8(string);
        let constant = Constant::Utf8(Utf8 {
            length: bytes.len() as u16,
            bytes: bytes.to_vec(),
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_class(&mut self, class: &str) -> u16 {
        let name_index = self.add_utf8(class);
        let constant = Constant::Class(Class { name_index });

        self.insert_constant(constant)
    }

    pub(crate) fn add_string(&mut self, string: &str) -> u16 {
        let string_index = self.add_utf8(string);
        let constant = Constant::String(constant::String { string_index });

        self.insert_constant(constant)
    }

    pub(crate) fn add_integer(&mut self, integer: i32) -> u16 {
        let constant = Constant::Integer(Integer {
            bytes: integer.to_be_bytes(),
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_float(&mut self, float: f32) -> u16 {
        let constant = Constant::Float(Float {
            bytes: float.to_be_bytes(),
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_long(&mut self, long: i64) -> u16 {
        let bytes = long.to_be_bytes();
        let (high_bytes, low_bytes) = bytes.split_at(4);
        let constant = Constant::Long(Long {
            high_bytes: high_bytes.try_into().unwrap(),
            low_bytes: low_bytes.try_into().unwrap(),
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_double(&mut self, double: f64) -> u16 {
        let bytes = double.to_be_bytes();
        let (high_bytes, low_bytes) = bytes.split_at(4);
        let constant = Constant::Double(Double {
            high_bytes: high_bytes.try_into().unwrap(),
            low_bytes: low_bytes.try_into().unwrap(),
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_field_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);
        let constant = Constant::FieldRef(FieldRef {
            class_index,
            name_and_type_index,
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_method_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);
        let constant = Constant::MethodRef(MethodRef {
            class_index,
            name_and_type_index,
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_interface_ref(&mut self, class: &str, name: &str, typ: &str) -> u16 {
        let class_index = self.add_class(class);
        let name_and_type_index = self.add_name_and_type(name, typ);
        let constant = Constant::InterfaceMethodRef(InterfaceMethodRef {
            class_index,
            name_and_type_index,
        });

        self.insert_constant(constant)
    }

    pub(crate) fn add_name_and_type(&mut self, name: &str, typ: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let type_index = self.add_utf8(typ);
        let constant = Constant::NameAndType(NameAndType {
            name_index,
            type_index,
        });

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
        let constant = Constant::MethodHandle(MethodHandle {
            reference_kind: *reference_kind as u8,
            reference_index,
        });

        self.insert_constant(constant)
    }

    // TODO: fn add_dynamic()
    // TODO: fn add_invoke_dynamic()

    pub(crate) fn add_module(&mut self, name: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let constant = Constant::Module(Module { name_index });

        self.insert_constant(constant)
    }

    pub(crate) fn add_package(&mut self, name: &str) -> u16 {
        let name_index = self.add_utf8(name);
        let constant = Constant::Package(Package { name_index });

        self.insert_constant(constant)
    }

    pub(crate) fn add_constant_attribute<CV>(&mut self, constant_value: CV) -> u16
    where
        CV: Into<constant_value::ConstantValue>,
    {
        let constant_value = constant_value.into();
        let constant_value_index = match &constant_value {
            constant_value::ConstantValue::Int(val) => self.add_integer(*val),
            constant_value::ConstantValue::Float(val) => self.add_float(*val),
            constant_value::ConstantValue::Long(val) => self.add_long(*val),
            constant_value::ConstantValue::Double(val) => self.add_double(*val),
            constant_value::ConstantValue::String(val) => self.add_string(val),
        };
        let attribute = Attribute::ConstantValue(ConstantValue {
            constant_value_index,
        });

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
        let constant = Constant::Dynamic(Dynamic {
            bootstrap_method_attr_index: boostrap_method_index,
            name_and_type_index,
        });

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
