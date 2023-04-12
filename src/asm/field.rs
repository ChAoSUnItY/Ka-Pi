use crate::asm::annotation::AnnotationVisitor;
use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::opcodes::{AccessFlag, FieldAccessFlag};
use crate::asm::symbol::SymbolTable;
use crate::asm::types::TypePath;

#[allow(unused_variables)]
pub trait FieldVisitor {
    fn visit_annotation(
        &mut self,
        descriptor: String,
        visible: bool,
    ) -> Option<Box<dyn AnnotationVisitor>> {
        None
    }

    fn visit_type_annotation(
        &mut self,
        type_ref: i32,
        type_path: &TypePath,
        descriptor: String,
        visible: bool,
    ) -> Option<Box<dyn AnnotationVisitor>> {
        None
    }

    // fn visit_attribute(&mut self, attribute: Box<dyn Attribute>) {}

    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub struct FieldWriter<'output> {
    byte_vec: &'output mut ByteVecImpl,
    symbol_table: &'output mut SymbolTable,
    access: &'output [FieldAccessFlag],
    name_index: u16,
    descriptor_index: u16,
}

impl<'output> FieldWriter<'output> {
    pub(crate) fn new(
        byte_vec: &'output mut ByteVecImpl,
        symbol_table: &'output mut SymbolTable,
        access: &'output [FieldAccessFlag],
        name: &str,
        descriptor: &str,
    ) -> Self {
        let name_index = symbol_table.add_utf8(name);
        let descriptor_index = symbol_table.add_utf8(descriptor);

        Self {
            byte_vec,
            symbol_table,
            access,
            name_index,
            descriptor_index,
        }
    }
}

impl<'output> FieldVisitor for FieldWriter<'output> {
    fn visit_end(self)
    where
        Self: Sized,
    {
        let Self {
            byte_vec,
            symbol_table: _,
            access,
            name_index,
            descriptor_index,
        } = self;

        byte_vec.put_be(access.fold_flags());
        byte_vec.put_be(name_index);
        byte_vec.put_be(descriptor_index);

        // TODO: Implement attribute writing
        byte_vec.put_be(0u16);
    }
}
