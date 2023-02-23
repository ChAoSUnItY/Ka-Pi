use crate::asm::annotation::AnnotationVisitor;
use crate::asm::attribute::Attribute;
use crate::asm::types::TypePath;

#[allow(unused_variables)]
pub trait FieldVisitor {
    fn visit_annotation(&mut self, descriptor: String, visible: bool) -> Option<Box<dyn AnnotationVisitor>>
    {
        None
    }
    fn visit_type_annotation(
        &mut self,
        type_ref: i32,
        type_path: &TypePath,
        descriptor: String,
        visible: bool,
    ) -> Option<Box<dyn AnnotationVisitor>>
    {
        None
    }
    fn visit_attribute(&mut self, attribute: Box<dyn Attribute>) {}
    fn visit_end(self) where Self: Sized {}
}
