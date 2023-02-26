use std::str::CharIndices;

use either::Either;
use unicode_segmentation::UnicodeSegmentation;

use crate::error::{KapiError, KapiResult};

const EXTENDS: char = '+';
const SUPER: char = '-';
const INSTANCEOF: char = '=';

#[derive(Clone, Copy)]
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

pub trait SignatureVisitor {
    fn builder(&mut self) -> &mut String;
    fn visit_formal_type_parameter(&mut self, name: String) {}
    fn visit_class_bound(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_interface_bound(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_super_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_interface(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_parameter_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_return_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_exception_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_base_type(&mut self, descriptor: char) {}
    fn visit_type_variable(&mut self, name: String) {}
    fn visit_array_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_class_type(&mut self, name: String) {}
    fn visit_inner_class_type(&mut self, name: String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(
        &mut self,
        wild_card: Wildcard,
    ) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_end(&self) {}
}

pub trait SignatureReader {
    fn signautre(&self) -> &String;
    fn accept(&self, mut visitor: Box<dyn SignatureVisitor>) {}
    fn accept_type(&self, mut visitor: Box<dyn SignatureVisitor>) {}
}

pub fn parse_type<'original>(
    signature: &String,
    start_offset: usize,
    mut visitor: Box<dyn SignatureVisitor + 'original>,
) -> KapiResult<usize> {
    let mut chars = signature.chars().collect::<Vec<_>>();
    let mut offset = start_offset;
    let char = chars.get(offset).ok_or(KapiError::ArgError(format!(
        "Unable to get character from given offset {}",
        start_offset
    )))?;
    offset += 1;

    match char {
        'Z' | 'C' | 'B' | 'S' | 'I' | 'F' | 'J' | 'D' | 'V' => {
            visitor.visit_base_type(*char);
            Ok(offset)
        }
        '[' => parse_type(signature, offset, visitor.visit_array_type()),
        'T' => {
            let mut name_len = 0;
            let name_segment = chars
                .iter()
                .skip(offset)
                .take_while(|c| {
                    name_len += 1;
                    **c != ';'
                })
                .collect::<String>();
            let end_offset = offset + name_len;
            visitor.visit_type_variable(name_segment);
            Ok(end_offset + 1)
        }
        'L' => {
            let mut start = offset;
            let mut visited = false;
            let mut inner = false;

            loop {
                let char = chars.get(offset).ok_or(KapiError::StateError(
                    "Expected character after object descriptor prefix `L` but got nothing",
                ))?;
                offset += 1;

                match char {
                    '.' | ';' => {
                        if !visited {
                            let name = chars[start..offset - 1].into_iter().collect();

                            if inner {
                                visitor.visit_inner_class_type(name);
                            } else {
                                visitor.visit_class_type(name);
                            }
                        }

                        if *char == ';' {
                            visitor.visit_end();
                            break;
                        }

                        start = offset;
                        visited = false;
                        inner = true;
                    }
                    '<' => {
                        let name = chars[start..offset - 1].into_iter().collect();

                        if inner {
                            visitor.visit_inner_class_type(name);
                        } else {
                            visitor.visit_class_type(name);
                        }

                        visited = true;

                        while *char != '>' {
                            match char {
                                '*' => {
                                    offset += 1;
                                    visitor.visit_type_argument();
                                }
                                '+' | '-' => {
                                    offset = parse_type(
                                        signature,
                                        offset + 1,
                                        visitor.visit_type_argument_wildcard(char.try_into()?),
                                    )?;
                                }
                                _ => {
                                    offset = parse_type(
                                        signature,
                                        offset,
                                        visitor.visit_type_argument_wildcard(Wildcard::INSTANCEOF),
                                    )?;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            Ok(offset)
        }
        _ => Err(KapiError::ArgError(format!(
            "Unable to match current character {} when parsing type",
            start_offset
        ))),
    }
}

#[derive(Debug)]
pub struct SignatureWriterImpl<'original> {
    builder: Either<String, &'original mut String>,
}

impl<'original> SignatureVisitor for SignatureWriterImpl<'original> {
    fn builder(&mut self) -> &mut String {
        match &mut self.builder {
            Either::Left(owned_builder) => owned_builder,
            Either::Right(ref_builder) => ref_builder,
        }
    }
}

impl<'a> Default for SignatureWriterImpl<'a> {
    fn default() -> Self {
        Self {
            builder: Either::Left(String::new()),
        }
    }
}
