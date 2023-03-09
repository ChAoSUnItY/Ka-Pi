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
struct AnnotationWriter<'output, BV> where BV: ByteVec {
    output: &'output mut BV,
}

impl<'output, BV> AnnotationWriter<'output, BV> where BV: ByteVec {
    pub(crate) fn new(output: &'output mut BV) -> Self {
        Self {
            output,
        }
    }
}

impl<'output, BV> AnnotationVisitor for AnnotationWriter<'output, BV> where BV: ByteVec {
    fn visit_runtime_annotation(&mut self, visible: bool) -> Box<dyn RuntimeAnnotationVisitor + '_> {
        Box::new(RuntimeAnnotationWriter::new(self.output, visible))
    }
}

struct RuntimeAnnotationWriter<'output, BV> where BV: ByteVec {
    output: &'output mut BV,
    visible: bool,
}

impl<'output, BV> RuntimeAnnotationWriter<'output, BV> where BV: ByteVec {
    pub(crate) fn new(output: &'output mut BV, visible: bool) -> Self {
        Self {
            output,
            visible
        }
    }
}

impl<'output, BV> RuntimeAnnotationVisitor for RuntimeAnnotationWriter<'output, BV> where BV: ByteVec {
    fn visit_annotation(&mut self, descriptor: &str) -> Box<dyn InnerRuntimeAnnotationVisitor + '_> {
        Box::new(InnerRuntimeAnnotationWriter::new(self.output))
    }
}

struct InnerRuntimeAnnotationWriter<'output, BV> where BV: ByteVec {
    output: &'output mut BV,
}

impl<'output, BV> InnerRuntimeAnnotationWriter<'output, BV> where BV: ByteVec {
    pub(crate) fn new(output: &'output mut BV) -> Self {
        Self {
            output
        }
    }
}

impl<'output, BV> InnerRuntimeAnnotationVisitor for InnerRuntimeAnnotationWriter<'output, BV> where BV: ByteVec {
    
}

#[cfg(test)]
mod test {
    use crate::asm::annotation::AnnotationWriter;
    use crate::asm::byte_vec::ByteVecImpl;

    #[test]
    fn test_annotation_writer() {
        let mut output = ByteVecImpl::with_capacity(32);
        let mut writer = AnnotationWriter::new(&mut output); 
        
        println!("{:?}", output);
    }    
}
