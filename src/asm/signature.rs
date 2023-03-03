use std::collections::VecDeque;
use std::{default, iter::Peekable, str::CharIndices};

use either::Either;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::asm::field::FieldVisitor;
use crate::error::{KapiError, KapiResult};

const EXTENDS: char = '+';
const SUPER: char = '-';
const INSTANCEOF: char = '=';

/// An enum representation for wildcard indicators, which is used in
/// [`Type::WildcardTypeArgument`](crate::asm::node::signature::Type::WildcardTypeArgument) as class
/// type argument bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Wildcard {
    /// Indicates type argument must extends class bound, see java's upper bounds wildcard.
    EXTENDS = EXTENDS as u8,
    /// Indicates type argument must super class bound, see java's lower bounds wildcard.
    SUPER = SUPER as u8,
    /// Indicates type argument must be instance of specified type.
    INSTANCEOF = INSTANCEOF as u8,
}

impl Into<char> for Wildcard {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl TryFrom<char> for Wildcard {
    type Error = KapiError;

    fn try_from(value: char) -> KapiResult<Self> {
        match value {
            EXTENDS => Ok(Wildcard::EXTENDS),
            SUPER => Ok(Wildcard::SUPER),
            INSTANCEOF => Ok(Self::INSTANCEOF),
            _ => Err(KapiError::ArgError(format!(
                "Character {} cannot be converted into Wildcard",
                value
            ))),
        }
    }
}

impl TryFrom<&char> for Wildcard {
    type Error = KapiError;

    fn try_from(value: &char) -> KapiResult<Self> {
        match *value {
            EXTENDS => Ok(Wildcard::EXTENDS),
            SUPER => Ok(Wildcard::SUPER),
            INSTANCEOF => Ok(Self::INSTANCEOF),
            _ => Err(KapiError::ArgError(format!(
                "Character {} cannot be converted into Wildcard",
                value
            ))),
        }
    }
}

/// A visitor to visit class generic signature. This trait requires struct also implements 
/// [FormalTypeParameterVisitable].
pub trait ClassSignatureVisitor: FormalTypeParameterVisitable {
    /// Visits class generic signature's super class type. This would be called on every classes 
    /// expect `java.lang.Object`.
    fn visit_super_class(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }

    /// Visits class generic signature's interface type. This could be called by multiple times
    /// when there's more than 1 interfaces implemented.
    fn visit_interface(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }

