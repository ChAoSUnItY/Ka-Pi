use std::collections::VecDeque;

use crate::node::signature::BaseType;
use crate::node::signature::WildcardIndicator;

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

    fn init_formal_types(&mut self) {
        if !self.has_formal {
            self.has_formal = true;
            self.signature_builder.push('<');
        }
    }

    fn finalize_formal_types(&mut self) {
        if self.has_formal {
            self.has_formal = false;
            self.signature_builder.push('>');
        }
    }

    //noinspection DuplicatedCode
    pub fn formal_type_parameter(&mut self, name: &str) -> FormalTypeParameterWriter<'_> {
        self.init_formal_types();

        self.signature_builder.push_str(name);

        FormalTypeParameterWriter::new(&mut self.signature_builder)
    }

    pub fn super_class(&mut self) -> TypeWriter<'_> {
        self.finalize_formal_types();

        TypeWriter::new(&mut self.signature_builder)
    }

    pub fn interface(&mut self) -> TypeWriter<'_> {
        self.finalize_formal_types();

        TypeWriter::new(&mut self.signature_builder)
    }
}

impl ToString for ClassSignatureWriter {
    fn to_string(&self) -> String {
        self.signature_builder.clone()
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

    pub fn field_type(&mut self) -> TypeWriter<'_> {
        TypeWriter::new(&mut self.signature_builder)
    }
}

impl ToString for FieldSignatureWriter {
    fn to_string(&self) -> String {
        self.signature_builder.clone()
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

    fn init_formal_types(&mut self) {
        if !self.has_formal {
            self.has_formal = true;
            self.signature_builder.push('<');
        }
    }

    fn finalize_formal_types(&mut self) {
        if self.has_formal {
            self.has_formal = false;
            self.signature_builder.push('>');
        }
    }

    fn init_parameter_types(&mut self) {
        if !self.has_parameters {
            self.has_parameters = true;
            self.signature_builder.push('(');
        }
    }

    fn finalize_parameter_types(&mut self) {
        if self.has_parameters {
            self.has_parameters = false;
            self.signature_builder.push(')');
        }
    }

    //noinspection DuplicatedCode
    pub fn formal_type_parameter(&mut self, name: &str) -> FormalTypeParameterWriter<'_> {
        self.init_formal_types();

        self.signature_builder.push_str(name);

        FormalTypeParameterWriter::new(&mut self.signature_builder)
    }

    pub fn parameter_type(&mut self) -> TypeWriter<'_> {
        self.finalize_formal_types();
        self.init_parameter_types();

        TypeWriter::new(&mut self.signature_builder)
    }

    pub fn return_type(&mut self) -> TypeWriter<'_> {
        self.finalize_formal_types();
        self.finalize_parameter_types();

        TypeWriter::new(&mut self.signature_builder)
    }

    pub fn exception_type(&mut self) -> TypeWriter<'_> {
        self.signature_builder.push('^');

        TypeWriter::new(&mut self.signature_builder)
    }
}

impl ToString for MethodSignatureWriter {
    fn to_string(&self) -> String {
        self.signature_builder.clone()
    }
}

pub struct FormalTypeParameterWriter<'parent> {
    parent_builder: &'parent mut String,
}

impl<'parent> FormalTypeParameterWriter<'parent> {
    fn new(parent_builder: &'parent mut String) -> Self {
        Self { parent_builder }
    }

    pub fn class_bound(&mut self) -> TypeWriter<'_> {
        TypeWriter::new(&mut self.parent_builder)
    }

    pub fn interface_bound(&mut self) -> TypeWriter<'_> {
        self.parent_builder.push(':');

        TypeWriter::new(&mut self.parent_builder)
    }
}

pub struct TypeWriter<'parent> {
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

    pub fn base_type(&mut self, base_type: BaseType) {
        self.parent_builder.push(base_type.into());
    }

    pub fn array_type(&mut self) {
        self.parent_builder.push('[');
    }

    pub fn class_type(&mut self, name: &str) {
        self.parent_builder.push('L');
        self.parent_builder.push_str(name);
        self.parent_builder.push(';');
        self.type_arg_stack.push_front(false);
    }

    pub fn inner_class_type(&mut self, name: &str) {
        self.end_args();
        self.parent_builder.push('.');
        self.parent_builder.push_str(name);
        self.type_arg_stack.push_front(false);
    }

    pub fn type_variable(&mut self, name: &str) {
        self.parent_builder.push('T');
        self.parent_builder.push_str(name);
        self.parent_builder.push(';');
    }

    pub fn wildcard(&mut self) {
        if self.type_arg_stack.front().map_or(false, |b| *b) {
            self.type_arg_stack[0] = true;
            self.parent_builder.push('<');
        }

        self.parent_builder.push('*');
    }

    pub fn type_argument(&mut self, wildcard: WildcardIndicator) {
        if self.type_arg_stack.front().map_or(false, |b| *b) {
            self.type_arg_stack[0] = true;
            self.parent_builder.push('<');
        }

        self.parent_builder.push(wildcard.into());
    }

    pub fn finalize(&mut self) {
        self.end_args();
        self.parent_builder.push(';');
    }
}
