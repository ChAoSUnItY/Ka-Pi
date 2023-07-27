use crate::node::error::{NodeResError, NodeResResult};
use serde::{Deserialize, Serialize};

/// Data representation of signatures, including [`Class`](Signature::Class), [`Field`](Signature::Field),
/// and [`Method`](Signature::Method).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Signature {
    /// Data representation of class signature.
    Class {
        formal_type_parameters: Vec<FormalTypeParameter>,
        super_class: ClassType,
        interfaces: Vec<ClassType>,
    },
    /// Data representation of field signature.
    Field { field_type: ReferenceType },
    /// Data representation of method signature.
    Method {
        formal_type_parameters: Vec<FormalTypeParameter>,
        parameter_types: Vec<SignatureType>,
        return_type: SignatureType,
        exception_types: Vec<ThrowsType>,
    },
}

/// Data representation of formal type parameter in signatures.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FormalTypeParameter {
    pub parameter_name: String,
    pub class_bound: Option<ClassType>,
    pub interface_bounds: Vec<ClassType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeArgument {
    Bounded {
        wildcard_indicator: WildcardIndicator,
        bounded_type: ReferenceType,
    },
    Wildcard,
}

const EXTENDS: char = '+';
const SUPER: char = '-';
const INSTANCE_OF: char = '=';

/// An enum representation for Wildcard indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WildcardIndicator {
    /// Indicates type argument must extends class bound, see java's upper bounds Wildcard.
    EXTENDS,
    /// Indicates type argument must super class bound, see java's lower bounds Wildcard.
    SUPER,
    #[allow(non_camel_case_types)]
    /// Indicates type argument must be the type.
    INSTANCE_OF,
}

impl Default for WildcardIndicator {
    fn default() -> Self {
        Self::INSTANCE_OF
    }
}

impl From<WildcardIndicator> for char {
    fn from(value: WildcardIndicator) -> Self {
        match value {
            WildcardIndicator::EXTENDS => EXTENDS,
            WildcardIndicator::SUPER => SUPER,
            WildcardIndicator::INSTANCE_OF => INSTANCE_OF,
        }
    }
}

impl TryFrom<char> for WildcardIndicator {
    type Error = NodeResError;

    fn try_from(value: char) -> NodeResResult<Self> {
        match value {
            EXTENDS => Ok(WildcardIndicator::EXTENDS),
            SUPER => Ok(WildcardIndicator::SUPER),
            _ => Err(NodeResError::InvalidWildcard(value)),
        }
    }
}

impl TryFrom<&char> for WildcardIndicator {
    type Error = NodeResError;

    fn try_from(value: &char) -> NodeResResult<Self> {
        TryFrom::<char>::try_from(*value)
    }
}

impl From<BaseType> for char {
    fn from(value: BaseType) -> Self {
        value as u8 as char
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SignatureType {
    BaseType(BaseType),
    ReferenceType(ReferenceType),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ThrowsType {
    Class(ClassType),
    TypeVariable(TypeVariable),
}

/// Data representation of Type in signatures.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReferenceType {
    Array(ArrayType),
    Class(ClassType),
    TypeVariable(TypeVariable),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArrayType(pub Box<SignatureType>);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassType {
    pub package_path: String,
    pub class_name: String,
    pub type_arguments: Vec<TypeArgument>,
    pub inner_classes: Vec<(String, Vec<TypeArgument>)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeVariable(pub String);

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
    type Error = NodeResError;

    fn try_from(value: char) -> NodeResResult<Self> {
        match value {
            'Z' => Ok(Self::Boolean),
            'B' => Ok(Self::Byte),
            'S' => Ok(Self::Short),
            'I' => Ok(Self::Int),
            'J' => Ok(Self::Long),
            'F' => Ok(Self::Float),
            'D' => Ok(Self::Double),
            'V' => Ok(Self::Void),
            _ => Err(NodeResError::InvalidBaseType(value)),
        }
    }
}

impl TryFrom<&char> for BaseType {
    type Error = NodeResError;

    fn try_from(value: &char) -> NodeResResult<Self> {
        TryFrom::<char>::try_from(*value)
    }
}
