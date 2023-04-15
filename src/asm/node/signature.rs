use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::asm::signature::{
    accept_class_signature_visitor, accept_field_signature_visitor,
    accept_method_signature_visitor, ClassSignatureVisitor, ClassSignatureWriter,
    FieldSignatureVisitor, FieldSignatureWriter, FormalTypeParameterVisitable,
    FormalTypeParameterVisitor, MethodSignatureVisitor, MethodSignatureWriter, TypeVisitor,
};
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

impl Signature {
    /// Converts signature string into [`Signature::Class`](Signature::Class).
    pub fn class_signature_from_str<S>(string: S) -> KapiResult<Self>
    where
        S: Into<String>,
    {
        let string = string.into();
        let mut collector = ClassSignatureCollector::default();

        accept_class_signature_visitor(&string, &mut collector)?;

        collector
            .signature
            .ok_or(KapiError::ClassParseError(format!(
                "Unable to parse class signature from `{}`",
                string
            )))
    }

    /// Converts signature string into [`Signature::Field`](Signature::Field).
    pub fn field_signature_from_str<S>(string: S) -> KapiResult<Self>
    where
        S: Into<String>,
    {
        let string = string.into();
        let mut collector = FieldSignatureCollector::default();

        accept_field_signature_visitor(&string, &mut collector)?;

        collector
            .signature
            .ok_or(KapiError::ClassParseError(format!(
                "Unable to parse field signature from `{}`",
                string
            )))
    }

    /// Converts signature string into [`Signature::Method`](Signature::Method).
    pub fn method_signature_from_str<S>(string: S) -> KapiResult<Self>
    where
        S: Into<String>,
    {
        let string = string.into();
        let mut collector = MethodSignatureCollector::default();

        accept_method_signature_visitor(&string, &mut collector)?;

        collector
            .signature
            .ok_or(KapiError::ClassParseError(format!(
                "Unable to parse method signature from `{}`",
                string
            )))
    }
}

impl TryFrom<ClassSignatureWriter> for Signature {
    type Error = KapiError;

    fn try_from(value: ClassSignatureWriter) -> KapiResult<Self> {
        Signature::class_signature_from_str(value.to_string())
    }
}

impl TryFrom<FieldSignatureWriter> for Signature {
    type Error = KapiError;

    fn try_from(value: FieldSignatureWriter) -> KapiResult<Self> {
        Signature::field_signature_from_str(value.to_string())
    }
}

impl TryFrom<MethodSignatureWriter> for Signature {
    type Error = KapiError;

    fn try_from(value: MethodSignatureWriter) -> KapiResult<Self> {
        Signature::method_signature_from_str(value.to_string())
    }
}

#[derive(Default)]
struct ClassSignatureCollector {
    signature: Option<Signature>,
    formal_type_parameters: Vec<FormalTypeParameter>,
    super_class: Type,
    interfaces: Vec<Type>,
}

impl ClassSignatureVisitor for ClassSignatureCollector {
    fn visit_super_class(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.super_class = typ))
    }

    fn visit_interface(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.interfaces.push(typ)))
    }

    fn visit_end(&mut self) {
        self.signature = Some(Signature::Class {
            formal_type_parameters: self.formal_type_parameters.clone(),
            super_class: self.super_class.clone(),
            interfaces: self.interfaces.clone(),
        })
    }
}

impl FormalTypeParameterVisitable for ClassSignatureCollector {
    fn visit_formal_type_parameter(
        &mut self,
        name: &str,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        Box::new(FormalTypeParameterCollector::new(
            name.to_owned(),
            |parameter| self.formal_type_parameters.push(parameter),
        ))
    }
}

#[derive(Default)]
struct FieldSignatureCollector {
    signature: Option<Signature>,
    field_type: Type,
}

impl FieldSignatureVisitor for FieldSignatureCollector {
    fn visit_field_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.field_type = typ))
    }

    fn visit_end(&mut self) {
        self.signature = Some(Signature::Field {
            field_type: self.field_type.clone(),
        })
    }
}

#[derive(Default)]
struct MethodSignatureCollector {
    signature: Option<Signature>,
    formal_type_parameters: Vec<FormalTypeParameter>,
    parameter_types: Vec<Type>,
    return_type: Type,
    exception_types: Vec<Type>,
}

impl MethodSignatureVisitor for MethodSignatureCollector {
    fn visit_parameter_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.parameter_types.push(typ)))
    }

    fn visit_return_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.return_type = typ))
    }

    fn visit_exception_type(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.exception_types.push(typ)))
    }

    fn visit_end(&mut self) {
        self.signature = Some(Signature::Method {
            formal_type_parameters: self.formal_type_parameters.clone(),
            parameter_types: self.parameter_types.clone(),
            return_type: self.return_type.clone(),
            exception_types: self.exception_types.clone(),
        })
    }
}

