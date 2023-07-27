use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::attribute::annotation::{
    Annotation, ArrayValue, ClassInfo, ConstValue, ElementValue, ElementValuePair, EnumConstValue,
    ParameterAnnotation, PathSegment, TableEntry, TargetInfo, TypeAnnotation, TypePath, Value,
};
use crate::node::attribute::{
    AnnotationDefault, Attribute, RuntimeInvisibleAnnotations,
    RuntimeInvisibleParameterAnnotations, RuntimeInvisibleTypeAnnotations,
    RuntimeVisibleAnnotations, RuntimeVisibleParameterAnnotations, RuntimeVisibleTypeAnnotations,
};
use crate::parse::error::{ParseError, ParseResult};

#[inline]
pub(super) fn runtime_visible_annotations<R: Read>(
    input: &mut R,
) -> ParseResult<Option<Attribute>> {
    let num_annotations = input.read_u16::<BigEndian>()?;
    let mut annotations = Vec::with_capacity(num_annotations as usize);

    for _ in 0..num_annotations {
        annotations.push(annotation(input)?);
    }

    Ok(Some(Attribute::RuntimeVisibleAnnotations(
        RuntimeVisibleAnnotations {
            num_annotations,
            annotations,
        },
    )))
}

#[inline]
pub(super) fn runtime_invisible_annotations<R: Read>(
    input: &mut R,
) -> ParseResult<Option<Attribute>> {
    let num_annotations = input.read_u16::<BigEndian>()?;
    let mut annotations = Vec::with_capacity(num_annotations as usize);

    for _ in 0..num_annotations {
        annotations.push(annotation(input)?);
    }

    Ok(Some(Attribute::RuntimeInvisibleAnnotations(
        RuntimeInvisibleAnnotations {
            num_annotations,
            annotations,
        },
    )))
}

#[inline]
pub(super) fn runtime_visible_parameter_annotations<R: Read>(
    input: &mut R,
) -> ParseResult<Option<Attribute>> {
    let num_parameters = input.read_u16::<BigEndian>()?;
    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);

    for _ in 0..num_parameters {
        parameter_annotations.push(parameter_annotation(input)?);
    }

    Ok(Some(Attribute::RuntimeVisibleParameterAnnotations(
        RuntimeVisibleParameterAnnotations {
            num_parameters,
            parameter_annotations,
        },
    )))
}

#[inline]
pub(super) fn runtime_invisible_parameter_annotations<R: Read>(
    input: &mut R,
) -> ParseResult<Option<Attribute>> {
    let num_parameters = input.read_u16::<BigEndian>()?;
    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);

    for _ in 0..num_parameters {
        parameter_annotations.push(parameter_annotation(input)?);
    }

    Ok(Some(Attribute::RuntimeInvisibleParameterAnnotations(
        RuntimeInvisibleParameterAnnotations {
            num_parameters,
            parameter_annotations,
        },
    )))
}

#[inline]
fn parameter_annotation<R: Read>(input: &mut R) -> ParseResult<ParameterAnnotation> {
    let num_annotations = input.read_u16::<BigEndian>()?;
    let mut annotations = Vec::with_capacity(num_annotations as usize);

    for _ in 0..num_annotations {
        annotations.push(annotation(input)?);
    }

    Ok(ParameterAnnotation {
        num_annotations,
        annotations,
    })
}

#[inline]
pub(super) fn runtime_visible_type_annotations<R: Read>(
    input: &mut R,
) -> ParseResult<Option<Attribute>> {
    let num_annotations = input.read_u16::<BigEndian>()?;
    let mut type_annotations = Vec::with_capacity(num_annotations as usize);

    for _ in 0..num_annotations {
        type_annotations.push(type_annotation(input)?);
    }

    Ok(Some(Attribute::RuntimeVisibleTypeAnnotations(
        RuntimeVisibleTypeAnnotations {
            num_annotations,
            type_annotations,
        },
    )))
}

#[inline]
pub(super) fn runtime_invisible_type_annotations<R: Read>(
    input: &mut R,
) -> ParseResult<Option<Attribute>> {
    let num_annotations = input.read_u16::<BigEndian>()?;
    let mut type_annotations = Vec::with_capacity(num_annotations as usize);

    for _ in 0..num_annotations {
        type_annotations.push(type_annotation(input)?);
    }

    Ok(Some(Attribute::RuntimeInvisibleTypeAnnotations(
        RuntimeInvisibleTypeAnnotations {
            num_annotations,
            type_annotations,
        },
    )))
}

#[inline]
fn type_annotation<R: Read>(input: &mut R) -> ParseResult<TypeAnnotation> {
    let target_type = input.read_u16::<BigEndian>()?;
    let target_info = target_info(input, target_type)?;
    let type_path = type_path(input)?;
    let type_index = input.read_u16::<BigEndian>()?;
    let num_element_value_pairs = input.read_u16::<BigEndian>()?;
    let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

    for _ in 0..num_element_value_pairs {
        element_value_pairs.push(parse_element_value_pairs(input)?);
    }

    Ok(TypeAnnotation {
        target_type,
        target_info,
        type_path,
        type_index,
        num_element_value_pairs,
        element_value_pairs,
    })
}

