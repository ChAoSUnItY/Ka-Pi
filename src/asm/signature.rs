use std::str::CharIndices;

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

pub trait SignatureVisitor {
    fn boxed<'original>(&'original mut self) -> Box<dyn SignatureVisitor + '_> {
        Box::new(SignatureWriterImpl::from_visitor(self))
    }
    fn builder(&mut self) -> &mut String;
    fn visit_formal_type_parameter(&mut self, name: &String) {}
    fn visit_class_bound(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_interface_bound(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_super_class(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_interface(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_parameter_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_return_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_exception_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_base_type(&mut self, descriptor: char) {}
    fn visit_type_variable(&mut self, name: &String) {}
    fn visit_array_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_class_type(&mut self, name: &String) {}
    fn visit_inner_class_type(&mut self, name: &String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(
        &mut self,
        wild_card: Wildcard,
    ) -> Box<dyn SignatureVisitor + '_> {
        self.boxed()
    }
    fn visit_end(&mut self) {}
}

pub trait SignatureReader {
    fn signautre(&self) -> &String;
    fn accept(&self, mut visitor: Box<dyn SignatureVisitor>) -> KapiResult<()> {
        accept(self, visitor)
    }
    fn accept_type(&self, mut visitor: Box<dyn SignatureVisitor>) -> KapiResult<()> {
        parse_type(self, 0, &mut visitor).map(|_| ())
    }
}

pub fn accept<'original, SR>(_self: &SR, mut visitor: Box<dyn SignatureVisitor>) -> KapiResult<()>
where
    SR: SignatureReader + ?Sized,
{
    let signature = _self.signautre().chars().collect::<Vec<_>>();
    let len = signature.len();
    let mut offset: usize;
    let mut char: &char;

    if signature.first().map_or(false, |c| *c == '>') {
        offset = 2;
        loop {
            let class_bound_start_offset = signature
                .iter()
                .skip(offset)
                .position(|c| *c == ':')
                .ok_or(KapiError::StateError(
                    "Expected class bound in signature but got nothing",
                ))?;
            visitor.visit_formal_type_parameter(
                &signature[offset - 1..class_bound_start_offset]
                    .into_iter()
                    .collect(),
            );

            offset = class_bound_start_offset + 1;
            char = signature
                .get(offset)
                .ok_or(KapiError::StateError("Expected character but got nothing"))?;

            if *char == 'L' || *char == '[' || *char == 'T' {
                offset = parse_type_chars(&signature, offset, &mut visitor)?;
            }

            while *char == ':' {
                char = signature
                    .get(offset)
                    .ok_or(KapiError::StateError("Expected character but got nothing"))?;
                offset += 1;
                offset = parse_type_chars(&signature, offset, &mut visitor.visit_interface())?;
            }

            if *char == '>' {
                break;
            }
        }
    } else {
        offset = 0;
    }

    if signature.get(offset).map_or(false, |c| *c == '(') {
        offset += 1;
        while signature.get(offset).map_or(false, |c| *c == ')') {
            offset = parse_type_chars(&signature, offset, &mut visitor.visit_parameter_type())?;
        }
        offset = parse_type_chars(&signature, offset + 1, &mut visitor.visit_return_type())?;
        while offset < len {
            offset = parse_type_chars(&signature, offset + 1, &mut visitor.visit_exception_type())?;
        }
    } else {
        offset = parse_type_chars(&signature, offset, &mut visitor.visit_super_class())?;
        while offset < len {
            offset = parse_type_chars(&signature, offset, &mut visitor.visit_interface())?;
        }
    }

    Ok(())
}

pub fn parse_type<'original, SR, SV>(
    _self: &SR,
    start_offset: usize,
    visitor: &mut Box<SV>,
) -> KapiResult<usize>
where
    SR: SignatureReader + ?Sized,
    SV: SignatureVisitor + 'original + ?Sized
{
    parse_type_chars(&_self.signautre().chars().collect(), start_offset, visitor)
}

fn parse_type_chars<'original, SV>(
    signature: &Vec<char>,
    start_offset: usize,
    visitor: &mut Box<SV>,
) -> KapiResult<usize> where SV: SignatureVisitor + 'original + ?Sized {
    let mut offset = start_offset;
    let char = signature.get(offset).ok_or(KapiError::ArgError(format!(
        "Unable to get character from given offset {}",
        start_offset
    )))?;
    offset += 1;

    match char {
        'Z' | 'C' | 'B' | 'S' | 'I' | 'F' | 'J' | 'D' | 'V' => {
            visitor.visit_base_type(*char);
            Ok(offset)
        }
        '[' => parse_type_chars(signature, offset, &mut visitor.visit_array_type()),
        'T' => {
            let mut name_len = 0;
            let name_segment = signature
                .iter()
                .skip(offset)
                .take_while(|c| {
                    name_len += 1;
                    **c != ';'
                })
                .collect::<String>();
            let end_offset = offset + name_len;
            visitor.visit_type_variable(&name_segment);
            Ok(end_offset + 1)
        }
        'L' => {
            let mut start = offset;
            let mut visited = false;
            let mut inner = false;

            loop {
                let char = signature.get(offset).ok_or(KapiError::StateError(
                    "Expected character after object descriptor prefix `L` but got nothing",
                ))?;
                offset += 1;

                match char {
                    '.' | ';' => {
                        if !visited {
                            let name = &signature[start..offset - 1].into_iter().collect();

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
                        let name = &signature[start..offset - 1].into_iter().collect();

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
                                    offset = parse_type_chars(
                                        signature,
                                        offset + 1,
                                        &mut visitor.visit_type_argument_wildcard(char.try_into()?),
                                    )?;
                                }
                                _ => {
                                    offset = parse_type_chars(
                                        signature,
                                        offset,
                                        &mut visitor
                                            .visit_type_argument_wildcard(Wildcard::INSTANCEOF),
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

pub struct SignatureReaderImpl {
    signature: String,
}

impl SignatureReader for SignatureReaderImpl {
    fn signautre(&self) -> &String {
        &self.signature
    }
}

#[derive(Debug)]
pub struct SignatureWriterImpl<'original> {
    builder: Either<String, &'original mut String>,
    has_formal: bool,
    has_parameter: bool,
    argument_stack: u16,
}

impl<'original> SignatureWriterImpl<'original> {
    pub fn from_visitor<SV>(visitor: &'original mut SV) -> Self
    where
        SV: SignatureVisitor + ?Sized + 'original,
    {
        Self {
            builder: Either::Right(visitor.builder()),
            has_formal: false,
            has_parameter: false,
            argument_stack: 1,
        }
    }

    // Utility functions

    fn push(&mut self, char: char) {
        self.builder().push(char);
    }

    fn push_str(&mut self, str: &str) {
        self.builder().push_str(str);
    }

    fn end_formals(&mut self) {
        if self.has_formal {
            self.has_formal = false;
            self.push('>');
        }
    }

    fn end_arguments(&mut self) {
        if self.argument_stack == 1 {
            self.push('>');
        }

        self.argument_stack -= 1;
    }
}

impl<'original> SignatureVisitor for SignatureWriterImpl<'original> {
    fn builder(&mut self) -> &mut String {
        match &mut self.builder {
            Either::Left(owned_builder) => owned_builder,
            Either::Right(ref_builder) => ref_builder,
        }
    }

    fn visit_formal_type_parameter(&mut self, name: &String) {
        if !self.has_formal {
            self.has_formal = true;
            self.push('<');
        }
        self.push_str(name);
        self.push(':');
    }

    fn visit_interface_bound(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.push(':');
        self.boxed()
    }

    fn visit_super_class(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.end_formals();
        self.boxed()
    }

    fn visit_parameter_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.end_formals();
        if !self.has_parameter {
            self.has_parameter = true;
            self.push('(');
        }
        self.boxed()
    }

    fn visit_return_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.end_formals();
        if !self.has_parameter {
            self.push('(');
        }
        self.push(')');
        self.boxed()
    }

    fn visit_exception_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.push('^');
        self.boxed()
    }

    fn visit_base_type(&mut self, descriptor: char) {
        self.push(descriptor)
    }

    fn visit_type_variable(&mut self, name: &String) {
        self.push('T');
        self.push_str(name);
        self.push(';');
    }

    fn visit_array_type(&mut self) -> Box<dyn SignatureVisitor + '_> {
        self.push('[');
        self.boxed()
    }

    fn visit_inner_class_type(&mut self, name: &String) {
        self.end_formals();
        self.push('.');
        self.push_str(name);
        self.argument_stack += 1;
    }

    fn visit_type_argument(&mut self) {
        if self.argument_stack == 0 {
            self.argument_stack += 1;
            self.push('<');
        }
        self.push('*');
    }

    fn visit_type_argument_wildcard(
        &mut self,
        wild_card: Wildcard,
    ) -> Box<dyn SignatureVisitor + '_> {
        if self.argument_stack == 0 {
            self.argument_stack += 1;
            self.push('<');
        }

        if wild_card != Wildcard::INSTANCEOF {
            self.push(wild_card.into());
        }
        
        self.boxed()
    }

    fn visit_end(&mut self) {
        self.end_arguments();
        self.push(';');
    }
}

impl<'original> Default for SignatureWriterImpl<'original> {
    fn default() -> Self {
        Self {
            builder: Either::Left(String::with_capacity(20)),
            has_formal: false,
            has_parameter: false,
            argument_stack: 1,
        }
    }
}
