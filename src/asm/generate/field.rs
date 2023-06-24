use crate::asm::generate::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::generate::constant_value::ConstantValue;
use crate::asm::generate::symbol::SymbolTable;
use crate::asm::generate::types::Type;
use crate::asm::generate::ByteVecGen;
use crate::asm::node::access_flag::{AccessFlags, FieldAccessFlag};
use crate::asm::node::attribute::Attribute;
use crate::error::{KapiError, KapiResult};

pub struct FieldWriter {
    // Internal writing buffers
    symbol_table: SymbolTable,
    // Class file format defined fields
    access_flags: Vec<FieldAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    // State utilities
    expected_field_type: Type,
}

impl FieldWriter {
    pub(crate) fn new<F>(access_flags: F, name: &str, descriptor: &str) -> KapiResult<Self>
    where
        F: IntoIterator<Item = FieldAccessFlag>,
    {
        let mut symbol_table = SymbolTable::default();
        let name_index = symbol_table.add_utf8(name);
        let descriptor_index = symbol_table.add_utf8(descriptor);

        Ok(Self {
            symbol_table,
            access_flags: access_flags.into_iter().collect(),
            name_index,
            descriptor_index,
            expected_field_type: Type::from_descriptor_no_void(descriptor)?,
        })
    }

    pub fn write_constant<CV>(&mut self, constant_value: CV) -> KapiResult<()>
    where
        CV: Into<ConstantValue>,
    {
        let constant_value = constant_value.into();

        // Check constant value is consist with give descriptor
        let symbol_table = &mut self.symbol_table;
        let descriptor = symbol_table.get_utf8(self.descriptor_index).unwrap();

        match &self.expected_field_type {
            Type::Boolean => {
                match constant_value {
                    ConstantValue::Int(value) if value == 0 || value == 1 => {}
                    ConstantValue::Int(value) if value != 0 && value != 1 => {
                        return Err(KapiError::ArgError(format!(
                            "Field has type `I` which can only have int type with value 0 or 1, but got {}",
                            value,
                        )))
                    }
                    _ => {
                        return Err(KapiError::ArgError(format!(
                            "Field has type `I` which can only have int type with value 0 or 1"
                        )))
                    }
                }
            }
            Type::Byte | Type::Short | Type::Char | Type::Int => {
                if !matches!(constant_value, ConstantValue::Int(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type `{}` which can only have int type",
                        descriptor,
                    )))
                }
            }
            Type::Float => {
                if !matches!(constant_value, ConstantValue::Float(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type `F` which can only have float type",
                    )))
                }
            }
            Type::Long => {
                if !matches!(constant_value, ConstantValue::Long(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type `J` which can only have long type",
                    )))
                }
            }
            Type::Double => {
                if !matches!(constant_value, ConstantValue::Double(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type `D` which can only have double type",
                    )))
                }
            }
            Type::ObjectRef(object_ref_type) if object_ref_type.as_str() == "java/lang/String" => {
                if !matches!(constant_value, ConstantValue::String(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type `java/lang/String` which can only have String type",
                    )))
                }
            }
            _ => {
                return Err(KapiError::ArgError(format!(
                    "Field has type {} which cannot have constant value, only primitive types and String can have constant value",
                    descriptor,
                )))
            }
        }

        symbol_table.add_constant_attribute(constant_value);

        Ok(())
    }
}

impl ByteVecGen for FieldWriter {
    fn put(&self, byte_vec: &mut ByteVecImpl, symbol_table: &mut SymbolTable) -> KapiResult<()> {
        let rearrangements = symbol_table.merge(&self.symbol_table)?;
        let name_index = rearrangements
            .get(&self.name_index)
            .unwrap_or(&self.name_index);
        let descriptor_index = rearrangements
            .get(&self.descriptor_index)
            .unwrap_or(&self.descriptor_index);

        let field_attributes = self
            .symbol_table
            .attributes
            .iter()
            .filter(|attr| {
                matches!(
                    attr,
                    Attribute::ConstantValue(..)
                        | Attribute::Synthetic
                        | Attribute::Deprecate
                        | Attribute::Signature { .. }
                )
            })
            .collect::<Vec<_>>();

        byte_vec.put_be(self.access_flags.fold_flags());
        byte_vec.put_be(*name_index);
        byte_vec.put_be(*descriptor_index);
        
        byte_vec.put_be(field_attributes.len() as u16);
        for attribute in field_attributes {
            attribute.put(byte_vec, symbol_table)?;
        }

        Ok(())
    }
}
