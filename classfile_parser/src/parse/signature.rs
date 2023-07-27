use std::iter::Peekable;
use std::str::Chars;

use crate::node::signature::{
    ArrayType, BaseType, ClassType, FormalTypeParameter, ReferenceType, Signature, SignatureType,
    ThrowsType, TypeArgument, TypeVariable, WildcardIndicator,
};
use crate::parse::error::{ParseError, ParseResult};

const EXCLUDED_IDENTIFIER_CHARACTERS: &'static str = ".;[/<>:";

pub fn class_signature(input: &str) -> ParseResult<Signature> {
    let mut input = input.chars().peekable();
    let formal_type_parameters = formal_types(&mut input)?;
    let super_class = class_type_signature(&mut input)?;
    let mut interfaces = Vec::new();

    while let Some(char) = input.peek() {
        if *char == 'L' {
            interfaces.push(class_type_signature(&mut input)?);
        } else {
            break;
        }
    }

    if input.peek().is_some() {
        let remain = input.collect::<Vec<_>>();

        Err(ParseError::Remains(remain.len()))
    } else {
        Ok(Signature::Class {
            formal_type_parameters,
            super_class,
            interfaces,
        })
    }
}

pub fn field_signature(input: &str) -> ParseResult<Signature> {
    let mut input = input.chars().peekable();
    let field_type = reference_type(&mut input)?;

    if input.peek().is_some() {
        let remain = input.collect::<Vec<_>>();

        Err(ParseError::Remains(remain.len()))
    } else {
        Ok(Signature::Field { field_type })
    }
}

pub fn method_signature(input: &str) -> ParseResult<Signature> {
    let mut input = input.chars().peekable();
    let formal_type_parameters = formal_types(&mut input)?;

    assert_char(input.next(), '(')?;

    let mut parameter_types = Vec::new();

    while let Some(char) = input.peek() {
        if *char != ')' {
            parameter_types.push(java_type_signature(&mut input)?);
        } else {
            break;
        }
    }

    assert_char(input.next(), ')')?;

    let return_type = result(&mut input)?;
    let mut exception_types = Vec::new();

    while let Some(char) = input.peek() {
        if *char == '^' {
            exception_types.push(throws_signature(&mut input)?);
        } else {
            break;
        }
    }

    if input.peek().is_some() {
        let remain = input.collect::<Vec<_>>();

        Err(ParseError::Remains(remain.len()))
    } else {
        Ok(Signature::Method {
            formal_type_parameters,
            parameter_types,
            return_type,
            exception_types,
        })
    }
}

fn formal_types(input: &mut Peekable<Chars>) -> ParseResult<Vec<FormalTypeParameter>> {
    assert_char(input.next(), '<')?;
    let mut formal_types = Vec::new();

    while let Some(char) = input.peek() {
        if *char != '>' {
            formal_types.push(formal_type(input)?);
        } else {
            break;
        }
    }

    assert_char(input.next(), '>')?;

    Ok(formal_types)
}

fn formal_type(input: &mut Peekable<Chars>) -> ParseResult<FormalTypeParameter> {
    let parameter_name = identifier(input)?;
    let class_bound = class_bound(input)?;
    let mut interface_bounds = Vec::new();

    while let Some(char) = input.peek() {
        if *char == ':' {
            interface_bounds.push(interface_bound(input)?);
        } else {
            break;
        }
    }

    Ok(FormalTypeParameter {
        parameter_name,
        class_bound,
        interface_bounds,
    })
}

fn result(input: &mut Peekable<Chars>) -> ParseResult<SignatureType> {
    if let Ok(java_type_signature) = java_type_signature(input) {
        Ok(java_type_signature)
    } else if let Ok(void_descriptor) = void_descriptor(input) {
        Ok(SignatureType::BaseType(void_descriptor))
    } else {
        Err(ParseError::OutOfBound("return type"))
    }
}

fn throws_signature(input: &mut Peekable<Chars>) -> ParseResult<ThrowsType> {
    assert_char(input.next(), '^')?;

    if let Ok(class_type_signature) = class_type_signature(input) {
        Ok(ThrowsType::Class(class_type_signature))
    } else if let Ok(type_variable) = type_variable(input) {
        Ok(ThrowsType::TypeVariable(type_variable))
    } else {
        Err(ParseError::OutOfBound("throws signature"))
    }
}

fn class_bound(input: &mut Peekable<Chars>) -> ParseResult<Option<ClassType>> {
    assert_char(input.next(), ':')?;

    match input.peek() {
        Some(char) if *char == 'L' => Ok(Some(class_type_signature(input)?)),
        _ => Ok(None),
    }
}

fn interface_bound(input: &mut Peekable<Chars>) -> ParseResult<ClassType> {
    assert_char(input.next(), ':')?;

    class_type_signature(input)
}

fn java_type_signature(input: &mut Peekable<Chars>) -> ParseResult<SignatureType> {
    if let Ok(base_type) = base_type(input) {
        Ok(SignatureType::BaseType(base_type))
    } else if let Ok(reference_type) = reference_type(input) {
        Ok(SignatureType::ReferenceType(reference_type))
    } else {
        Err(ParseError::OutOfBound("java type signature"))
    }
}

