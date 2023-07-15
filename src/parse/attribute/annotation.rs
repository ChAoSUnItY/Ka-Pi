use nom::error::{make_error, ErrorKind};
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::IResult;

use byte_span::{offset, BytesSpan};

use crate::node::attribute::annotation::{
    Annotation, ArrayValue, ClassInfo, ConstValue, ElementValue, ElementValuePair, EnumConstValue,
    ParameterAnnotation, PathSegment, TableEntry, TargetInfo, TypeAnnotation, TypePath, Value,
};
use crate::node::attribute::{
    AnnotationDefault, Attribute, RuntimeInvisibleAnnotations,
    RuntimeInvisibleParameterAnnotations, RuntimeInvisibleTypeAnnotations,
    RuntimeVisibleAnnotations, RuntimeVisibleParameterAnnotations, RuntimeVisibleTypeAnnotations,
};
use crate::node::{Node, Nodes};
use crate::parse::{collect, map_node, node};

pub(crate) fn runtime_visible_annotations(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), annotation),
        |(num_annotations, annotations): (Node<u16>, Nodes<Annotation>)| {
            Attribute::RuntimeVisibleAnnotations(RuntimeVisibleAnnotations {
                num_annotations,
                annotations,
            })
        },
    )(input)
}

pub(crate) fn runtime_invisible_annotations(
    input: BytesSpan,
) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), annotation),
        |(num_annotations, annotations): (Node<u16>, Nodes<Annotation>)| {
            Attribute::RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations {
                num_annotations,
                annotations,
            })
        },
    )(input)
}

pub(crate) fn runtime_visible_parameter_annotations(
    input: BytesSpan,
) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), parameter_annotation),
        |(num_parameters, parameter_annotations): (Node<u16>, Nodes<ParameterAnnotation>)| {
            Attribute::RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations {
                num_parameters,
                parameter_annotations,
            })
        },
    )(input)
}

pub(crate) fn runtime_invisible_parameter_annotations(
    input: BytesSpan,
) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), parameter_annotation),
        |(num_parameters, parameter_annotations): (Node<u16>, Nodes<ParameterAnnotation>)| {
            Attribute::RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations {
                num_parameters,
                parameter_annotations,
            })
        },
    )(input)
}

fn parameter_annotation(input: BytesSpan) -> IResult<BytesSpan, Node<ParameterAnnotation>> {
    map_node(
        collect(node(be_u16), annotation),
        |(num_annotations, annotations): (Node<u16>, Nodes<Annotation>)| ParameterAnnotation {
            num_annotations,
            annotations,
        },
    )(input)
}

pub(crate) fn runtime_visible_type_annotations(
    input: BytesSpan,
) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), type_annotation),
        |(num_annotations, type_annotations): (Node<u16>, Nodes<TypeAnnotation>)| {
            Attribute::RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotations {
                num_annotations,
                type_annotations,
            })
        },
    )(input)
}

pub(crate) fn runtime_invisible_type_annotations(
    input: BytesSpan,
) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), type_annotation),
        |(num_annotations, type_annotations): (Node<u16>, Nodes<TypeAnnotation>)| {
            Attribute::RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotations {
                num_annotations,
                type_annotations,
            })
        },
    )(input)
}

fn type_annotation(input: BytesSpan) -> IResult<BytesSpan, Node<TypeAnnotation>> {
    let (input, offset) = offset(input)?;
    let (input, target_type) = node(be_u16)(input)?;
    let (input, target_info) = target_info(input, *target_type)?;
    let (input, type_path) = type_path(input)?;
    let (input, type_index) = node(be_u16)(input)?;
    let (input, (num_element_value_pairs, element_value_pairs)) =
        collect(node(be_u16), element_value_pairs)(input)?;

    Ok((
        input,
        Node(
            offset..input.offset,
            TypeAnnotation {
                target_type,
                target_info,
                type_path,
                type_index,
                num_element_value_pairs,
                element_value_pairs,
            },
        ),
    ))
}

