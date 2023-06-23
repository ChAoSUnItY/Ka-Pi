use std::collections::VecDeque;

use crate::asm::node::signature::{BaseType, Wildcard};
use crate::asm::visitor::signature::{
    ClassSignatureVisitor, FieldSignatureVisitor, FormalTypeParameterVisitable,
    FormalTypeParameterVisitor, MethodSignatureVisitor, TypeVisitor,
};

/// A default implementation of class signature writer.
///
/// This is commonly used in class file generation.
#[derive(Debug, Default)]
pub struct ClassSignatureWriter {
    signature_builder: String,
    has_formal: bool,
}

impl ClassSignatureWriter {
    /// Reserve capacity for builder to build.
    ///
    /// This is useful when mass appending is required.
    pub fn with_capacity(size: usize) -> Self {
        Self {
            signature_builder: String::with_capacity(size),
            has_formal: false,
        }
    }
}

impl ToString for ClassSignatureWriter {
    fn to_string(&self) -> String {
        self.signature_builder.clone()
    }
}

impl ClassSignatureVisitor for ClassSignatureWriter {
    fn visit_super_class(&mut self) -> Box<dyn TypeVisitor + '_> {
        if self.has_formal {
            self.has_formal = false;
            self.signature_builder.push('>');
        }

        Box::new(TypeWriter::new(&mut self.signature_builder))
    }

    fn visit_interface(&mut self) -> Box<dyn TypeVisitor + '_> {
        if self.has_formal {
            self.has_formal = false;
            self.signature_builder.push('>');
        }

        Box::new(TypeWriter::new(&mut self.signature_builder))
    }
}

impl FormalTypeParameterVisitable for ClassSignatureWriter {
    //noinspection DuplicatedCode
    fn visit_formal_type_parameter(
        &mut self,
        name: &str,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        if !self.has_formal {
            self.has_formal = true;
            self.signature_builder.push('<');
        }

        self.signature_builder.push_str(name);

        Box::new(FormalTypeParameterWriter::new(&mut self.signature_builder))
    }
}

/// A default implementation of field signature writer.
///
/// This is commonly used in class file generation.
#[derive(Debug, Default)]
pub struct FieldSignatureWriter {
    signature_builder: String,
}

impl FieldSignatureWriter {
    /// Reserve capacity for builder to build.
    ///
    /// This is useful when mass appending is required.
    pub fn with_capacity(size: usize) -> Self {
        Self {
            signature_builder: String::with_capacity(size),
        }
    }
}

impl ToString for FieldSignatureWriter {
    fn to_string(&self) -> String {
        self.signature_builder.clone()
    }
}

impl FieldSignatureVisitor for FieldSignatureWriter {
    fn visit_field_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeWriter::new(&mut self.signature_builder))
    }
}

#[derive(Debug, Default)]
pub struct MethodSignatureWriter {
    signature_builder: String,
    has_formal: bool,
    has_parameters: bool,
}

/// A default implementation of method signature writer.
///
/// This is commonly used in class file generation.
impl MethodSignatureWriter {
    /// Reserve capacity for builder to build.
    ///
    /// This is useful when mass appending is required.
    pub fn with_capacity(size: usize) -> Self {
        Self {
            signature_builder: String::with_capacity(size),
            has_formal: false,
            has_parameters: false,
        }
    }
}

impl ToString for MethodSignatureWriter {
    fn to_string(&self) -> String {
        self.signature_builder.clone()
    }
}

impl MethodSignatureVisitor for MethodSignatureWriter {
    fn visit_parameter_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        if self.has_formal {
            self.has_formal = false;
            self.signature_builder.push('>');
        }

        if !self.has_parameters {
            self.has_parameters = true;
            self.signature_builder.push('(');
        }

        Box::new(TypeWriter::new(&mut self.signature_builder))
    }

    fn visit_return_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        if self.has_formal {
            self.has_formal = false;
            self.signature_builder.push('>');
        }

        if self.has_parameters {
            self.has_parameters = false;
        } else {
            self.signature_builder.push('(');
        }

        self.signature_builder.push(')');

        Box::new(TypeWriter::new(&mut self.signature_builder))
    }

    fn visit_exception_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        self.signature_builder.push('^');

        Box::new(TypeWriter::new(&mut self.signature_builder))
    }
}

impl FormalTypeParameterVisitable for MethodSignatureWriter {
    //noinspection DuplicatedCode
    fn visit_formal_type_parameter(
        &mut self,
        name: &str,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        if !self.has_formal {
            self.has_formal = true;
            self.signature_builder.push('<');
        }

        self.signature_builder.push_str(name);

        Box::new(FormalTypeParameterWriter::new(&mut self.signature_builder))
    }
}

