use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::{char, one_of};
use nom::combinator::{map, map_res, opt, peek};
use nom::error::ErrorKind;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, tuple};
use nom::Err::Error;
use nom::{IResult, Offset, Parser};

use crate::asm::node::signature::{
    ArrayType, BaseType, ClassType, FormalTypeParameter, ReferenceType, Signature, SignatureType,
    ThrowsType, TypeArgument, TypeVariable, WildcardIndicator,
};

pub(crate) const EXCLUDED_IDENTIFIER_CHARACTERS: &'static str = ".;[/<>:";

pub(crate) fn class_signature(input: &str) -> IResult<&str, Signature> {
    map(
        tuple((
            formal_types,
            class_type_signature,
            many0(class_type_signature),
        )),
        |(formal_type_parameters, super_class, interfaces)| Signature::Class {
            formal_type_parameters,
            super_class,
            interfaces,
        },
    )(input)
}

pub(crate) fn field_signature(input: &str) -> IResult<&str, Signature> {
    map(reference_type, |field_type| Signature::Field { field_type })(input)
}

pub(crate) fn method_signature(input: &str) -> IResult<&str, Signature> {
    map(
        tuple((
            formal_types,
            delimited(char('('), many0(java_type_signature), char(')')),
            result,
            many0(throws_signature),
        )),
        |(formal_type_parameters, parameter_types, return_type, exception_types)| {
            Signature::Method {
                formal_type_parameters,
                parameter_types,
                return_type,
                exception_types,
            }
        },
    )(input)
}

fn formal_types(input: &str) -> IResult<&str, Vec<FormalTypeParameter>> {
    delimited(char('<'), many1(formal_type), char('>'))(input)
}

fn formal_type(input: &str) -> IResult<&str, FormalTypeParameter> {
    map(
        tuple((identifier, class_bound, many0(interface_bound))),
        |(parameter_name, class_bound, interface_bounds)| FormalTypeParameter {
            parameter_name: parameter_name.to_string(),
            class_bound,
            interface_bounds,
        },
    )(input)
}

fn result(input: &str) -> IResult<&str, SignatureType> {
    alt((
        java_type_signature,
        map(void_descriptor, |void_type| {
            SignatureType::BaseType(void_type)
        }),
    ))(input)
}

fn throws_signature(input: &str) -> IResult<&str, ThrowsType> {
    preceded(
        char('^'),
        alt((
            map(class_type_signature, |class_type| {
                ThrowsType::Class(class_type)
            }),
            map(type_variable, |type_variable| {
                ThrowsType::TypeVariable(type_variable)
            }),
        )),
    )(input)
}

fn class_bound(input: &str) -> IResult<&str, Option<ClassType>> {
    preceded(char(':'), opt(class_type_signature))(input)
}

fn interface_bound(input: &str) -> IResult<&str, ClassType> {
    preceded(char(':'), class_type_signature)(input)
}

fn java_type_signature(input: &str) -> IResult<&str, SignatureType> {
    alt((
        map(base_type, |base_type| SignatureType::BaseType(base_type)),
        map(reference_type, |reference_type| {
            SignatureType::ReferenceType(reference_type)
        }),
    ))(input)
}

fn base_type(input: &str) -> IResult<&str, BaseType> {
    map_res(one_of("ZBSIJFD"), |char| BaseType::try_from(char))(input)
}

fn void_descriptor(input: &str) -> IResult<&str, BaseType> {
    map_res(char('V'), |char| BaseType::try_from(char))(input)
}

fn reference_type(input: &str) -> IResult<&str, ReferenceType> {
    let (input, leading_type_char) = peek(one_of("LT["))(input)?;

    match leading_type_char {
        'L' => map(class_type_signature, |class_type| {
            ReferenceType::Class(class_type)
        })(input),
        'T' => map(type_variable, |type_variable| {
            ReferenceType::TypeVariable(type_variable)
        })(input),
        '[' => map(array_type, |array_type| ReferenceType::Array(array_type))(input),
        _ => Err(Error(nom::error::Error::new(input, ErrorKind::OneOf))),
    }
}

fn class_type_signature(input: &str) -> IResult<&str, ClassType> {
    map(
        delimited(
            char('L'),
            tuple((
                opt(package_specifier),
                simple_class_type_signature,
                opt(many0(class_type_signature_suffix)),
            )),
            char(';'),
        ),
        |(package_specifier, (class_name, type_arguments), inner_classes)| ClassType {
            package_path: package_specifier.unwrap_or("").to_string(),
            class_name: class_name.to_string(),
            type_arguments,
            inner_classes: inner_classes
                .unwrap_or_default()
                .into_iter()
                .map(|(class_name, type_arguments)| (class_name.to_string(), type_arguments))
                .collect(),
        },
    )(input)
}

fn package_specifier(input: &str) -> IResult<&str, &str> {
    let mut remain = input;

    loop {
        let (current_remain, _) = tuple((identifier, char('/')))(remain)?;
        let (current_remain, peek_char) =
            opt(peek(tuple((identifier, char('/')))))(current_remain)?;

        remain = current_remain;

        if !peek_char.is_some() {
            break;
        }
    }

    let offset = input.offset(remain);

    Ok((remain, &input[0..offset - 1])) // Remove last `/`
}

fn simple_class_type_signature(input: &str) -> IResult<&str, (&str, Vec<TypeArgument>)> {
    map(
        tuple((
            identifier,
            opt(delimited(char('<'), many1(type_argument), char('>'))),
        )),
        |(class_name, type_arguments)| (class_name, type_arguments.unwrap_or_default()),
    )(input)
}

fn class_type_signature_suffix(input: &str) -> IResult<&str, (&str, Vec<TypeArgument>)> {
    preceded(char('.'), simple_class_type_signature)(input)
}

fn type_argument(input: &str) -> IResult<&str, TypeArgument> {
    alt((
        map(
            tuple((opt(one_of("+-")), reference_type)),
            |(wildcard_indicator, bounded_type)| TypeArgument::Bounded {
                wildcard_indicator: wildcard_indicator.and_then(|wildcard_indicator| {
                    WildcardIndicator::try_from(wildcard_indicator).ok()
                }),
                bounded_type,
            },
        ),
        map(char('*'), |_| TypeArgument::Wildcard),
    ))(input)
}

fn type_variable(input: &str) -> IResult<&str, TypeVariable> {
    map(
        delimited(char('T'), identifier, char(';')),
        |type_variable| TypeVariable(type_variable.to_string()),
    )(input)
}

fn array_type(input: &str) -> IResult<&str, ArrayType> {
    map(preceded(char('['), java_type_signature), |inner_type| {
        ArrayType(Box::new(inner_type))
    })(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_till(|c| EXCLUDED_IDENTIFIER_CHARACTERS.contains(c))(input)
}
