use crate::asm::annotation::AnnotationVisitor;
use crate::asm::attribute::Attribute;
use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
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

    fn visit_attribute(&mut self, attribute: Box<dyn Attribute>) {}

    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub struct FieldWriter<'output, BV> where BV: ByteVec {
    byte_vec: &'output BV
}

impl<'output, BV> FieldWriter<'output, BV> where BV: ByteVec {
    pub fn new(byte_vec: &'output BV, access: u32, name: &str, descriptor: &str) -> Self {
        Self {
            byte_vec
        }
    }
}

impl<'output, BV> FieldVisitor for FieldWriter<'output, BV> where BV: ByteVec {
    fn visit_attribute(&mut self, attribute: Box<dyn Attribute>) {
        todo!()
    }
}
