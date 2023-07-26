use crate::node::error::{NodeResError, NodeResResult};
use serde::{Deserialize, Serialize};

use crate::visitor::signature::{FormalTypeParameterVisitor, SignatureVisitor, TypeVisitor};
use crate::visitor::Visitable;

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

impl<SCTV, ITV, FTV, PTV, RTV, ETV, FTPV, SV> Visitable<SV> for Signature
where
    SCTV: TypeVisitor,
    ITV: TypeVisitor,
    FTV: TypeVisitor,
    PTV: TypeVisitor,
    RTV: TypeVisitor,
    ETV: TypeVisitor,
    FTPV: FormalTypeParameterVisitor,
    SV: SignatureVisitor<
        SCTV = SCTV,
        ITV = ITV,
        FTV = FTV,
        PTV = PTV,
        RTV = RTV,
        ETV = ETV,
        FTPV = FTPV,
    >,
{
    fn visit(&self, visitor: &mut SV) {
        match self {
            Signature::Class {
                formal_type_parameters,
                super_class,
                interfaces,
            } => {
                visitor.visit_formal_type_parameters(formal_type_parameters);

                for formal_type_parameter in formal_type_parameters {
                    let mut formal_type_parameter_visitor =
                        visitor.visit_formal_type_parameter(formal_type_parameter);

                    formal_type_parameter.visit(&mut formal_type_parameter_visitor);
                }

                let mut super_class_visitor = visitor.visit_super_class(super_class);

                super_class.visit(&mut super_class_visitor);

                visitor.visit_interfaces(interfaces);

                for interface in interfaces {
                    let mut interface_type_visitor = visitor.visit_interface(interface);

                    interface.visit(&mut interface_type_visitor);
                }
            }
            Signature::Field { field_type } => {
                let mut field_type_visitor = visitor.visit_field_type(field_type);

                field_type.visit(&mut field_type_visitor);
            }
            Signature::Method {
                formal_type_parameters,
                parameter_types,
                return_type,
                exception_types,
            } => {
                visitor.visit_formal_type_parameters(formal_type_parameters);

                for formal_type_parameter in formal_type_parameters {
                    let mut formal_type_parameter_visitor =
                        visitor.visit_formal_type_parameter(formal_type_parameter);

                    formal_type_parameter.visit(&mut formal_type_parameter_visitor);
                }

                visitor.visit_parameter_types(parameter_types);

                for parameter_type in parameter_types {
                    let mut parameter_type_visitor = visitor.visit_parameter_type(parameter_type);

                    parameter_type.visit(&mut parameter_type_visitor);
                }

                let mut return_type_visitor = visitor.visit_return_type(return_type);

                return_type.visit(&mut return_type_visitor);

                visitor.visit_exception_types(exception_types);

                for exception_type in exception_types {
                    let mut exception_type_visitor = visitor.visit_exception_type(exception_type);

                    exception_type.visit(&mut exception_type_visitor);
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

impl<CBTV, IBTV, FTPV> Visitable<FTPV> for FormalTypeParameter
where
    CBTV: TypeVisitor,
    IBTV: TypeVisitor,
    FTPV: FormalTypeParameterVisitor<CBTV = CBTV, IBTV = IBTV>,
{
    fn visit(&self, visitor: &mut FTPV) {
        if let Some(class_bound) = &self.class_bound {
            let mut class_bound_type_visitor = visitor.visit_class_bound(class_bound);

            class_bound.visit(&mut class_bound_type_visitor);
        }

        visitor.visit_interface_bounds(&self.interface_bounds);

        for interface_bound in &self.interface_bounds {
            let mut interface_bound_type_visitor = visitor.visit_interface_bound(interface_bound);

            interface_bound.visit(&mut interface_bound_type_visitor);
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
    fn visit(&self, visitor: &mut TV) {
        visitor.visit_type_argument(self);

        match self {
            TypeArgument::Bounded {
                wildcard_indicator,
                bounded_type,
            } => {
                visitor.visit_type_argument_bounded(wildcard_indicator, bounded_type);

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

impl<TV> Visitable<TV> for SignatureType
where
    TV: TypeVisitor,
{
    fn visit(&self, visitor: &mut TV) {
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
    fn visit(&self, visitor: &mut TV) {
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
    fn visit(&self, visitor: &mut TV) {
        match self {
            ReferenceType::Array(inner_type) => {
                visitor.visit_array_type(inner_type);

                let mut inner_type = inner_type.0.as_ref();

                while let SignatureType::ReferenceType(ReferenceType::Array(typ)) = inner_type {
                    inner_type = typ.0.as_ref();
                }

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
    fn visit(&self, visitor: &mut TV) {
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
    fn visit(&self, visitor: &mut TV) {
        visitor.visit_class_type(self);

        for type_argument in &self.type_arguments {
            type_argument.visit(visitor);
        }

        visitor.visit_inner_class_types(&self.inner_classes);

        for (inner_class_name, type_arguments) in &self.inner_classes {
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
    fn visit(&self, visitor: &mut TV) {
        visitor.visit_type_variable(&self.0);
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
    fn visit(&self, visitor: &mut TV) {
        visitor.visit_base_type(self);
    }
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
