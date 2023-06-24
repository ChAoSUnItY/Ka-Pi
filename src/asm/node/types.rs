use serde::{Deserialize, Serialize};
use crate::asm::node::signature::Wildcard;
use crate::error::{KapiError, KapiResult};

/// Data representation of Type in signatures.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Type {
    BaseType(BaseType),
    Array(Box<Type>),
    Class(String),
    InnerClass(String),
    TypeVariable(String),
    TypeArgument,
    WildcardTypeArgument(Wildcard, Box<Type>),
    /// Unknown is only used in internal code for placeholder usage, you should not see it appears
    /// in returned data structure.
    Unknown,
}

impl Default for Type {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Data representation of base type in descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum BaseType {
    Boolean = b'Z',
    Byte = b'B',
    Short = b'S',
    Int = b'I',
    Long = b'J',
    Float = b'F',
    Double = b'D',
    Void = b'V',
}

impl TryFrom<char> for BaseType {
    type Error = KapiError;

    fn try_from(value: char) -> KapiResult<Self> {
        match value {
            'Z' => Ok(Self::Boolean),
            'B' => Ok(Self::Byte),
            'S' => Ok(Self::Short),
            'I' => Ok(Self::Int),
            'J' => Ok(Self::Long),
            'F' => Ok(Self::Float),
            'D' => Ok(Self::Double),
            'V' => Ok(Self::Void),
            _ => Err(KapiError::ArgError(format!(
                "Unexpected char `{value}` for base type"
            ))),
        }
    }
}

impl TryFrom<&char> for BaseType {
    type Error = KapiError;

    fn try_from(value: &char) -> KapiResult<Self> {
        TryFrom::<char>::try_from(*value)
    }
}