impl FormalTypeParameterVisitable for MethodSignatureCollector {
    fn visit_formal_type_parameter(
        &mut self,
        name: &str,
    ) -> Box<dyn FormalTypeParameterVisitor + '_> {
        Box::new(FormalTypeParameterCollector::new(
            name.to_owned(),
            |parameter| self.formal_type_parameters.push(parameter),
        ))
    }
}

/// Data representation of formal type parameter in signatures.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FormalTypeParameter {
    parameter_name: String,
    class_bound: Option<Type>,
    interface_bounds: Vec<Type>,
}

struct FormalTypeParameterCollector<F>
where
    F: FnMut(FormalTypeParameter),
{
    parameter_name: String,
    class_bound: Option<Type>,
    interface_bounds: Vec<Type>,
    post_action: F,
}

impl<F> FormalTypeParameterCollector<F>
where
    F: FnMut(FormalTypeParameter),
{
    fn new(parameter_name: String, post_action: F) -> Self {
        Self {
            parameter_name,
            class_bound: None,
            interface_bounds: Vec::new(),
            post_action,
        }
    }
}

impl<F> FormalTypeParameterVisitor for FormalTypeParameterCollector<F>
where
    F: FnMut(FormalTypeParameter),
{
    fn visit_class_bound(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.class_bound = Some(typ)))
    }

    fn visit_interface_bound(&mut self) -> Box<dyn TypeVisitor + '_> {
        Box::new(TypeCollector::new(|typ| self.interface_bounds.push(typ)))
    }

    fn visit_end(&mut self) {
        (self.post_action)(FormalTypeParameter {
            parameter_name: self.parameter_name.clone(),
            class_bound: self.class_bound.clone(),
            interface_bounds: self.interface_bounds.clone(),
        })
    }
}

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
        TryFrom::<char>::try_from(*value)
    }
}

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

struct TypeCollector<F>
where
    F: FnMut(Type),
{
    holder: Type,
    stack_actions: VecDeque<(Option<Wildcard>)>,
    // If option is None, then it's array wrapping, otherwise it's type argument wrapping
    post_action: F,
}

impl<F> TypeCollector<F>
where
    F: FnMut(Type),
{
    fn new(post_action: F) -> Self {
        Self {
            holder: Type::Unknown,
            stack_actions: VecDeque::new(),
            post_action,
        }
    }

    #[inline]
    fn wrap_array_type(typ: Type) -> Type {
        Type::Array(Box::new(typ))
    }

    #[inline]
    fn wrap_type_argument(wildcard: Wildcard, typ: Type) -> Type {
        Type::WildcardTypeArgument(wildcard, Box::new(typ))
    }
}

impl<F> TypeVisitor for TypeCollector<F>
where
    F: FnMut(Type),
{
    fn visit_base_type(&mut self, base_type: BaseType) {
        self.holder = Type::BaseType(base_type);
    }

    fn visit_array_type(&mut self) {
        self.stack_actions.push_back(None);
    }

    fn visit_class_type(&mut self, name: &str) {
        self.holder = Type::Class(name.to_owned());
    }

    fn visit_inner_class_type(&mut self, name: &str) {
        self.holder = Type::InnerClass(name.to_owned());
    }

    fn visit_type_variable(&mut self, name: &str) {
        self.holder = Type::TypeVariable(name.to_owned());
    }

    fn visit_type_argument(&mut self) {
        self.holder = Type::TypeArgument;
    }

    fn visit_type_argument_wildcard(&mut self, wildcard: Wildcard) {
        self.stack_actions.push_back(Some(wildcard))
    }

    fn visit_end(&mut self) {
        let mut typ = self.holder.clone();

        while let Some(wildcard) = self.stack_actions.pop_back() {
            if let Some(wildcard) = wildcard {
                typ = Self::wrap_type_argument(wildcard.clone(), typ);
            } else {
                typ = Self::wrap_array_type(typ);
            }
        }

        (self.post_action)(typ)
    }
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;
    use rstest::rstest;

    use crate::asm::node::signature::Signature;
    use crate::error::KapiResult;

    #[rstest]
    #[case("<T:[Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")]
    fn test_class_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let class_signature = Signature::class_signature_from_str(signature)?;

        assert_yaml_snapshot!(class_signature);

        Ok(())
    }

    #[rstest]
    #[case("Ljava/lang/Object;")]
    #[case("TT;")]
    fn test_field_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let field_signature = Signature::field_signature_from_str(signature)?;

        assert_yaml_snapshot!(field_signature);

        Ok(())
    }

    #[rstest]
    #[case("<T:Ljava/lang/Object;>(Z[[ZTT;)Ljava/lang/Object;^Ljava/lang/Exception;")]
    fn test_method_signatures(#[case] signature: &'static str) -> KapiResult<()> {
        let method_signature = Signature::method_signature_from_str(signature)?;

        assert_yaml_snapshot!(method_signature);

        Ok(())
    }
}
