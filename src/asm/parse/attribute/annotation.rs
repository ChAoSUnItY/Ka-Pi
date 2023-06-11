use nom::combinator::map;
use nom::error::{make_error, ErrorKind};
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::IResult;

use crate::asm::node::attribute::annotation::{
    Annotation, ArrayValue, ClassInfo, ConstValue, ElementValue, ElementValuePair, EnumConstValue,
    ParameterAnnotation, PathSegment, TableEntry, TargetInfo, TypeAnnotation, TypePath, Value,
};
use crate::asm::node::attribute::{
    AnnotationDefault, Attribute, RuntimeInvisibleAnnotations,
    RuntimeInvisibleParameterAnnotations, RuntimeInvisibleTypeAnnotations,
    RuntimeVisibleAnnotations, RuntimeVisibleParameterAnnotations, RuntimeVisibleTypeAnnotations,
};
use crate::asm::parse::collect;

pub(crate) fn runtime_visible_annotations(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, annotation),
        |(num_annotations, annotations)| {
            Some(Attribute::RuntimeVisibleAnnotations(
                RuntimeVisibleAnnotations {
                    num_annotations,
                    annotations,
                },
            ))
        },
    )(input)
}

pub(crate) fn runtime_invisible_annotations(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, annotation),
        |(num_annotations, annotations)| {
            Some(Attribute::RuntimeInvisibleAnnotations(
                RuntimeInvisibleAnnotations {
                    num_annotations,
                    annotations,
                },
            ))
        },
    )(input)
}

pub(crate) fn runtime_visible_parameter_annotations(
    input: &[u8],
) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, parameter_annotation),
        |(num_parameters, parameter_annotations)| {
            Some(Attribute::RuntimeVisibleParameterAnnotations(
                RuntimeVisibleParameterAnnotations {
                    num_parameters,
                    parameter_annotations,
                },
            ))
        },
    )(input)
}

pub(crate) fn runtime_invisible_parameter_annotations(
    input: &[u8],
) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, parameter_annotation),
        |(num_parameters, parameter_annotations)| {
            Some(Attribute::RuntimeInvisibleParameterAnnotations(
                RuntimeInvisibleParameterAnnotations {
                    num_parameters,
                    parameter_annotations,
                },
            ))
        },
    )(input)
}

fn parameter_annotation(input: &[u8]) -> IResult<&[u8], ParameterAnnotation> {
    map(
        collect(be_u16, annotation),
        |(num_annotations, annotations)| ParameterAnnotation {
            num_annotations,
            annotations,
        },
    )(input)
}

pub(crate) fn runtime_visible_type_annotations(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, type_annotation),
        |(num_annotations, type_annotations)| {
            Some(Attribute::RuntimeVisibleTypeAnnotations(
                RuntimeVisibleTypeAnnotations {
                    num_annotations,
                    type_annotations,
                },
            ))
        },
    )(input)
}

pub(crate) fn runtime_invisible_type_annotations(
    input: &[u8],
) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, type_annotation),
        |(num_annotations, type_annotations)| {
            Some(Attribute::RuntimeInvisibleTypeAnnotations(
                RuntimeInvisibleTypeAnnotations {
                    num_annotations,
                    type_annotations,
                },
            ))
        },
    )(input)
}

fn type_annotation(input: &[u8]) -> IResult<&[u8], TypeAnnotation> {
    let (input, target_type) = be_u16(input)?;
    let (input, target_info) = target_info(input, target_type)?;
    let (input, type_path) = type_path(input)?;
    let (input, type_index) = be_u16(input)?;
    let (input, (num_element_value_pairs, element_value_pairs)) =
        collect(be_u16, element_value_pairs)(input)?;

    Ok((
        input,
        TypeAnnotation {
            target_type,
            target_info,
            type_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        },
    ))
}

