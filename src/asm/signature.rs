use either::Either;

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

pub trait SignatureVisitor<'original: 'referenced, 'referenced> {
    fn builder(&mut self) -> &mut String;
    fn visit_formal_type_parameter(&mut self, name: String) {}
    fn visit_class_bound(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_interface_bound(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_super_type(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_interface(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_parameter_type(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_return_type(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_exception_type(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_base_type(&mut self, descriptor: char) {}
    fn visit_type_variable(&mut self, name: String) {}
    fn visit_array_type(&'original mut self) -> Box<dyn SignatureVisitor<'original, 'referenced> + 'referenced> {
        Box::new(SignatureWriterImpl {
            builder: either::Right(self.builder()),
        })
    }
    fn visit_class_type(&mut self, name: String) {}
    fn visit_inner_class_type(&mut self, name: String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(&mut self, wild_card: Wildcard) {}
    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub trait SignatureReader {
    fn signautre(&self) -> &String;
    fn accept(&self, mut visitor: Box<dyn SignatureVisitor>) {}
    fn accept_type(&self, mut visitor: Box<dyn SignatureVisitor>) {}
}

fn parse_type(
    signature: &String,
    start_offset: usize,
    mut visitor: Box<dyn SignatureVisitor>,
) -> KapiResult<usize> {
    let char = signature
        .chars()
        .nth(start_offset)
        .ok_or(KapiError::ArgError(format!(
            "Unable to get character from given offset {}",
            start_offset
        )))?;

    match char {
        'Z' | 'C' | 'B' | 'S' | 'I' | 'F' | 'J' | 'D' | 'V' => {
            visitor.visit_base_type(char);
            Ok(start_offset)
        }
        '[' => {
            parse_type(signature, start_offset + 1, visitor.visit_array_type())
        }
        _ => Err(KapiError::ArgError(format!(
            "Unable to match current character {} when parsing type",
            start_offset
        ))),
    }
}

#[derive(Debug)]
pub struct SignatureWriterImpl<'a> {
    builder: Either<String, &'a mut String>,
}

impl<'original: 'referenced, 'referenced> SignatureVisitor<'original, 'referenced> for SignatureWriterImpl<'original> {
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
