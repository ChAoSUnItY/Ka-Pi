use std::{default, str::CharIndices};

use either::Either;
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
    fn visit_formal_type_parameter(&mut self) -> Box<dyn ClassFormalTypeParameterVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
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
    fn visit_formal_type_parameter(&mut self) -> Box<dyn ClassFormalTypeParameterVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
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
    fn visit_identifier(&mut self, name: &str) {}
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
    fn visit_class_type(&mut self, name: &str) {}
    fn visit_inner_class_type(&mut self, name: &str) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(&mut self, wildcard: Wildcard) -> Box<dyn ClassTypeVisitor> {
        Box::new(SignatureVisitorImpl::default())
    }
    fn visit_end(&mut self) {}
}

#[derive(Debug, Default)]
pub struct SignatureVisitorImpl {}

impl ClassSignatureVisitor for SignatureVisitorImpl {}
impl ClassFormalTypeParameterVisitor for SignatureVisitorImpl {}
impl ClassTypeVisitor for SignatureVisitorImpl {}

pub trait ClassSignatureReader {
    fn signautre(&self) -> &String;
    fn accept(&mut self, mut visitor: Box<dyn ClassSignatureVisitor>) -> KapiResult<()> {
        accept_class_visitor(self, visitor)
    }
    // fn accept_type(&mut self, mut visitor: Box<dyn ClassSignatureVisitor>) -> KapiResult<()> {
    //     parse_type(self, 0, &mut visitor).map(|_| ())
    // }
}

pub fn accept_class_visitor<CSR, CSV>(reader: &mut CSR, mut visitor: Box<CSV>) -> KapiResult<()>
where
    CSR: ClassSignatureReader + ?Sized,
    CSV: ClassSignatureVisitor + ?Sized,
{
    let signature_iter = reader.signautre().chars();

    Ok(())
}

// pub fn parse_type<SR, SV>(
//     _self: &SR,
//     start_offset: usize,
//     visitor: &mut Box<SV>,
// ) -> KapiResult<usize>
// where
//     SR: SignatureReader + ?Sized,
//     SV: ClassTypeVisitor + ?Sized,
// {
//     parse_type_chars(&_self.signautre().chars().collect(), start_offset, visitor)
// }

// fn parse_type_chars<SV>(
//     signature: &Vec<char>,
//     start_offset: usize,
//     visitor: &mut Box<SV>,
// ) -> KapiResult<usize>
// where
//     SV: ClassTypeVisitor + ?Sized,
// {
//     let mut offset = start_offset;
//     let char = signature.get(offset).ok_or(KapiError::ArgError(format!(
//         "Unable to get character from given offset {}",
//         start_offset
//     )))?;
//     offset += 1;

//     match char {
//         'Z' | 'C' | 'B' | 'S' | 'I' | 'F' | 'J' | 'D' | 'V' => {
//             visitor.visit_base_type(char);
//             Ok(offset)
//         }
//         '[' => parse_type_chars(signature, offset, &mut visitor.visit_array_type()),
//         'T' => {
//             let mut name_len = 0;
//             let name_segment = signature
//                 .iter()
//                 .skip(offset)
//                 .take_while(|c| {
//                     name_len += 1;
//                     **c != ';'
//                 })
//                 .collect::<String>();
//             let end_offset = offset + name_len;
//             visitor.visit_type_variable(&name_segment);
//             Ok(end_offset + 1)
//         }
//         'L' => {
//             let mut start = offset;
//             let mut visited = false;
//             let mut inner = false;

//             loop {
//                 let char = signature.get(offset).ok_or(KapiError::StateError(
//                     "Expected character after object descriptor prefix `L` but got nothing",
//                 ))?;
//                 offset += 1;

//                 match char {
//                     '.' | ';' => {
//                         if !visited {
//                             let name = &signature[start..offset - 1].into_iter().collect();

//                             if inner {
//                                 visitor.visit_inner_class_type(name);
//                             } else {
//                                 visitor.visit_class_type(name);
//                             }
//                         }

//                         if *char == ';' {
//                             visitor.visit_end();
//                             break;
//                         }

//                         start = offset;
//                         visited = false;
//                         inner = true;
//                     }
//                     '<' => {
//                         let name = &signature[start..offset - 1].into_iter().collect();

//                         if inner {
//                             visitor.visit_inner_class_type(name);
//                         } else {
//                             visitor.visit_class_type(name);
//                         }

//                         visited = true;

//                         while *char != '>' {
//                             match char {
//                                 '*' => {
//                                     offset += 1;
//                                     visitor.visit_type_argument();
//                                 }
//                                 '+' | '-' => {
//                                     offset = parse_type_chars(
//                                         signature,
//                                         offset + 1,
//                                         &mut visitor.visit_type_argument_wildcard(char.try_into()?),
//                                     )?;
//                                 }
//                                 _ => {
//                                     offset = parse_type_chars(
//                                         signature,
//                                         offset,
//                                         &mut visitor
//                                             .visit_type_argument_wildcard(Wildcard::INSTANCEOF),
//                                     )?;
//                                 }
//                             }
//                         }
//                     }
//                     _ => {}
//                 }
//             }

//             Ok(offset)
//         }
//         _ => Err(KapiError::ArgError(format!(
//             "Unable to match current character {} when parsing type",
//             start_offset
//         ))),
//     }
// }

// pub struct SignatureReaderImpl {
//     signature: String,
// }

// impl SignatureReader for SignatureReaderImpl {
//     fn signautre(&self) -> &String {
//         &self.signature
//     }
// }
