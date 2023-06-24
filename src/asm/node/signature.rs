use serde::{Deserialize, Serialize};
use crate::asm::node::types::{BaseType, Type};

use crate::error::{KapiError, KapiResult};

/// Data representation of signatures, including [`Class`](Signature::Class), [`Field`](Signature::Field),
/// and [`Method`](Signature::Method).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Signature {
    /// Data representation of class signature.
    Class {
        formal_type_parameters: Vec<FormalTypeParameter>,
        super_class: Type,
        interfaces: Vec<Type>,
    },
    /// Data representation of field signature.
    Field { field_type: Type },
    /// Data representation of method signature.
    Method {
        formal_type_parameters: Vec<FormalTypeParameter>,
        parameter_types: Vec<Type>,
        return_type: Type,
        exception_types: Vec<Type>,
    },
}

/// Data representation of formal type parameter in signatures.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FormalTypeParameter {
    parameter_name: String,
    class_bound: Option<Type>,
    interface_bounds: Vec<Type>,
}

const EXTENDS: char = '+';
const SUPER: char = '-';
const INSTANCEOF: char = '=';

/// An enum representation for wildcard indicators, which is used in
/// [`Type::WildcardTypeArgument`] as class
/// type argument bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Wildcard {
    /// Indicates type argument must extends class bound, see java's upper bounds wildcard.
    EXTENDS = EXTENDS as u8,
    /// Indicates type argument must super class bound, see java's lower bounds wildcard.
    SUPER = SUPER as u8,
    /// Indicates type argument must be instance of specified type.
    INSTANCEOF = INSTANCEOF as u8,
}

impl From<Wildcard> for char {
    fn from(value: Wildcard) -> Self {
        value as u8 as char
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
                "Character {value} cannot be converted into Wildcard"
            ))),
        }
    }
}

impl TryFrom<&char> for Wildcard {
    type Error = KapiError;

    fn try_from(value: &char) -> KapiResult<Self> {
        TryFrom::<char>::try_from(*value)
    }
}

impl From<BaseType> for char {
    fn from(value: BaseType) -> Self {
        value as u8 as char
    }
}

#[cfg(test)]
mod test {
    // #[test]
    // fn test_class_signature_with_generic() -> KapiResult<()> {
    //     let class_signature = Signature::class_signature_from_str(
    //         "<T:[Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;",
    //     )?;
    // 
    //     assert_yaml_snapshot!(class_signature);
    // 
    //     Ok(())
    // }
    // 
    // #[test]
    // fn test_field_signature_object() -> KapiResult<()> {
    //     let field_signature = Signature::field_signature_from_str("Ljava/lang/Object;")?;
    // 
    //     assert_yaml_snapshot!(field_signature);
    // 
    //     Ok(())
    // }
    // 
    // #[test]
    // fn test_field_signature_type_variable() -> KapiResult<()> {
    //     let field_signature = Signature::field_signature_from_str("TT;")?;
    // 
    //     assert_yaml_snapshot!(field_signature);
    // 
    //     Ok(())
    // }
    // 
    // #[test]
    // fn test_method_signature_with_generic() -> KapiResult<()> {
    //     let method_signature = Signature::method_signature_from_str(
    //         "<T:Ljava/lang/Object;>(Z[[ZTT;)Ljava/lang/Object;^Ljava/lang/Exception;",
    //     )?;
    // 
    //     assert_yaml_snapshot!(method_signature);
    // 
    //     Ok(())
    // }
}
