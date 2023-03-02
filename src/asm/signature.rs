use std::{default, iter::Peekable, str::CharIndices};

use either::Either;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

use crate::error::{KapiError, KapiResult};

const EXTENDS: char = '+';
const SUPER: char = '-';
const INSTANCEOF: char = '=';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Wildcard {
    EXTENDS = EXTENDS as u8,
    SUPER = SUPER as u8,
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

pub trait ClassSignatureVisitor: FormalTypeParameterVisitable {
    fn visit_super_class(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_interface(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
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
    fn visit_array_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_class_type(&mut self, name: &String) {}
    fn visit_inner_class_type(&mut self, name: &String) {}
    fn visit_type_variable(&mut self, name: &String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(&mut self, wildcard: Wildcard) -> Box<dyn TypeVisitor + '_> {
        Box::new(SignatureVisitorImpl::default())
    }
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

pub trait SignatureReader {
    fn signature(&self) -> &String;
}

pub trait ClassSignatureReader: SignatureReader {
    fn accept(&mut self, visitor: &mut Box<dyn ClassSignatureVisitor>) -> KapiResult<()> {
        accept_class_signature_visitor(self, visitor)
    }
}

#[derive(Debug, Default)]
pub struct ClassSignatureReaderImpl {
    signature: String,
}

impl ClassSignatureReaderImpl {
    pub const fn new(signature: String) -> Self {
        Self { signature }
    }
}

impl ClassSignatureReader for ClassSignatureReaderImpl {}
impl SignatureReader for ClassSignatureReaderImpl {
    fn signature(&self) -> &String {
        &self.signature
    }
}

pub trait MethodSignatureReader: SignatureReader {
    fn accept(&mut self, visitor: &mut Box<dyn MethodSignatureVisitor>) -> KapiResult<()> {
        accept_method_signature_visitor(self, visitor)
    }
}

#[derive(Debug, Default)]
pub struct MethodSignatureReaderImpl {
    signature: String,
}

impl MethodSignatureReaderImpl {
    pub const fn new(signature: String) -> Self {
        Self { signature }
    }
}

impl MethodSignatureReader for MethodSignatureReaderImpl {}
impl SignatureReader for MethodSignatureReaderImpl {
    fn signature(&self) -> &String {
        &self.signature
    }
}

pub trait FieldSignatureReader: SignatureReader {
    fn accept(&mut self, visitor: &mut Box<dyn FieldSignatureVisitor>) -> KapiResult<()> {
        accept_field_signature_visitor(self, visitor)
    }
}

#[derive(Debug, Default)]
pub struct FieldSignatureReaderImpl {
    signature: String,
}

impl FieldSignatureReaderImpl {
    pub const fn new(signature: String) -> Self {
        Self { signature }
    }
}

impl FieldSignatureReader for FieldSignatureReaderImpl {}
impl SignatureReader for FieldSignatureReaderImpl {
    fn signature(&self) -> &String {
        &self.signature
    }
}

pub fn accept_class_signature_visitor<CSR, CSV>(
    reader: &mut CSR,
    visitor: &mut Box<CSV>,
) -> KapiResult<()>
where
    CSR: ClassSignatureReader + ?Sized,
    CSV: ClassSignatureVisitor + ?Sized,
{
    let mut signature_iter = reader.signature().chars().peekable();

    // Formal type parameters
    accept_formal_type_parameters(&mut signature_iter, visitor)?;

    // Super class type
    accept_class_type(&mut signature_iter, visitor.visit_super_class())?;

    // Interface class types
    while signature_iter.peek().is_some() {
        accept_class_type(&mut signature_iter, visitor.visit_interface())?;
    }
    
    visitor.visit_end();

    // Strict check
    strict_check_iter_empty(&mut signature_iter)
}

fn accept_field_signature_visitor<FSR, FSV>(
    reader: &mut FSR,
    visitor: &mut Box<FSV>,
) -> KapiResult<()>
where
    FSR: FieldSignatureReader + ?Sized,
    FSV: FieldSignatureVisitor + ?Sized,
{
    let mut signature_iter = reader.signature().chars().peekable();

    // Field type
    accept_type(&mut signature_iter, visitor.visit_field_type())?;

    visitor.visit_end();
    
    // Strict check
    strict_check_iter_empty(&mut signature_iter)
}

fn accept_method_signature_visitor<MSR, MSV>(
    reader: &mut MSR,
    visitor: &mut Box<MSV>,
) -> KapiResult<()>
where
    MSR: MethodSignatureReader + ?Sized,
    MSV: MethodSignatureVisitor + ?Sized,
{
    let mut signature_iter = reader.signature().chars().peekable();

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

            accept_type(&mut signature_iter, visitor.visit_parameter_type())?;
        }
    }

    // Return type
    accept_type(&mut signature_iter, visitor.visit_return_type())?;

    // Exception types (opt)
    if signature_iter.next_if_eq(&'^').is_some() {
        accept_type(&mut signature_iter, visitor.visit_exception_type())?;
    }
    
    visitor.visit_end();

    // Strict check
    strict_check_iter_empty(&mut signature_iter)
}

fn accept_formal_type_parameters<SI, FTV>(
    signature_iter: &mut Peekable<SI>,
    formal_type_visitable: &mut Box<FTV>,
) -> KapiResult<()>
where
    SI: Iterator<Item = char> + Clone,
    FTV: FormalTypeParameterVisitable + ?Sized,
{
    if signature_iter.next_if_eq(&'<').is_some() {
        loop {
            let formal_type_parameter = signature_iter.by_ref().take_while(|c| *c != ':').collect();
            let mut formal_type_visitor = formal_type_visitable.visit_formal_type_parameter(&formal_type_parameter);

            let char = signature_iter.peek().ok_or(KapiError::ClassParseError(String::from(
                "Attempt to parse class bound for formal type parameter in signature but parameters are not enclosed by `>`"
            )))?;

            // Class bound
            match *char {
                'L' | '[' | 'T' => {
                    accept_class_type(signature_iter, formal_type_visitor.visit_class_bound())?
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
                    accept_class_type(signature_iter, formal_type_visitor.visit_interface_bound())?;
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

fn accept_type<SI, CTV>(signature_iter: &mut Peekable<SI>, mut visitor: Box<CTV>) -> KapiResult<()>
where
    SI: Iterator<Item = char> + Clone,
    CTV: TypeVisitor + ?Sized,
{
    let char = signature_iter
        .next()
        .ok_or(KapiError::ClassParseError(String::from("Expected primitive type, array type, class type, or type variable descriptor, but got nothing")))?;

    match char {
        'Z' | 'C' | 'B' | 'S' | 'I' | 'F' | 'J' | 'D' | 'V' => {
            visitor.visit_base_type(&char);
            Ok(())
        }
        '[' => accept_type(signature_iter, visitor.visit_array_type()),
        'T' => {
            let type_variable = signature_iter
                .take_while(|c| *c != ';')
                .collect();
            visitor.visit_type_variable(&type_variable);
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
    mut visitor: Box<CTV>,
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
                        '+' => accept_class_type(signature_iter, visitor.visit_type_argument_wildcard(Wildcard::EXTENDS))?,
                        '-' => accept_class_type(signature_iter, visitor.visit_type_argument_wildcard(Wildcard::SUPER))?,
                        _ => accept_class_type(signature_iter, visitor.visit_type_argument_wildcard(Wildcard::INSTANCEOF))?,
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

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::asm::signature::{ClassSignatureReader, ClassSignatureReaderImpl, ClassSignatureVisitor, TypeVisitor, FieldSignatureReader, FieldSignatureReaderImpl, FormalTypeParameterVisitable, FormalTypeParameterVisitor};
    use crate::error::KapiResult;

    use super::{MethodSignatureReader, MethodSignatureReaderImpl, SignatureVisitorImpl};

    #[rstest]
    #[case("<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")]
    fn test_class_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let mut visitor = Box::new(SignatureVisitorImpl::default()) as _;
        let mut reader = ClassSignatureReaderImpl::new(signature.to_string());

        reader.accept(&mut visitor)?;

        Ok(())
    }

    #[rstest]
    #[case("Ljava/lang/Object;")]
    #[case("TT;")]
    fn test_field_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let mut visitor = Box::new(SignatureVisitorImpl::default()) as _;
        let mut reader = FieldSignatureReaderImpl::new(signature.to_string());

        reader.accept(&mut visitor)?;

        Ok(())
    }

    #[rstest]
    #[case("<T:Ljava/lang/Object;>(Z[[Z)Ljava/lang/Object;^Ljava/lang/Exception;")]
    fn test_method_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let mut visitor = Box::new(SignatureVisitorImpl::default()) as _;
        let mut reader = MethodSignatureReaderImpl::new(signature.to_string());

        reader.accept(&mut visitor)?;

        Ok(())
    }
}
