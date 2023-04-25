use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;
use std::string::ToString;

use itertools::{Itertools, PeekingNext};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::error::KapiResult;
use crate::{asm::byte_vec::ByteVec, error::KapiError};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Void,
    Boolean,
    Char,
    Byte,
    Short,
    Int,
    Float,
    Long,
    Double,
    Array(Box<Type>),
    ObjectRef(String),
    /// [Type::Null] should not be ever used to emit bytecode, it's intended for stack validation usage
    Null,
}

impl Type {
    pub(crate) const OBJECT_TYPE: Self = Self::ObjectRef("java/lang/Object".to_string());
    pub(crate) const STRING_TYPE: Self = Self::ObjectRef("java/lang/String".to_string());
    
    pub fn array_type(depth: usize, inner_type: Self) -> Self {
        let mut depth = depth - 1;
        let mut array_type = Type::Array(Box::new(inner_type));
        
        while depth >= 1 {
            array_type = Type::Array(Box::new(array_type));
        }

        array_type
    }
    
    pub fn object_type() -> Self {
        Self::OBJECT_TYPE
    }
    
    pub(crate) fn from_method_descriptor(method_descriptor: &str) -> KapiResult<(Vec<Self>, Self)> {
        let mut descriptor_iter = method_descriptor.chars().peekable();
        let mut argument_types = Vec::new();

        if descriptor_iter.peeking_next(|char| *char == '(').is_none() {
            return Err(KapiError::ArgError(format!(
                "Expected `(` for descriptor arguments start"
            )));
        }

        while descriptor_iter.peek().map_or(false, |char| *char != ')') {
            let argument_type =
                Self::from_descriptor_iter(method_descriptor, &mut descriptor_iter, false, true)?;

            argument_types.push(argument_type);
        }

        if descriptor_iter.peeking_next(|char| *char == ')').is_none() {
            return Err(KapiError::ArgError(format!(
                "Expected `)` for descriptor arguments ending"
            )));
        }

        let return_type =
            Self::from_descriptor_iter(method_descriptor, &mut descriptor_iter, true, false)?;

        if descriptor_iter.next().is_some() {
            Err(KapiError::StateError(format!(
                "Expected type descriptor reached end but found `{}`",
                descriptor_iter.collect::<String>()
            )))
        } else {
            Ok((argument_types, return_type))
        }
    }

    pub(crate) fn from_descriptor_no_void(type_descriptor: &str) -> KapiResult<Self> {
        Self::from_descriptor(type_descriptor, false)
    }

    pub(crate) fn from_descriptor(
        type_descriptor: &str,
        allow_void_type: bool,
    ) -> KapiResult<Self> {
        Self::from_descriptor_iter(
            type_descriptor,
            &mut type_descriptor.chars().peekable(),
            allow_void_type,
            false,
        )
    }

