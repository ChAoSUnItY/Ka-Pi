use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use crate::asm::attribute::{Attribute, ConstantValue};
use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::opcodes::{AccessFlag, FieldAccessFlag};
use crate::asm::symbol::SymbolTable;
use crate::error::{KapiError, KapiResult};

#[allow(unused_variables)]
pub trait FieldVisitor {
    fn visit_constant<CV>(&mut self, constant_value: CV) -> KapiResult<()>
    where
        CV: Into<ConstantValue>;

    fn visit_end(&mut self) {}
}

pub struct FieldWriter {
    byte_vec: Rc<RefCell<ByteVecImpl>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    field_symbol_table: SymbolTable,
    access_flags: Vec<FieldAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
}

impl FieldWriter {
    pub(crate) fn new<F>(
        byte_vec: &Rc<RefCell<ByteVecImpl>>,
        symbol_table: &Rc<RefCell<SymbolTable>>,
        access_flags: F,
        name: &str,
        descriptor: &str,
    ) -> KapiResult<Self>
    where
        F: IntoIterator<Item = FieldAccessFlag>,
    {
        let name_index = symbol_table.borrow_mut().add_utf8(name);
        let descriptor_index = symbol_table.borrow_mut().add_utf8(descriptor);

        Ok(Self {
            byte_vec: byte_vec.clone(),
            symbol_table: symbol_table.clone(),
            field_symbol_table: SymbolTable::default(),
            access_flags: access_flags.into_iter().collect(),
            name_index,
            descriptor_index,
        })
    }
}

impl FieldVisitor for FieldWriter {
    fn visit_constant<CV>(&mut self, constant_value: CV) -> KapiResult<()>
    where
        CV: Into<ConstantValue>,
    {
        let constant_value = constant_value.into();

        // Check constant value is consist with give descriptor
        let symbol_table = self.symbol_table.borrow();
        let descriptor = symbol_table.get_utf8(self.descriptor_index).unwrap();

        // TODO: Descriptor must be checked when writer is created
        match descriptor.chars().next().unwrap() {
            'I' | 'S' | 'C' | 'B' | 'Z' => {
                if !matches!(constant_value, ConstantValue::Int(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type {} which can only have int type",
                        descriptor,
                    )))
                }
            }
            'F' => {
                if !matches!(constant_value, ConstantValue::Float(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type F which can only have float type",
                    )))
                }
            }
            'J' => {
                if !matches!(constant_value, ConstantValue::Long(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type J which can only have long type",
                    )))
                }
            }
            'D' => {
                if !matches!(constant_value, ConstantValue::Double(_)) {
                    return Err(KapiError::ArgError(format!(
                        "Field has type D which can only have double type",
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

        self.field_symbol_table
            .add_constant_attribute(constant_value);

        Ok(())
    }

    fn visit_end(&mut self) {
        let Self {
            byte_vec,
            symbol_table,
            field_symbol_table,
            access_flags,
            name_index,
            descriptor_index,
        } = self;

        let mut byte_vec = byte_vec.borrow_mut();
        let mut symbol_table = symbol_table.borrow_mut();
        let rearrangements = symbol_table.merge(field_symbol_table);

        if let Some(new_name_index) = rearrangements.get(name_index) {
            *name_index = *new_name_index;
        }

        if let Some(new_descriptor_index) = rearrangements.get(descriptor_index) {
            *descriptor_index = *new_descriptor_index;
        }

        let field_attributes = field_symbol_table
            .attributes
            .iter()
            .filter(|attr| {
                matches!(
                    attr,
                    Attribute::ConstantValue { .. }
                        | Attribute::Synthetic
                        | Attribute::Deprecate
                        | Attribute::Signature { .. }
                )
            })
            .collect::<Vec<_>>();

        byte_vec.put_be(access_flags.fold_flags()); // access flags
        byte_vec.put_be(*name_index); // name index
        byte_vec.put_be(*descriptor_index); // descriptor index
        byte_vec.put_be(field_attributes.len() as u16); // attribute length
    
        for attribute in field_attributes {
            attribute.bytecode(&mut *byte_vec, &mut *symbol_table);
        }
    }
}
