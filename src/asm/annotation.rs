use crate::asm::constants::ConstantObject;

pub trait AnnotationVisitor {
    fn visit(&mut self, name: &str, value: &dyn ConstantObject) {}
    
    fn visit_enum(&mut self, name: &str, descriptor: &str, value: &str) {}
    
    fn visit_annotation(&mut self, name: &str, descriptor: &str) -> Box<dyn AnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }
    
    fn visit_array(&mut self, name: &str) {}
    
    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct AnnotationVisitorImpl {}

impl AnnotationVisitor for AnnotationVisitorImpl {}