fn target_info(input: BytesSpan, target_type: u16) -> IResult<BytesSpan, Node<TargetInfo>> {
    match target_type {
        0x00..=0x01 => map_node(node(be_u8), |type_parameter_index: Node<u8>| {
            TargetInfo::TypeParameter {
                type_parameter_index,
            }
        })(input),
        0x10 => map_node(node(be_u16), |super_type_index: Node<u16>| {
            TargetInfo::SuperType { super_type_index }
        })(input),
        0x11..=0x12 => map_node(
            tuple((node(be_u8), node(be_u8))),
            |(type_parameter_index, bound_index): (Node<u8>, Node<u8>)| {
                TargetInfo::TypeParameterBound {
                    type_parameter_index,
                    bound_index,
                }
            },
        )(input),
        0x13..=0x15 => Ok((input, Node(input.offset..input.offset, TargetInfo::Empty))),
        0x16 => map_node(node(be_u8), |formal_parameter_index: Node<u8>| {
            TargetInfo::FormalParameter {
                formal_parameter_index,
            }
        })(input),
        0x17 => map_node(node(be_u16), |throws_type_index: Node<u16>| {
            TargetInfo::Throws { throws_type_index }
        })(input),
        0x40..=0x41 => map_node(
            collect(node(be_u16), table_entry),
            |(table_length, table): (Node<u16>, Nodes<TableEntry>)| TargetInfo::LocalVar {
                table_length,
                table,
            },
        )(input),
        0x42 => map_node(node(be_u16), |exception_table_index: Node<u16>| {
            TargetInfo::Catch {
                exception_table_index,
            }
        })(input),
        0x43..=0x46 => map_node(node(be_u16), |offset: Node<u16>| TargetInfo::Offset {
            offset,
        })(input),
        0x47..=0x4B => map_node(
            tuple((node(be_u16), node(be_u8))),
            |(offset, type_argument_index): (Node<u16>, Node<u8>)| TargetInfo::TypeArgument {
                offset,
                type_argument_index,
            },
        )(input),
        _ => Err(Error(make_error(input, ErrorKind::NoneOf))),
    }
}

fn table_entry(input: BytesSpan) -> IResult<BytesSpan, Node<TableEntry>> {
    map_node(
        tuple((node(be_u16), node(be_u16), node(be_u16))),
        |(start_pc, length, index): (Node<u16>, Node<u16>, Node<u16>)| TableEntry {
            start_pc,
            length,
            index,
        },
    )(input)
}

fn type_path(input: BytesSpan) -> IResult<BytesSpan, Node<TypePath>> {
    map_node(
        collect(node(be_u8), path_segment),
        |(path_length, path): (Node<u8>, Nodes<PathSegment>)| TypePath { path_length, path },
    )(input)
}

fn path_segment(input: BytesSpan) -> IResult<BytesSpan, Node<PathSegment>> {
    map_node(
        tuple((node(be_u8), node(be_u8))),
        |(type_path_kind, type_argument_index)| PathSegment {
            type_path_kind,
            type_argument_index,
        },
    )(input)
}

pub(crate) fn annotation_default(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(element_value, |default_value: Node<ElementValue>| {
        Attribute::AnnotationDefault(AnnotationDefault { default_value })
    })(input)
}

fn annotation(input: BytesSpan) -> IResult<BytesSpan, Node<Annotation>> {
    map_node(
        tuple((node(be_u16), collect(node(be_u16), element_value_pairs))),
        |(type_index, (num_element_value_pairs, element_value_pairs)): (
            Node<u16>,
            (Node<u16>, Nodes<ElementValuePair>),
        )| Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        },
    )(input)
}

fn element_value_pairs(input: BytesSpan) -> IResult<BytesSpan, Node<ElementValuePair>> {
    map_node(
        tuple((node(be_u16), element_value)),
        |(element_name_index, value): (Node<u16>, Node<ElementValue>)| ElementValuePair {
            element_name_index,
            value,
        },
    )(input)
}

fn element_value(input: BytesSpan) -> IResult<BytesSpan, Node<ElementValue>> {
    let (input, offset) = offset(input)?;
    let (input, tag) = node(be_u8)(input)?;
    let (input, value) = value(input, *tag)?;

    Ok((
        input,
        Node(offset..input.offset, ElementValue { tag, value }),
    ))
}

fn value(input: BytesSpan, tag: u8) -> IResult<BytesSpan, Node<Value>> {
    match tag as char {
        'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
            map_node(node(be_u16), |const_value_index: Node<u16>| {
                Value::ConstValue(ConstValue { const_value_index })
            })(input)
        }
        'e' => map_node(
            tuple((node(be_u16), node(be_u16))),
            |(type_name_index, const_name_index): (Node<u16>, Node<u16>)| {
                Value::EnumConstValue(EnumConstValue {
                    type_name_index,
                    const_name_index,
                })
            },
        )(input),
        'c' => map_node(node(be_u16), |class_info_index: Node<u16>| {
            Value::ClassInfo(ClassInfo { class_info_index })
        })(input),
        '@' => map_node(annotation, |annotation: Node<Annotation>| {
            Value::AnnotationValue(annotation.1)
        })(input),
        '[' => map_node(
            collect(node(be_u16), element_value),
            |(num_values, values): (Node<u16>, Nodes<ElementValue>)| {
                Value::ArrayValue(ArrayValue { num_values, values })
            },
        )(input),
        _ => Err(Error(make_error(input, ErrorKind::NoneOf))),
    }
}