struct FormalTypeParameterWriter<'parent> {
    parent_builder: &'parent mut String,
}

impl<'parent> FormalTypeParameterWriter<'parent> {
    fn new(parent_builder: &'parent mut String) -> Self {
        Self { parent_builder }
    }
}

impl<'parent> FormalTypeParameterVisitor for FormalTypeParameterWriter<'parent> {
    fn visit_class_bound(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeWriter::new(&mut self.parent_builder))
    }

    fn visit_interface_bound(&mut self) -> Box<dyn TypeVisitor + '_> {
        self.parent_builder.push(':');
        Box::new(TypeWriter::new(&mut self.parent_builder))
    }
}

struct TypeWriter<'parent> {
    parent_builder: &'parent mut String,
    type_arg_stack: VecDeque<bool>,
}

impl<'parent> TypeWriter<'parent> {
    fn new(parent_builder: &'parent mut String) -> Self {
        let mut type_arg_stack = VecDeque::with_capacity(64);

        type_arg_stack.push_back(true);

        Self {
            parent_builder,
            type_arg_stack,
        }
    }

    fn end_args(&mut self) {
        if self.type_arg_stack.front().map_or(false, |b| *b) {
            self.parent_builder.push('>');
        }

        self.type_arg_stack.pop_front();
    }
}

impl<'parent> TypeVisitor for TypeWriter<'parent> {
    fn visit_base_type(&mut self, base_type: BaseType) {
        self.parent_builder.push(base_type.into());
    }

    fn visit_array_type(&mut self) {
        self.parent_builder.push('[');
    }

    fn visit_class_type(&mut self, name: &str) {
        self.parent_builder.push('L');
        self.parent_builder.push_str(name);
        self.parent_builder.push(';');
        self.type_arg_stack.push_front(false);
    }

    fn visit_inner_class_type(&mut self, name: &str) {
        self.end_args();
        self.parent_builder.push('.');
        self.parent_builder.push_str(name);
        self.type_arg_stack.push_front(false);
    }

    fn visit_type_variable(&mut self, name: &str) {
        self.parent_builder.push('T');
        self.parent_builder.push_str(name);
        self.parent_builder.push(';');
    }

    fn visit_type_argument(&mut self) {
        if self.type_arg_stack.front().map_or(false, |b| *b) {
            self.type_arg_stack[0] = true;
            self.parent_builder.push('<');
        }

        self.parent_builder.push('*');
    }

    fn visit_type_argument_wildcard(&mut self, wildcard: Wildcard) {
        if self.type_arg_stack.front().map_or(false, |b| *b) {
            self.type_arg_stack[0] = true;
            self.parent_builder.push('<');
        }

        if wildcard != Wildcard::INSTANCEOF {
            self.parent_builder.push(wildcard.into());
        }
    }

    fn visit_end(&mut self) {
        self.end_args();
        self.parent_builder.push(';');
    }
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use crate::asm::generate::signature::{
        ClassSignatureWriter, FieldSignatureWriter, MethodSignatureWriter,
    };
    use crate::asm::visitor::signature::{
        ClassSignatureVisitor, FieldSignatureVisitor, FormalTypeParameterVisitable,
        MethodSignatureVisitor,
    };

    #[test]
    fn test_class_signature_writer() {
        let mut writer = ClassSignatureWriter::default();

        writer
            .visit_formal_type_parameter(&"T".to_string())
            .visit_class_bound()
            .visit_class_type("java/lang/Object");

        writer
            .visit_super_class()
            .visit_class_type("java/lang/Object");
        writer
            .visit_interface()
            .visit_class_type("java/lang/Comparable");

        assert_yaml_snapshot!(writer.to_string());
    }

    #[test]
    fn test_field_signature_writer() {
        let mut writer = FieldSignatureWriter::default();

        writer
            .visit_field_type()
            .visit_class_type("java/lang/String");

        assert_yaml_snapshot!(writer.to_string());
    }

    #[test]
    fn test_method_signature_writer() {
        let mut writer = MethodSignatureWriter::default();

        writer
            .visit_formal_type_parameter(&"T".to_string())
            .visit_class_bound()
            .visit_class_type("java/lang/Object");

        writer
            .visit_parameter_type()
            .visit_class_type("java/lang/Object");
        writer
            .visit_return_type()
            .visit_class_type("java/lang/String");

        assert_yaml_snapshot!(writer.to_string());
    }
}
