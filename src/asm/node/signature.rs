use serde::{Deserialize, Serialize};

use crate::asm::visitor::signature::{FormalTypeParameterVisitor, SignatureVisitor, TypeVisitor};
use crate::asm::visitor::Visitable;
use crate::error::{KapiError, KapiResult};

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

impl<TV, FTPV, SV> Visitable<SV> for Signature
where
    TV: TypeVisitor,
    FTPV: FormalTypeParameterVisitor,
    SV: SignatureVisitor<TV = TV, FTPV = FTPV>,
{
    fn visit(&mut self, visitor: &mut SV) {
        match self {
            Signature::Class {
                formal_type_parameters,
                super_class,
                interfaces,
            } => {
                for formal_type_parameter in formal_type_parameters {
                    let mut formal_type_parameter_visitor = visitor
                        .visit_formal_type_parameter(&mut formal_type_parameter.parameter_name);

                    formal_type_parameter.visit(&mut formal_type_parameter_visitor);
                }

                super_class.visit(&mut visitor.visit_super_class());

                for interface in interfaces {
                    interface.visit(&mut visitor.visit_interface());
                }
            }
            Signature::Field { field_type } => {
                field_type.visit(&mut visitor.visit_field_type());
            }
            Signature::Method {
                formal_type_parameters,
                parameter_types,
                return_type,
                exception_types,
            } => {
                for formal_type_parameter in formal_type_parameters {
                    let mut formal_type_parameter_visitor = visitor
                        .visit_formal_type_parameter(&mut formal_type_parameter.parameter_name);

                    formal_type_parameter.visit(&mut formal_type_parameter_visitor);
                }

                for parameter_type in parameter_types {
                    parameter_type.visit(&mut visitor.visit_parameter_type());
                }

                return_type.visit(&mut visitor.visit_return_type());

                for exception_type in exception_types {
                    exception_type.visit(&mut visitor.visit_exception_type());
                }
            }
        }
    }
}

/// Data representation of formal type parameter in signatures.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FormalTypeParameter {
    pub parameter_name: String,
    pub class_bound: Option<ClassType>,
    pub interface_bounds: Vec<ClassType>,
}

impl<TV, FTPV> Visitable<FTPV> for FormalTypeParameter
where
    TV: TypeVisitor,
    FTPV: FormalTypeParameterVisitor<TV = TV>,
{
    fn visit(&mut self, visitor: &mut FTPV) {
        if let Some(class_bound) = &mut self.class_bound {
            class_bound.visit(&mut visitor.visit_class_bound());
        }

        for interface_bound in &mut self.interface_bounds {
            interface_bound.visit(&mut visitor.visit_interface_bound());
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeArgument {
    Bounded {
        wildcard_indicator: WildcardIndicator,
        bounded_type: ReferenceType,
    },
    Wildcard,
}

impl<TV> Visitable<TV> for TypeArgument
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        match self {
            TypeArgument::Bounded {
                wildcard_indicator,
                bounded_type,
            } => {
                visitor.visit_type_argument_bounded(wildcard_indicator);

                bounded_type.visit(visitor);
            }
            TypeArgument::Wildcard => {
                visitor.visit_type_argument_wildcard();
            }
        }
    }
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
    type Error = KapiError;

    fn try_from(value: char) -> KapiResult<Self> {
        match value {
            EXTENDS => Ok(WildcardIndicator::EXTENDS),
            SUPER => Ok(WildcardIndicator::SUPER),
            _ => Err(KapiError::ArgError(format!(
                "Character {value} cannot be converted into Wildcard"
            ))),
        }
    }
}

impl TryFrom<&char> for WildcardIndicator {
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SignatureType {
    BaseType(BaseType),
    ReferenceType(ReferenceType),
}

impl<TV> Visitable<TV> for SignatureType
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        match self {
            SignatureType::BaseType(base_type) => visitor.visit_base_type(base_type),
            SignatureType::ReferenceType(reference_type) => {
                reference_type.visit(visitor);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ThrowsType {
    Class(ClassType),
    TypeVariable(TypeVariable),
}

impl<TV> Visitable<TV> for ThrowsType
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        match self {
            ThrowsType::Class(class_type) => {
                class_type.visit(visitor);
            }
            ThrowsType::TypeVariable(type_variable) => {
                type_variable.visit(visitor);
            }
        }
    }
}

/// Data representation of Type in signatures.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReferenceType {
    Array(ArrayType),
    Class(ClassType),
    TypeVariable(TypeVariable),
}

impl<TV> Visitable<TV> for ReferenceType
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        match self {
            ReferenceType::Array(inner_type) => {
                visitor.visit_array_type();

                inner_type.visit(visitor);
            }
            ReferenceType::Class(class_type) => {
                class_type.visit(visitor);
            }
            ReferenceType::TypeVariable(type_variable) => {
                type_variable.visit(visitor);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArrayType(pub Box<SignatureType>);

impl<TV> Visitable<TV> for ArrayType
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        self.0.visit(visitor)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassType {
    pub package_path: String,
    pub class_name: String,
    pub type_arguments: Vec<TypeArgument>,
    pub inner_classes: Vec<(String, Vec<TypeArgument>)>,
}

impl<TV> Visitable<TV> for ClassType
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        visitor.visit_class_type(&mut self.package_path, &mut self.class_name);

        for type_argument in &mut self.type_arguments {
            type_argument.visit(visitor);
        }

        for (inner_class_name, type_arguments) in &mut self.inner_classes {
            visitor.visit_inner_class_type(inner_class_name);

            for type_argument in type_arguments {
                type_argument.visit(visitor);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeVariable(pub String);

impl<TV> Visitable<TV> for TypeVariable
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        visitor.visit_type_variable(&mut self.0);
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

impl<TV> Visitable<TV> for BaseType
where
    TV: TypeVisitor,
{
    fn visit(&mut self, visitor: &mut TV) {
        visitor.visit_base_type(self);
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