#[inline]
fn target_info<R: Read>(input: &mut R, target_type: u16) -> ParseResult<TargetInfo> {
    let target_info = match target_type {
        0x00..=0x01 => {
            let type_parameter_index = input.read_u8()?;

            TargetInfo::TypeParameter {
                type_parameter_index,
            }
        }
        0x10 => {
            let super_type_index = input.read_u16::<BigEndian>()?;

            TargetInfo::SuperType { super_type_index }
        }
        0x11..=0x12 => {
            let type_parameter_index = input.read_u8()?;
            let bound_index = input.read_u8()?;

            TargetInfo::TypeParameterBound {
                type_parameter_index,
                bound_index,
            }
        }
        0x13..=0x15 => TargetInfo::Empty,
        0x16 => {
            let formal_parameter_index = input.read_u8()?;

            TargetInfo::FormalParameter {
                formal_parameter_index,
            }
        }
        0x17 => {
            let throws_type_index = input.read_u16::<BigEndian>()?;

            TargetInfo::Throws { throws_type_index }
        }
        0x40..=0x41 => {
            let table_length = input.read_u16::<BigEndian>()?;
            let mut table = Vec::with_capacity(table_length as usize);

            for _ in 0..table_length {
                table.push(table_entry(input)?);
            }

            TargetInfo::LocalVar {
                table_length,
                table,
            }
        }
        0x42 => {
            let exception_table_index = input.read_u16::<BigEndian>()?;

            TargetInfo::Catch {
                exception_table_index,
            }
        }
        0x43..=0x46 => {
            let offset = input.read_u16::<BigEndian>()?;

            TargetInfo::Offset { offset }
        }
        0x47..=0x4B => {
            let offset = input.read_u16::<BigEndian>()?;
            let type_argument_index = input.read_u8()?;

            TargetInfo::TypeArgument {
                offset,
                type_argument_index,
            }
        }
        _ => {
            return Err(ParseError::MatchOutOfBoundUsize(
                "target type",
                vec![
                    "0x00..=0x01",
                    "0x10",
                    "0x11..=0x12",
                    "0x13..=0x15",
                    "0x16",
                    "0x17",
                    "0x40..=0x41",
                    "0x42",
                    "0x43..=0x46",
                    "0x47..=0x4B",
                ],
                target_type as usize,
            ))
        }
    };

    Ok(target_info)
}

#[inline]
fn table_entry<R: Read>(input: &mut R) -> ParseResult<TableEntry> {
    let start_pc = input.read_u16::<BigEndian>()?;
    let length = input.read_u16::<BigEndian>()?;
    let index = input.read_u16::<BigEndian>()?;

    Ok(TableEntry {
        start_pc,
        length,
        index,
    })
}

#[inline]
fn type_path<R: Read>(input: &mut R) -> ParseResult<TypePath> {
    let path_length = input.read_u8()?;
    let mut path = Vec::with_capacity(path_length as usize);

    for _ in 0..path_length {
        path.push(path_segment(input)?);
    }

    Ok(TypePath { path_length, path })
}

#[inline]
fn path_segment<R: Read>(input: &mut R) -> ParseResult<PathSegment> {
    let type_path_kind = input.read_u8()?;
    let type_argument_index = input.read_u8()?;

    Ok(PathSegment {
        type_path_kind,
        type_argument_index,
    })
}

#[inline]
pub(super) fn annotation_default<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let default_value = element_value(input)?;

    Ok(Some(Attribute::AnnotationDefault(AnnotationDefault {
        default_value,
    })))
}

#[inline]
fn annotation<R: Read>(input: &mut R) -> ParseResult<Annotation> {
    let type_index = input.read_u16::<BigEndian>()?;
    let num_element_value_pairs = input.read_u16::<BigEndian>()?;
    let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

    for _ in 0..num_element_value_pairs {
        element_value_pairs.push(parse_element_value_pairs(input)?);
    }

    Ok(Annotation {
        type_index,
        num_element_value_pairs,
        element_value_pairs,
    })
}

#[inline]
fn parse_element_value_pairs<R: Read>(input: &mut R) -> ParseResult<ElementValuePair> {
    let element_name_index = input.read_u16::<BigEndian>()?;
    let value = element_value(input)?;

    Ok(ElementValuePair {
        element_name_index,
        value,
    })
}

#[inline]
fn element_value<R: Read>(input: &mut R) -> ParseResult<ElementValue> {
    let tag = input.read_u8()?;
    let value = value(input, tag)?;

    Ok(ElementValue { tag, value })
}

#[inline]
fn value<R: Read>(input: &mut R, tag: u8) -> ParseResult<Value> {
    match tag as char {
        'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
            let const_value_index = input.read_u16::<BigEndian>()?;

            Ok(Value::ConstValue(ConstValue { const_value_index }))
        }
        'e' => {
            let type_name_index = input.read_u16::<BigEndian>()?;
            let const_name_index = input.read_u16::<BigEndian>()?;

            Ok(Value::EnumConstValue(EnumConstValue {
                type_name_index,
                const_name_index,
            }))
        }
        'c' => {
            let class_info_index = input.read_u16::<BigEndian>()?;

            Ok(Value::ClassInfo(ClassInfo { class_info_index }))
        }
        '@' => {
            let annotation = annotation(input)?;

            Ok(Value::AnnotationValue(annotation))
        }
        '[' => {
            let num_values = input.read_u16::<BigEndian>()?;
            let mut values = Vec::with_capacity(num_values as usize);

            for _ in 0..num_values {
                values.push(element_value(input)?);
            }

            Ok(Value::ArrayValue(ArrayValue { num_values, values }))
        }
        _ => Err(ParseError::MatchOutOfBoundChar(
            "value",
            vec![
                'B', 'C', 'D', 'F', 'I', 'J', 'S', 'Z', 's', 'e', 'c', '@', '[',
            ],
            tag as char,
        )),
    }
}