fn base_type(input: &mut Peekable<Chars>) -> ParseResult<BaseType> {
    let Some(char) = input.peek()
    else {
        return Err(ParseError::OutOfBound("base type"));
    };

    for base_type_char in "ZBSIJFD".chars() {
        if *char == base_type_char {
            let char = input.next().unwrap();

            return BaseType::try_from(char).map_err(|_| {
                ParseError::MismatchedCharacter(char, vec!['Z', 'B', 'S', 'I', 'J', 'F', 'D'])
            });
        }
    }

    Err(ParseError::MismatchedCharacter(
        *char,
        vec!['Z', 'B', 'S', 'I', 'J', 'F', 'D'],
    ))
}

fn void_descriptor(input: &mut Peekable<Chars>) -> ParseResult<BaseType> {
    assert_char(input.next(), 'V')?;

    Ok(BaseType::Void)
}

fn reference_type(input: &mut Peekable<Chars>) -> ParseResult<ReferenceType> {
    match input.peek() {
        Some(char) if *char == 'L' => Ok(ReferenceType::Class(class_type_signature(input)?)),
        Some(char) if *char == 'T' => Ok(ReferenceType::TypeVariable(type_variable(input)?)),
        Some(char) if *char == '[' => Ok(ReferenceType::Array(array_type(input)?)),
        Some(char) => Err(ParseError::MismatchedCharacter(*char, vec!['L', 'T', '['])),
        None => Err(ParseError::OutOfBound("reference type")),
    }
}

fn class_type_signature(input: &mut Peekable<Chars>) -> ParseResult<ClassType> {
    assert_char(input.next(), 'L')?;

    let (package_path, class_name) = package_specifier_and_class_type(input)?;
    let type_arguments = type_arguments(input)?;
    let mut inner_classes = Vec::new();

    while let Some(char) = input.peek() {
        if *char != ';' {
            inner_classes.push(class_type_signature_suffix(input)?);
        } else {
            break;
        }
    }

    assert_char(input.next(), ';')?;

    Ok(ClassType {
        package_path,
        class_name,
        type_arguments,
        inner_classes,
    })
}

fn package_specifier_and_class_type(input: &mut Peekable<Chars>) -> ParseResult<(String, String)> {
    let mut class_path_builder = Vec::with_capacity(3);

    loop {
        class_path_builder.push(identifier(input)?);

        match input.peek() {
            Some(builder) if *builder == '/' => {
                class_path_builder.push(input.next().unwrap().to_string());
                continue;
            }
            _ => break,
        }
    }

    let (class_type, package_specifier) = if class_path_builder.len() == 1 {
        (String::new(), class_path_builder[0].clone())
    } else if class_path_builder.len() == 0 {
        return Err(ParseError::OutOfBound("package specifier and class type"));
    } else {
        class_path_builder
            .split_last()
            .map(|(class_type, package_specifier)| {
                (class_type.to_string(), package_specifier.join(""))
            })
            .unwrap()
    };

    Ok((package_specifier, class_type))
}

fn class_type_signature_suffix(
    input: &mut Peekable<Chars>,
) -> ParseResult<(String, Vec<TypeArgument>)> {
    assert_char(input.next(), '.')?;

    let class_type = identifier(input)?;
    let type_arguments = type_arguments(input)?;

    Ok((class_type, type_arguments))
}

fn type_arguments(input: &mut Peekable<Chars>) -> ParseResult<Vec<TypeArgument>> {
    match input.peek() {
        Some(char) if *char == '<' => {
            assert_char(input.next(), '<')?;

            let mut type_arguments = Vec::new();

            while let Some(char) = input.peek() {
                if *char != '>' {
                    type_arguments.push(type_argument(input)?);
                } else {
                    break;
                }
            }

            assert_char(input.next(), '>')?;

            Ok(type_arguments)
        }
        _ => Ok(Vec::new()),
    }
}

fn type_argument(input: &mut Peekable<Chars>) -> ParseResult<TypeArgument> {
    match input.peek() {
        Some(char) if *char == '+' || *char == '-' => {
            let wildcard_indicator = WildcardIndicator::try_from(char).unwrap();
            let bounded_type = reference_type(input)?;

            Ok(TypeArgument::Bounded {
                wildcard_indicator,
                bounded_type,
            })
        }
        Some(char) if *char == '*' => Ok(TypeArgument::Wildcard),
        Some(char) => Err(ParseError::MismatchedCharacter(*char, vec!['+', '-', '*'])),
        None => Err(ParseError::OutOfBound("type_argument")),
    }
}

fn type_variable(input: &mut Peekable<Chars>) -> ParseResult<TypeVariable> {
    assert_char(input.next(), 'T')?;

    let identifier = identifier(input)?;

    assert_char(input.next(), ';')?;

    Ok(TypeVariable(identifier))
}

fn array_type(input: &mut Peekable<Chars>) -> ParseResult<ArrayType> {
    assert_char(input.next(), '[')?;

    let inner_type = java_type_signature(input)?;

    Ok(ArrayType(Box::new(inner_type)))
}

fn identifier(input: &mut Peekable<Chars>) -> ParseResult<String> {
    let mut identifier_builder = String::with_capacity(8);

    while let Some(char) = input.peek() {
        if !EXCLUDED_IDENTIFIER_CHARACTERS.contains(*char) {
            identifier_builder.push(input.next().unwrap());
        } else {
            break;
        }
    }

    Ok(identifier_builder)
}

#[inline]
fn assert_char(char: Option<char>, expected: char) -> ParseResult<char> {
    match char {
        Some(char) => {
            if char == expected {
                Ok(char)
            } else {
                Err(ParseError::MismatchedCharacter(char, vec![expected]))
            }
        }
        None => Err(ParseError::OutOfBound("signature")),
    }
}