    fn from_descriptor_iter(
        type_descriptor: &str,
        descriptor_iter: &mut Peekable<Chars>,
        allow_void_type: bool,
        streaming: bool,
    ) -> KapiResult<Self> {
        let mut array_dim = 0;
        let mut result_type = None;

        while result_type.is_none() || (!streaming && descriptor_iter.peek().is_some()) {
            let current_char = descriptor_iter.next();

            if let None = current_char {
                return Err(KapiError::StateError(format!(
                    "Incomplete type descriptor `{}`",
                    type_descriptor
                )));
            }

            let current_char = current_char.unwrap();

            match current_char {
                primitive_type_char if PRIMITIVE_DESCRIPTORS.contains(current_char) => {
                    result_type = Some(match primitive_type_char {
                        'V' => {
                            if !allow_void_type {
                                return Err(KapiError::StateError(format!("Expected non-void type but got type descriptor `V` which is void type")));
                            }

                            Self::Void
                        }
                        'Z' => Self::Boolean,
                        'B' => Self::Byte,
                        'S' => Self::Short,
                        'I' => Self::Int,
                        'F' => Self::Float,
                        'J' => Self::Long,
                        'D' => Self::Double,
                        _ => unreachable!(),
                    });
                }
                '[' => {
                    array_dim += 1;
                    continue; // we ignore while's condition since the type is not fully resolved yet
                }
                'L' => {
                    let object_ref_type = descriptor_iter
                        .by_ref()
                        .peeking_take_while(|&char| char != ';')
                        .collect::<String>();

                    match descriptor_iter.next() {
                        Some(char) if char == ';' => {
                            result_type = Some(Self::ObjectRef(object_ref_type));
                        }
                        _ => return Err(KapiError::StateError(format!("Expected character `;` for object ref type descriptor ending but found `{}`", descriptor_iter.collect::<String>()))),
                    }
                }
                _ => {
                    return Err(KapiError::StateError(format!(
                        "Unexpected type descriptor format `{}{}`",
                        current_char,
                        descriptor_iter.collect::<String>()
                    )));
                }
            }
        }

        for _ in 0..array_dim {
            result_type = result_type.map(|inner_type| Self::Array(Box::new(inner_type)));
        }

        if !streaming && descriptor_iter.peek().is_some() {
            Err(KapiError::StateError(format!(
                "Expected type descriptor reached end but found `{}`",
                descriptor_iter.collect::<String>()
            )))
        } else if let Some(result_type) = result_type {
            Ok(result_type)
        } else {
            Err(KapiError::StateError(format!(
                "Invalid type descriptor `{}`",
                type_descriptor
            )))
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Long | Self::Double => 2,
            _ => 1,
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Void => "void".into(),
            Type::Boolean => "boolean".into(),
            Type::Char => "char".into(),
            Type::Byte => "byte".into(),
            Type::Short => "short".into(),
            Type::Int => "int".into(),
            Type::Float => "float".into(),
            Type::Long => "long".into(),
            Type::Double => "double".into(),
            Type::Array(inner_typ) => format!("{}[]", inner_typ.to_string()),
            Type::ObjectRef(type_ref) => type_ref.clone(),
            Type::Null => "null".into(),
        }
    }
}

pub const VOID: u8 = 0;

/// The sort of the {@code boolean} type. See {@link #getSort}.
pub const BOOLEAN: u8 = 1;

/// The sort of the {@code char} type. See {@link #getSort}.
pub const CHAR: u8 = 2;

/// The sort of the {@code byte} type. See {@link #getSort}.
pub const BYTE: u8 = 3;

/// The sort of the {@code short} type. See {@link #getSort}.
pub const SHORT: u8 = 4;

/// The sort of the {@code int} type. See {@link #getSort}.
pub const INT: u8 = 5;

/// The sort of the {@code float} type. See {@link #getSort}.
pub const FLOAT: u8 = 6;

/// The sort of the {@code long} type. See {@link #getSort}.
pub const LONG: u8 = 7;

/// The sort of the {@code double} type. See {@link #getSort}.
pub const DOUBLE: u8 = 8;

/// The sort of array reference types. See {@link #getSort}.
pub const ARRAY: u8 = 9;

/// The sort of object reference types. See {@link #getSort}.
pub const OBJECT: u8 = 10;

/// The sort of method types. See {@link #getSort}.
pub const METHOD: u8 = 11;

/// The (private) sort of object reference types represented with an internal name.
const INTERNAL: u8 = 12;

/// The descriptors of the primitive types.
const PRIMITIVE_DESCRIPTORS: &'static str = "VZCBSIFJD";

#[cfg(test)]
mod test {
    use crate::asm::types::Type;
    use crate::error::KapiResult;

    #[test]
    fn test_type_conversion() -> KapiResult<()> {
        assert_eq!(Type::Int, Type::from_descriptor_no_void("I")?);
        assert_eq!(
            Type::Array(Box::new(Type::Int)),
            Type::from_descriptor_no_void("[I")?
        );
        assert_eq!(
            Type::ObjectRef("java/lang/Object".to_string()),
            Type::from_descriptor_no_void("Ljava/lang/Object;")?
        );

        Ok(())
    }
}
