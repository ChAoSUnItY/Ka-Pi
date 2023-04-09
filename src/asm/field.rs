use crate::asm::annotation::AnnotationVisitor;
use crate::asm::byte_vec::ByteVec;
use crate::asm::opcodes::FieldAccessFlag;
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

pub struct FieldWriter<'output, BV>
where
    BV: ByteVec,
{
    byte_vec: &'output mut BV,
    symbol_table: &'output mut SymbolTable,
    access: FieldAccessFlag,
    name_index: u16,
    descriptor_index: u16,
}

impl<'output, BV> FieldWriter<'output, BV>
where
    BV: ByteVec,
{
    pub(crate) fn new(
        byte_vec: &'output mut BV,
        symbol_table: &'output mut SymbolTable,
        access: FieldAccessFlag,
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

impl<'output, BV> FieldVisitor for FieldWriter<'output, BV>
where
    BV: ByteVec,
{
    fn visit_end(self)
    where
        Self: Sized,
    {
        let byte_vec = self.byte_vec;

        byte_vec.put_be(self.access as u16);
        byte_vec.put_be(self.name_index);
        byte_vec.put_be(self.descriptor_index);

        // TODO: Implement attribute writing
        byte_vec.put_be(0u16);
    }
}
