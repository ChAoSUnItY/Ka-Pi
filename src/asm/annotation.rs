use crate::asm::node::types::BaseType;

#[allow(unused_variables)]
pub trait AnnotationVisitor {
    fn visit_runtime_annotation(
        &mut self,
        visible: bool,
    ) -> Box<dyn RuntimeAnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_runtime_annotation_parameter(
        &mut self,
        name: &str,
        visible: bool,
    ) -> Box<dyn RuntimeAnnotationParameterVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait RuntimeAnnotationVisitor {
    fn visit_annotation(
        &mut self,
        descriptor: &str,
    ) -> Box<dyn InnerRuntimeAnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait RuntimeAnnotationParameterVisitor {
    fn visit_parameter(&mut self) -> Box<dyn ParameterVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait ParameterVisitor {
    fn visit_annotation(
        &mut self,
        descriptor: &str,
    ) -> Box<dyn InnerRuntimeAnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait InnerRuntimeAnnotationVisitor {
    fn visit_constant_value(&mut self, base_type: BaseType) {}

    fn visit_enum(&mut self, descriptor: &str, value: &str) {}

    fn visit_class(&mut self, descriptor: &str) {}

    fn visit_annotation(&mut self) -> Box<dyn RuntimeAnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_array(&mut self) -> Box<dyn InnerRuntimeAnnotationVisitor + '_> {
        Box::new(AnnotationVisitorImpl::default())
    }

    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct AnnotationVisitorImpl {}

impl AnnotationVisitor for AnnotationVisitorImpl {}
impl RuntimeAnnotationVisitor for AnnotationVisitorImpl {}
impl RuntimeAnnotationParameterVisitor for AnnotationVisitorImpl {}
impl ParameterVisitor for AnnotationVisitorImpl {}
impl InnerRuntimeAnnotationVisitor for AnnotationVisitorImpl {}
