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

pub trait ClassSignatureVisitor {
    fn visit_formal_type_parameter(&mut self, name: &String) {}
    fn visit_super_class(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_interface(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

pub trait FieldSignatureVisitor {
    fn visit_field_type(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

pub trait MethodSignatureVisitor {
    fn visit_formal_type_parameter(&mut self, name: &String) {}
    fn visit_parameter_type(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_return_type(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_exception_type(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
}

pub trait ClassFormalTypeParameterVisitor {
    fn visit_identifier(&mut self, name: &String) {}
    fn visit_class_bound(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_interface_bound(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

pub trait ClassTypeVisitor {
    fn visit_base_type(&mut self, char: &char) {}
    fn visit_array_type(&mut self) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_class_type(&mut self, name: &String) {}
    fn visit_inner_class_type(&mut self, name: &String) {}
    fn visit_type_variable(&mut self, name: &String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(&mut self, wildcard: Wildcard) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct SignatureVisitorImpl {}

impl ClassSignatureVisitor for SignatureVisitorImpl {}
impl FieldSignatureVisitor for SignatureVisitorImpl {}
impl MethodSignatureVisitor for SignatureVisitorImpl {}
impl ClassFormalTypeParameterVisitor for SignatureVisitorImpl {}
impl ClassTypeVisitor for SignatureVisitorImpl {}

pub trait ClassSignatureReader {
    fn signautre(&self) -> &String;
    fn accept(&mut self, mut visitor: Box<dyn ClassSignatureVisitor>) -> KapiResult<()> {
        accept_class_signature_visitor(self, visitor)
    }
}

pub fn accept_class_signature_visitor<CSR, CSV>(
    reader: &mut CSR,
    mut visitor: Box<CSV>,
) -> KapiResult<()>
where
    CSR: ClassSignatureReader + ?Sized,
    CSV: ClassSignatureVisitor + ?Sized,
{
    let mut signature_iter = reader.signautre().chars().peekable();

    // Formal type parameters
    if signature_iter.next_if_eq(&'<').is_some() {
        loop {
            let formal_type_parameter = signature_iter.by_ref().take_while(|c| *c != ':').collect();

            visitor.visit_formal_type_parameter(&formal_type_parameter);

            if signature_iter.peek().is_none() {
                return Err(KapiError::ClassParseError(String::from(
                    "Attempt to parse formal type parameter in signature but parameters are not enclosed by `>`"
                )));
            } else if signature_iter.next_if_eq(&'>').is_some() {
                break;
            }
        }
    }

    // Super class type
    accept_class_type(&mut signature_iter, visitor.visit_super_class())?;

    // Interface class types
    while signature_iter.peek().is_some() {
        accept_class_type(&mut signature_iter, visitor.visit_interface())?;
    }

    // Strict check
    if signature_iter.peek().is_some() {
        Err(KapiError::ClassParseError(format!(
            "Expected nothing after fully parsed but got `{}`",
            signature_iter.collect::<String>()
        )))
    } else {
        Ok(())
    }
}

fn accept_field_signature_visitor<CSR, FSV>(
    reader: &mut CSR,
    mut vititor: Box<FSV>,
) -> KapiResult<()>
where
    CSR: ClassSignatureReader + ?Sized,
    FSV: FieldSignatureVisitor + ?Sized,
{
    let mut signature_iter = reader.signautre().chars().peekable();

    // Field type
    accept_type(&mut signature_iter, vititor.visit_field_type())?;

    // Strict check
    if signature_iter.peek().is_some() {
        Err(KapiError::ClassParseError(format!(
            "Expected nothing after fully parsed but got `{}`",
            signature_iter.collect::<String>()
        )))
    } else {
        Ok(())
    }
}

fn accpet_method_signature_visitor<CSR, MSV>(
    reader: &mut CSR,
    mut visitor: Box<MSV>,
) -> KapiResult<()>
where
    CSR: ClassSignatureReader + ?Sized,
    MSV: MethodSignatureVisitor + ?Sized,
{
    let mut signature_iter = reader.signautre().chars().peekable();

    // Formal type parameters
    if signature_iter.next_if_eq(&'<').is_some() {
        loop {
            let formal_type_parameter = signature_iter.by_ref().take_while(|c| *c != ':').collect();

            visitor.visit_formal_type_parameter(&formal_type_parameter);

            if signature_iter.peek().is_none() {
                return Err(KapiError::ClassParseError(String::from(
                    "Attempt to parse formal type parameter in signature but parameters are not enclosed by `>`"
                )));
            } else if signature_iter.next_if_eq(&'>').is_some() {
                break;
            }
        }
    }

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

    // Strict check
    if signature_iter.peek().is_some() {
        Err(KapiError::ClassParseError(format!(
            "Expected nothing after fully parsed but got `{}`",
            signature_iter.collect::<String>()
        )))
    } else {
        Ok(())
    }
}

fn accept_type<SI, CTV>(signature_iter: &mut Peekable<SI>, mut visitor: Box<CTV>) -> KapiResult<()>
where
    SI: Iterator<Item = char>,
    CTV: ClassTypeVisitor + ?Sized,
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
    SI: Iterator<Item = char>,
    CTV: ClassTypeVisitor + ?Sized,
{
    let mut visited = false;
    let mut inner = false;

    loop {
        let char = signature_iter.next().ok_or(KapiError::ClassParseError(String::from(
            "Expected any character after class type or type variable descriptor prefix `L` but got nothing",
        )))?;
        let name = signature_iter
            .peeking_take_while(|c| *c != '.' || *c != ';' || *c != '<')
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
