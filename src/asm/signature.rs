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
            _ => Err(KapiError::ArgError(format!("Character {} cannot be converted into Wildcard", value))),
        }
    }
}

pub trait SignatureVisitor<'a> {
    fn builder(&mut self) -> &mut String;
    fn visit_formal_type_parameter(&mut self, name: String) {}
    fn visit_class_bound(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_interface_bound(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_super_type(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_interface(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_parameter_type(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_return_type(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_exception_type(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_base_type(&mut self, descriptor: char) {}
    fn visit_type_variable(&mut self, name: String) {}
    fn visit_array_type(&'a mut self) -> Box<dyn SignatureVisitor<'a> + 'a> {
        Box::new(SignatureWriter{ builder: either::Right(self.builder()) })
    }
    fn visit_class_type(&mut self, name: String) {}
    fn visit_inner_class_type(&mut self, name: String) {}
    fn visit_type_argument(&mut self) {}
    fn visit_type_argument_wildcard(&mut self, wild_card: Wildcard) {}
    fn visit_end(self) where Self: Sized {}
}

#[derive(Debug)]
pub struct SignatureWriter<'a> {
    builder: Either<String, &'a mut String>
}

impl<'a> SignatureVisitor<'a> for SignatureWriter<'a> {
    fn builder(&mut self) -> &mut String {
        match &mut self.builder {
            Either::Left(owned_builder) => owned_builder,
            Either::Right(ref_builder) => ref_builder,
        }
    }
}

impl<'a> Default for SignatureWriter<'a> {
    fn default() -> Self {
        Self { builder: Either::Left(String::new()) }
    }
}
