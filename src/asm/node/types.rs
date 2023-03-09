use serde::{Deserialize, Serialize};
use crate::error::{KapiError, KapiResult};

/// Data representation of base type in descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum BaseType {
    Boolean = 'Z' as u8,
    Byte = 'B' as u8,
    Short = 'S' as u8,
    Int = 'I' as u8,
    Long = 'J' as u8,
    Float = 'F' as u8,
    Double = 'D' as u8,
    Void = 'V' as u8,
}

impl Into<char> for BaseType {
    fn into(self) -> char {
        self as u8 as char
    }
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
                "Unexpected char `{}` for base type",
                value
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
