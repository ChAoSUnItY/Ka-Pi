use nom::combinator::{map, map_res};
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;

use byte_span::{offset, BytesSpan};

use crate::error::KapiError;
use crate::node::constant::{
    Class, Constant, ConstantInfo, ConstantPool, ConstantTag, Double, Dynamic, FieldRef, Float,
    Integer, InterfaceMethodRef, InvokeDynamic, Long, MethodHandle, MethodRef, MethodType, Module,
    NameAndType, Package, RefKind, Utf8,
};
use crate::node::{constant, Node};
use crate::parse::{collect, node, take_sized_node, ParseResult};

pub(crate) fn constant_pool(input: BytesSpan) -> ParseResult<(Node<u16>, Node<ConstantPool>)> {
    collect(node(be_u16), constant_info)(input)
}

fn constant_info(input: BytesSpan) -> ParseResult<Node<ConstantInfo>> {
    let (input, info_offset) = offset(input)?;
    let (input, tag) = node(map_res(be_u8, ConstantTag::try_from))(input)?;
    let (input, constant) = constant(input, tag.1)?;

    Ok((
        input,
        Node(info_offset..input.offset, ConstantInfo { tag, constant }),
    ))
}

fn constant(input: BytesSpan, tag: ConstantTag) -> ParseResult<Node<Constant>> {
    let (input, offset) = offset(input)?;
    let (input, constant) = match tag {
        ConstantTag::Utf8 => map(
            collect(node(be_u16), be_u8),
            |(length, bytes): (Node<u16>, Node<Vec<u8>>)| Constant::Utf8(Utf8 { length, bytes }),
        )(input)?,
        ConstantTag::Integer => map(take_sized_node::<4>(), |bytes: Node<[u8; 4]>| {
            Constant::Integer(Integer { bytes })
        })(input)?,
        ConstantTag::Float => map(take_sized_node::<4>(), |bytes: Node<[u8; 4]>| {
            Constant::Float(Float { bytes })
        })(input)?,
        ConstantTag::Long => map(
            tuple((take_sized_node::<4>(), take_sized_node::<4>())),
            |(high_bytes, low_bytes): (Node<[u8; 4]>, Node<[u8; 4]>)| {
                Constant::Long(Long {
                    high_bytes,
                    low_bytes,
                })
            },
        )(input)?,
        ConstantTag::Double => map(
            tuple((take_sized_node::<4>(), take_sized_node::<4>())),
            |(high_bytes, low_bytes): (Node<[u8; 4]>, Node<[u8; 4]>)| {
                Constant::Double(Double {
                    high_bytes,
                    low_bytes,
                })
            },
        )(input)?,
        ConstantTag::Class => map(node(be_u16), |name_index| {
            Constant::Class(Class { name_index })
        })(input)?,
        ConstantTag::String => map(node(be_u16), |string_index: Node<u16>| {
            Constant::String(constant::String { string_index })
        })(input)?,
        ConstantTag::FieldRef => map(
            tuple((node(be_u16), node(be_u16))),
            |(class_index, name_and_type_index): (Node<u16>, Node<u16>)| {
                Constant::FieldRef(FieldRef {
                    class_index,
                    name_and_type_index,
                })
            },
        )(input)?,
        ConstantTag::MethodRef => map(
            tuple((node(be_u16), node(be_u16))),
            |(class_index, name_and_type_index): (Node<u16>, Node<u16>)| {
                Constant::MethodRef(MethodRef {
                    class_index,
                    name_and_type_index,
                })
            },
        )(input)?,
        ConstantTag::InterfaceMethodRef => map(
            tuple((node(be_u16), node(be_u16))),
            |(class_index, name_and_type_index): (Node<u16>, Node<u16>)| {
                Constant::InterfaceMethodRef(InterfaceMethodRef {
                    class_index,
                    name_and_type_index,
                })
            },
        )(input)?,
        ConstantTag::NameAndType => map(
            tuple((node(be_u16), node(be_u16))),
            |(name_index, type_index): (Node<u16>, Node<u16>)| {
                Constant::NameAndType(NameAndType {
                    name_index,
                    type_index,
                })
            },
        )(input)?,
        ConstantTag::MethodHandle => map(
            tuple((node(be_u8), node(be_u16))),
            |(reference_kind, reference_index): (Node<u8>, Node<u16>)| {
                let reference_kind = reference_kind.map(|kind| RefKind::try_from(kind).map_err(|err| {
                    KapiError::ClassParseError(format!(
                        "Reference kind {} does not match any kinds described in specification, reason: {}",
                        err.number,
                        err.to_string()
                    ))
                }).unwrap());

                Constant::MethodHandle(MethodHandle {
                    reference_kind,
                    reference_index,
                })
            },
        )(input)?,
        ConstantTag::MethodType => map(node(be_u16), |descriptor_index| {
            Constant::MethodType(MethodType { descriptor_index })
        })(input)?,
        ConstantTag::Dynamic => map(
            tuple((node(be_u16), node(be_u16))),
            |(bootstrap_method_attr_index, name_and_type_index): (Node<u16>, Node<u16>)| {
                Constant::Dynamic(Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            },
        )(input)?,
        ConstantTag::InvokeDynamic => map(
            tuple((node(be_u16), node(be_u16))),
            |(bootstrap_method_attr_index, name_and_type_index): (Node<u16>, Node<u16>)| {
                Constant::InvokeDynamic(InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                })
            },
        )(input)?,
        ConstantTag::Module => map(node(be_u16), |name_index: Node<u16>| {
            Constant::Module(Module { name_index })
        })(input)?,
        ConstantTag::Package => map(node(be_u16), |name_index: Node<u16>| {
            Constant::Package(Package { name_index })
        })(input)?,
    };

    Ok((input, Node(offset..input.offset, constant)))
}
