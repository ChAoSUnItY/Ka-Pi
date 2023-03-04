use crate::asm::constants::ConstantObject;

pub trait AnnotationVisitor {
    fn visit(&mut self, name: &String, value: &dyn ConstantObject) {}
    
    fn visit_enum(&mut self, name: &String, descriptor: &String, value: &String) {}
    
    fn visit_annotation(&mut self, name: &String, descriptor: &String) -> Box<dyn AnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }
    
    fn visit_array(&mut self, name: &String) {}
    
    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct AnnotationVisitorImpl {}

impl AnnotationVisitor for AnnotationVisitorImpl {}