fn target_info(input: &[u8], target_type: u16) -> IResult<&[u8], TargetInfo> {
    match target_type {
        0x00..=0x01 => map(be_u8, |type_parameter_index| TargetInfo::TypeParameter {
            type_parameter_index,
        })(input),
        0x10 => map(be_u16, |super_type_index| TargetInfo::SuperType {
            super_type_index,
        })(input),
        0x11..=0x12 => map(
            tuple((be_u8, be_u8)),
            |(type_parameter_index, bound_index)| TargetInfo::TypeParameterBound {
                type_parameter_index,
                bound_index,
            },
        )(input),
        0x13..=0x15 => Ok((input, TargetInfo::Empty)),
        0x16 => map(be_u8, |formal_parameter_index| {
            TargetInfo::FormalParameter {
                formal_parameter_index,
            }
        })(input),
        0x17 => map(be_u16, |throws_type_index| TargetInfo::Throws {
            throws_type_index,
        })(input),
        0x40..=0x41 => map(collect(be_u16, table_entry), |(table_length, table)| {
            TargetInfo::LocalVar {
                table_length,
                table,
            }
        })(input),
        0x42 => map(be_u16, |exception_table_index| TargetInfo::Catch {
            exception_table_index,
        })(input),
        0x43..=0x46 => map(be_u16, |offset| TargetInfo::Offset { offset })(input),
        0x47..=0x4B => map(tuple((be_u16, be_u8)), |(offset, type_argument_index)| {
            TargetInfo::TypeArgument {
                offset,
                type_argument_index,
            }
        })(input),
        _ => Err(Error(make_error(input, ErrorKind::NoneOf))),
    }
}

fn table_entry(input: &[u8]) -> IResult<&[u8], TableEntry> {
    map(
        tuple((be_u16, be_u16, be_u16)),
        |(start_pc, length, index)| TableEntry {
            start_pc,
            length,
            index,
        },
    )(input)
}

fn type_path(input: &[u8]) -> IResult<&[u8], TypePath> {
    map(collect(be_u8, path_segment), |(path_length, path)| {
        TypePath { path_length, path }
    })(input)
}

fn path_segment(input: &[u8]) -> IResult<&[u8], PathSegment> {
    map(
        tuple((be_u8, be_u8)),
        |(type_path_kind, type_argument_index)| PathSegment {
            type_path_kind,
            type_argument_index,
        },
    )(input)
}

pub(crate) fn annotation_default(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(element_value, |default_value| {
        Some(Attribute::AnnotationDefault(AnnotationDefault {
            default_value,
        }))
    })(input)
}

fn annotation(input: &[u8]) -> IResult<&[u8], Annotation> {
    map(
        tuple((be_u16, collect(be_u16, element_value_pairs))),
        |(type_index, (num_element_value_pairs, element_value_pairs))| Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        },
    )(input)
}

fn element_value_pairs(input: &[u8]) -> IResult<&[u8], ElementValuePair> {
    map(
        tuple((be_u16, element_value)),
        |(element_name_index, value)| ElementValuePair {
            element_name_index,
            value,
        },
    )(input)
}

fn element_value(input: &[u8]) -> IResult<&[u8], ElementValue> {
    let (input, tag) = be_u8(input)?;
    let (input, value) = value(input, tag)?;

    Ok((input, ElementValue { tag, value }))
}

fn value(input: &[u8], tag: u8) -> IResult<&[u8], Value> {
    match tag as char {
        'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => map(be_u16, |const_value_index| {
            Value::ConstValue(ConstValue { const_value_index })
        })(input),
        'e' => map(
            tuple((be_u16, be_u16)),
            |(type_name_index, const_name_index)| {
                Value::EnumConstValue(EnumConstValue {
                    type_name_index,
                    const_name_index,
                })
            },
        )(input),
        'c' => map(be_u16, |class_info_index| {
            Value::ClassInfo(ClassInfo { class_info_index })
        })(input),
        '@' => map(annotation, |annotation| Value::AnnotationValue(annotation))(input),
        '[' => map(collect(be_u16, element_value), |(num_values, values)| {
            Value::ArrayValue(ArrayValue { num_values, values })
        })(input),
        _ => Err(Error(make_error(input, ErrorKind::NoneOf))),
    }
}