    /// Finalizes the visitor for further process.
    fn visit_end(&mut self) {}
}

pub trait FieldSignatureVisitor {
    fn visit_field_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

pub trait MethodSignatureVisitor: FormalTypeParameterVisitable {
    fn visit_parameter_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_return_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_exception_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait FormalTypeParameterVisitable {
    fn visit_formal_type_parameter(
        &mut self,
        name: &String,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
}

#[allow(unused_variables)]
pub trait FormalTypeParameterVisitor {
    fn visit_class_bound(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_interface_bound(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

#[allow(unused_variables)]
pub trait TypeVisitor {
    fn visit_base_type(&mut self, char: &char) {}
    fn visit_array_type(&mut self) {}
    fn visit_class_type(&mut self, name: &String) {}
    fn visit_inner_class_type(&mut self, name: &String) {}
    fn visit_type_variable(&mut self, name: &String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(&mut self, wildcard: Wildcard) {}
    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct SignatureVisitorImpl {}

impl ClassSignatureVisitor for SignatureVisitorImpl {}
impl FieldSignatureVisitor for SignatureVisitorImpl {}
impl MethodSignatureVisitor for SignatureVisitorImpl {}
impl FormalTypeParameterVisitable for SignatureVisitorImpl {}
impl FormalTypeParameterVisitor for SignatureVisitorImpl {}
impl TypeVisitor for SignatureVisitorImpl {}

#[derive(Debug, Default)]
pub struct ClassSignatureReader {
    signature: String,
}

impl ClassSignatureReader {
    pub fn new<S>(signature: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            signature: signature.into(),
        }
    }

    pub fn accept(&mut self, visitor: &mut impl ClassSignatureVisitor) -> KapiResult<()> {
        accept_class_signature_visitor(self, visitor)
    }
}

#[derive(Debug, Default)]
pub struct MethodSignatureReader {
    signature: String,
}

impl MethodSignatureReader {
    pub fn new<S>(signature: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            signature: signature.into(),
        }
    }

    pub fn accept(&mut self, visitor: &mut impl MethodSignatureVisitor) -> KapiResult<()> {
        accept_method_signature_visitor(self, visitor)
    }
}

#[derive(Debug, Default)]
pub struct FieldSignatureReader {
    signature: String,
}

impl FieldSignatureReader {
    pub fn new<S>(signature: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            signature: signature.into(),
        }
    }

    pub fn accept(&mut self, visitor: &mut impl FieldSignatureVisitor) -> KapiResult<()> {
        accept_field_signature_visitor(self, visitor)
    }
}

pub fn accept_class_signature_visitor(
    reader: &mut ClassSignatureReader,
    visitor: &mut impl ClassSignatureVisitor,
) -> KapiResult<()> {
    let mut signature_iter = reader.signature.chars().peekable();

    // Formal type parameters
    accept_formal_type_parameters(&mut signature_iter, visitor)?;

    // Super class type
    accept_class_type(&mut signature_iter, &mut visitor.visit_super_class())?;

    // Interface class types
    while signature_iter.peek().is_some() {
        accept_class_type(&mut signature_iter, &mut visitor.visit_interface())?;
    }

    visitor.visit_end();

    // Strict check
    strict_check_iter_empty(&mut signature_iter)
}

fn accept_field_signature_visitor(
    reader: &mut FieldSignatureReader,
    visitor: &mut impl FieldSignatureVisitor,
) -> KapiResult<()> {
    let mut signature_iter = reader.signature.chars().peekable();

    // Field type
    accept_type(&mut signature_iter, &mut visitor.visit_field_type())?;

    visitor.visit_end();

    // Strict check
    strict_check_iter_empty(&mut signature_iter)
}

fn accept_method_signature_visitor(
    reader: &mut MethodSignatureReader,
    visitor: &mut impl MethodSignatureVisitor,
) -> KapiResult<()> {
    let mut signature_iter = reader.signature.chars().peekable();

    // Formal type parameters
    accept_formal_type_parameters(&mut signature_iter, visitor)?;

    // Parameter types
    if signature_iter.next_if_eq(&'(').is_some() {
        loop {
            if signature_iter.peek().is_none() {
                return Err(KapiError::ClassParseError(String::from(
                    "Attempt to parse method parameter types in signature but parameters are not enclosed by `)`"
                )));
            } else if signature_iter.next_if_eq(&')').is_some() {
                break;
            }

            accept_type(&mut signature_iter, &mut visitor.visit_parameter_type())?;
        }
    }

    // Return type
    accept_type(&mut signature_iter, &mut visitor.visit_return_type())?;

    // Exception types (opt)
    if signature_iter.next_if_eq(&'^').is_some() {
        accept_type(&mut signature_iter, &mut visitor.visit_exception_type())?;
    }

    visitor.visit_end();

    // Strict check
    strict_check_iter_empty(&mut signature_iter)
}

fn accept_formal_type_parameters<SI, FTV>(
    signature_iter: &mut Peekable<SI>,
    formal_type_visitable: &mut FTV,
) -> KapiResult<()>
where
    SI: Iterator<Item = char> + Clone,
    FTV: FormalTypeParameterVisitable + ?Sized,
{
    if signature_iter.next_if_eq(&'<').is_some() {
        loop {
            let formal_type_parameter = signature_iter.by_ref().take_while(|c| *c != ':').collect();
            let mut formal_type_visitor =
                formal_type_visitable.visit_formal_type_parameter(&formal_type_parameter);

            let char = signature_iter.peek().ok_or(KapiError::ClassParseError(String::from(
                "Attempt to parse class bound for formal type parameter in signature but parameters are not enclosed by `>`"
            )))?;

            // Class bound
            match *char {
                'L' | '[' | 'T' => {
                    accept_type(signature_iter, &mut formal_type_visitor.visit_class_bound())?
                }
                _ => {}
            }

            // Interface bounds
            loop {
                if signature_iter.peek().is_none() {
                    return Err(KapiError::ClassParseError(String::from(
                        "Attempt to parse interface bounds for formal type parameter in signature but parameters are not enclosed by `>`"
                    )));
                } else if signature_iter.next_if(|c| *c == ':').is_some() {
                    accept_type(
                        signature_iter,
                        &mut formal_type_visitor.visit_interface_bound(),
                    )?;
                } else {
                    break;
                }
            }

            formal_type_visitor.visit_end();

            if signature_iter.peek().is_none() {
                return Err(KapiError::ClassParseError(String::from(
                    "Attempt to parse class and interface bounds for formal type parameter in signature but parameters are not enclosed by `>`"
                )));
            } else if signature_iter.next_if_eq(&'>').is_some() {
                break;
            }
        }
    }

    Ok(())
}

fn accept_type<SI, CTV>(signature_iter: &mut Peekable<SI>, visitor: &mut Box<CTV>) -> KapiResult<()>
where
    SI: Iterator<Item = char> + Clone,
    CTV: TypeVisitor + ?Sized,
{
    let char = signature_iter
        .peek()
        .ok_or(KapiError::ClassParseError(String::from("Expected primitive type, array type, class type, or type variable descriptor, but got nothing")))?;

    match char {
        'Z' | 'C' | 'B' | 'S' | 'I' | 'F' | 'J' | 'D' | 'V' => {
            visitor.visit_base_type(&char);
            visitor.visit_end();
            signature_iter.next();
            Ok(())
        }
        '[' => {
            visitor.visit_array_type();
            signature_iter.next();
            accept_type(signature_iter, visitor)
        },
        'T' => {
            signature_iter.next();
            let type_variable = signature_iter
                .take_while(|c| *c != ';')
                .collect();
            visitor.visit_type_variable(&type_variable);
            visitor.visit_end();
            Ok(())
        }
        'L' => accept_class_type(signature_iter, visitor),
        _ => Err(KapiError::ClassParseError(format!(
            "Expected primitive type, array type, class type, or type variable descriptor, but got `{}`",
            char
        )))
    }
}

fn accept_class_type<SI, CTV>(
    signature_iter: &mut Peekable<SI>,
    visitor: &mut Box<CTV>,
) -> KapiResult<()>
where
    SI: Iterator<Item = char> + Clone,
    CTV: TypeVisitor + ?Sized,
{
    let mut visited = false;
    let mut inner = false;

    loop {
        let _char = signature_iter
            .next()
            .ok_or(KapiError::ClassParseError(String::from(
                "Expected `L` (object descriptor prefix) but got nothing",
            )))?; // Object descriptor prefix `L` is now supposedly consumed
        let name = signature_iter
            .take_while_ref(|c| *c != '.' && *c != ';' && *c != '<')
            .collect();
        let suffix = signature_iter.next().ok_or(KapiError::ClassParseError(String::from(
            "Expected character `.` or `;` for class type, or `<` for type variable descriptor but got nothing"
        )))?;

        match suffix {
            '.' | ';' => {
                if !visited {
                    if inner {
                        visitor.visit_inner_class_type(&name);
                    } else {
                        visitor.visit_class_type(&name);
                    }
                }

                if suffix == ';' {
                    visitor.visit_end();
                    break;
                }

                visited = false;
                inner = true;
            }
            '<' => {
                if !visited {
                    if inner {
                        visitor.visit_inner_class_type(&name);
                    } else {
                        visitor.visit_class_type(&name);
                    }
                }

                visited = true;

                loop {
                    if signature_iter.peek().is_none() {
                        return Err(KapiError::ClassParseError(String::from(
                            "Attempt to parse type arguments in signature but arguments are not enclosed by `>`"
                        )));
                    } else if signature_iter.next_if(|c| *c == '>').is_some() {
                        break;
                    }

                    let char = signature_iter.next().ok_or(KapiError::ClassParseError(String::from(
                        "Expected wildcard character `*`, `+`, `-`, or any other type but got nothing"
                    )))?;

                    match char {
                        '*' => visitor.visit_type_argument(),
                        '+' => {
                            visitor.visit_type_argument_wildcard(Wildcard::EXTENDS);
                            accept_class_type(signature_iter, visitor)?;
                        },
                        '-' => {
                            visitor.visit_type_argument_wildcard(Wildcard::SUPER);
                            accept_class_type(signature_iter, visitor)?;
                        },
                        _ => {
                            visitor.visit_type_argument_wildcard(Wildcard::INSTANCEOF);
                            accept_class_type(signature_iter, visitor)?;
                        },
                    }
                }
            }
            _ => return Err(KapiError::ClassParseError(format!(
                "Expected character `.` or `;` for class type, or `<` for type variable descriptor but got `{}`",
                suffix
            )))
        }
    }

    Ok(())
}

fn strict_check_iter_empty<SI>(signature_iter: &mut Peekable<SI>) -> KapiResult<()>
where
    SI: Iterator<Item = char>,
{
    let remaining = signature_iter.collect::<String>();

    if remaining.is_empty() {
        Ok(())
    } else {
        Err(KapiError::ClassParseError(format!(
            "Expected nothing after fully parsed but got `{}`",
            remaining
        )))
    }
}

#[derive(Debug, Default)]
pub struct ClassSignatureWriter {
    signature_builder: String,
    has_formal: bool,
}

impl ClassSignatureWriter {
    fn with_capacity(size: usize) -> Self {
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
    fn visit_formal_type_parameter(
        &mut self,
        name: &String,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        if !self.has_formal {
            self.has_formal = true;
            self.signature_builder.push('<');
        }

        self.signature_builder.push_str(name);

        Box::new(FormalTypeParameterWriter::new(&mut self.signature_builder))
    }
}

#[derive(Debug, Default)]
pub struct FieldSignatureWriter {
    signature_builder: String,
}

impl FieldSignatureWriter {
    fn with_capacity(size: usize) -> Self {
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

impl MethodSignatureWriter {
    fn with_capacity(size: usize) -> Self {
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
    fn visit_formal_type_parameter(
        &mut self,
        name: &String,
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
    fn visit_base_type(&mut self, char: &char) {
        self.parent_builder.push(*char);
    }

    fn visit_array_type(&mut self) {
        self.parent_builder.push('[');
    }

    fn visit_class_type(&mut self, name: &String) {
        self.parent_builder.push('L');
        self.parent_builder.push_str(name);
        self.parent_builder.push(';');
        self.type_arg_stack.push_front(false);
    }

    fn visit_inner_class_type(&mut self, name: &String) {
        self.end_args();
        self.parent_builder.push('.');
        self.parent_builder.push_str(name);
        self.type_arg_stack.push_front(false);
    }

    fn visit_type_variable(&mut self, name: &String) {
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
    use rstest::rstest;

    use crate::asm::signature::{
        ClassSignatureReader, ClassSignatureVisitor, ClassSignatureWriter, FieldSignatureReader,
        FieldSignatureVisitor, FieldSignatureWriter, FormalTypeParameterVisitable,
        FormalTypeParameterVisitor, MethodSignatureReader, MethodSignatureVisitor,
        MethodSignatureWriter, SignatureVisitorImpl, TypeVisitor,
    };
    use crate::error::KapiResult;

    #[rstest]
    #[case("<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")]
    fn test_class_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let mut visitor = SignatureVisitorImpl::default();
        let mut reader = ClassSignatureReader::new(signature.to_string());

        reader.accept(&mut visitor)?;

        Ok(())
    }

    #[rstest]
    #[case("Ljava/lang/Object;")]
    #[case("TT;")]
    fn test_field_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let mut visitor = SignatureVisitorImpl::default();
        let mut reader = FieldSignatureReader::new(signature.to_string());

        reader.accept(&mut visitor)?;

        Ok(())
    }

    #[rstest]
    #[case("<T:Ljava/lang/Object;>(Z[[Z)Ljava/lang/Object;^Ljava/lang/Exception;")]
    fn test_method_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let mut visitor = SignatureVisitorImpl::default();
        let mut reader = MethodSignatureReader::new(signature.to_string());

        reader.accept(&mut visitor)?;

        Ok(())
    }

    #[test]
    fn test_class_signature_writer() {
        let mut writer = ClassSignatureWriter::default();

        writer
            .visit_formal_type_parameter(&"T".to_string())
            .visit_class_bound()
            .visit_class_type(&"java/lang/Object".to_string());

        writer
            .visit_super_class()
            .visit_class_type(&"java/lang/Object".to_string());
        writer
            .visit_interface()
            .visit_class_type(&"java/lang/Comparable".to_string());

        assert_yaml_snapshot!(writer.to_string());
    }

    #[test]
    fn test_field_signature_writer() {
        let mut writer = FieldSignatureWriter::default();

        writer
            .visit_field_type()
            .visit_class_type(&"java/lang/String".to_string());

        assert_yaml_snapshot!(writer.to_string());
    }

    #[test]
    fn test_method_signature_writer() {
        let mut writer = MethodSignatureWriter::default();

        writer
            .visit_formal_type_parameter(&"T".to_string())
            .visit_class_bound()
            .visit_class_type(&"java/lang/Object".to_string());

        writer
            .visit_parameter_type()
            .visit_class_type(&"java/lang/Object".to_string());
        writer
            .visit_return_type()
            .visit_class_type(&"java/lang/String".to_string());

        assert_yaml_snapshot!(writer.to_string());
    }
}
