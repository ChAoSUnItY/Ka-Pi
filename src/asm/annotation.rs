use std::cell::RefCell;
use std::rc::Rc;
use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
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
        Box::<AnnotationVisitorImpl>::default()
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait RuntimeAnnotationParameterVisitor {
    fn visit_parameter(&mut self) -> Box<dyn ParameterVisitor + '_> {
        Box::<AnnotationVisitorImpl>::default()
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait ParameterVisitor {
    fn visit_annotation(
        &mut self,
        descriptor: &str,
    ) -> Box<dyn InnerRuntimeAnnotationVisitor + '_> {
        Box::<AnnotationVisitorImpl>::default()
    }

    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait InnerRuntimeAnnotationVisitor {
    fn visit_constant_value(&mut self, base_type: BaseType) {}

    fn visit_enum(&mut self, descriptor: &str, value: &str) {}

    fn visit_class(&mut self, descriptor: &str) {}

    fn visit_annotation(&mut self) -> Box<dyn RuntimeAnnotationVisitor + '_> {
        Box::<AnnotationVisitorImpl>::default()
    }

    fn visit_array(&mut self, len: usize) {}

    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct AnnotationVisitorImpl {}

impl AnnotationVisitor for AnnotationVisitorImpl {}
impl RuntimeAnnotationVisitor for AnnotationVisitorImpl {}
impl RuntimeAnnotationParameterVisitor for AnnotationVisitorImpl {}
impl ParameterVisitor for AnnotationVisitorImpl {}
impl InnerRuntimeAnnotationVisitor for AnnotationVisitorImpl {}

#[derive(Debug)]
struct AnnotationWriter {
    output: Rc<RefCell<ByteVecImpl>>
}

impl AnnotationWriter {
    pub(crate) fn new(output: &Rc<RefCell<ByteVecImpl>>) -> Self {
        Self {
            output: output.clone(),
        }
    }
}

impl AnnotationVisitor for AnnotationWriter {
    fn visit_runtime_annotation(&mut self, visible: bool) -> Box<dyn RuntimeAnnotationVisitor + '_> {
        Box::new(RuntimeAnnotationWriter::new(&self.output, visible))
    }
}

struct RuntimeAnnotationWriter {
    output: Rc<RefCell<ByteVecImpl>>,
    visible: bool,
}

impl RuntimeAnnotationWriter {
    pub(crate) fn new(output: &Rc<RefCell<ByteVecImpl>>, visible: bool) -> Self {
        Self {
            output: output.clone(),
            visible
        }
    }
}

impl RuntimeAnnotationVisitor for RuntimeAnnotationWriter {
    fn visit_annotation(&mut self, descriptor: &str) -> Box<dyn InnerRuntimeAnnotationVisitor + '_> {
        Box::new(InnerRuntimeAnnotationWriter::new(&self.output))
    }
}

struct InnerRuntimeAnnotationWriter {
    output: Rc<RefCell<ByteVecImpl>>,
}

impl InnerRuntimeAnnotationWriter {
    pub(crate) fn new(output: &Rc<RefCell<ByteVecImpl>>) -> Self {
        Self {
            output: output.clone()
        }
    }
}

impl InnerRuntimeAnnotationVisitor for InnerRuntimeAnnotationWriter {
    
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::asm::annotation::AnnotationWriter;
    use crate::asm::byte_vec::ByteVecImpl;

    #[test]
    fn test_annotation_writer() {
        let mut output = Rc::new(RefCell::new(ByteVecImpl::with_capacity(32)));
        let mut writer = AnnotationWriter::new(&output);
        
        println!("{:?}", output);
    }    
}
