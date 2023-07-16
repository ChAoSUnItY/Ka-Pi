use nom::combinator::map;
use nom::error::ErrorKind;
use nom::error_position;
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;

use byte_span::{offset, BytesSpan};

use crate::node::attribute::{
    Attribute, Object, StackMapFrameEntry, StackMapTable, VerificationType,
};
use crate::node::{Node, Nodes};
use crate::parse::{collect, map_node, node, ParseResult};

pub(crate) fn stack_map_table(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        collect(node(be_u16), stack_map_frame_entry),
        |(number_of_entries, entries): (Node<u16>, Nodes<StackMapFrameEntry>)| {
            Attribute::StackMapTable(StackMapTable {
                number_of_entries,
                entries,
            })
        },
    )(input)
}

fn stack_map_frame_entry(input: BytesSpan) -> ParseResult<Node<StackMapFrameEntry>> {
    let (input, entry_offset) = offset(input)?;
    let (input, frame_type) = node(be_u8)(input)?;
    let (input, frame_entry) = match frame_type.1 {
        0..=63 => Ok((
            input,
            StackMapFrameEntry::Same {
                frame_type: frame_type.clone(),
            },
        )),
        64..=127 => map(verification_type, |stack: Node<VerificationType>| {
            StackMapFrameEntry::SameLocal1StackItem {
                frame_type: frame_type.clone(),
                stack,
            }
        })(input),
        247 => map(
            tuple((node(be_u16), verification_type)),
            |(offset_delta, stack): (Node<u16>, Node<VerificationType>)| {
                StackMapFrameEntry::SameLocal1StackItemExtended {
                    frame_type: frame_type.clone(),
                    offset_delta,
                    stack,
                }
            },
        )(input),
        248..=250 => map(node(be_u16), |offset_delta: Node<u16>| {
            StackMapFrameEntry::Chop {
                frame_type: frame_type.clone(),
                offset_delta,
            }
        })(input),
        251 => map(node(be_u16), |offset_delta: Node<u16>| {
            StackMapFrameEntry::SameExtended {
                frame_type: frame_type.clone(),
                offset_delta,
            }
        })(input),
        252..=254 => {
            let (input, offset_delta) = node(be_u16)(input)?;
            let (mut input, locals_offset) = offset(input)?;
            let mut locals = Vec::with_capacity(*frame_type as usize - 251);

            for _ in 0..*frame_type - 251 {
                let (remain, verification_type) = verification_type(input)?;

                locals.push(verification_type);
                input = remain;
            }

            Ok((
                input,
                StackMapFrameEntry::Append {
                    frame_type,
                    offset_delta,
                    locals: Node(locals_offset..input.offset, locals),
                },
            ))
        }
        255 => map(
            tuple((
                node(be_u16),
                collect(node(be_u16), verification_type),
                collect(node(be_u16), verification_type),
            )),
            |(offset_delta, (number_of_locals, locals), (number_of_stack_items, stack)): (
                Node<u16>,
                (Node<u16>, Nodes<VerificationType>),
                (Node<u16>, Nodes<VerificationType>),
            )| {
                StackMapFrameEntry::Full {
                    frame_type: frame_type.clone(),
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            },
        )(input),
        _ => Err(Error(error_position!(input, ErrorKind::OneOf))),
    }?;

    Ok((input, Node(entry_offset..input.offset, frame_entry)))
}

fn verification_type(input: BytesSpan) -> ParseResult<Node<VerificationType>> {
    let (input, offset) = offset(input)?;
    let (input, tag) = be_u8(input)?;
    let (input, verification_type) = match tag {
        0 => Ok((input, VerificationType::Top)),
        1 => Ok((input, VerificationType::Integer)),
        2 => Ok((input, VerificationType::Float)),
        3 => Ok((input, VerificationType::Double)),
        4 => Ok((input, VerificationType::Long)),
        5 => Ok((input, VerificationType::Null)),
        6 => Ok((input, VerificationType::UninitializedThis)),
        7 => map(node(be_u16), |cpool_index: Node<u16>| {
            VerificationType::Object(Object { cpool_index })
        })(input),
        8 => map(node(be_u16), |offset: Node<u16>| {
            VerificationType::Uninitialized { offset }
        })(input),
        _ => Err(Error(error_position!(input, ErrorKind::OneOf))),
    }?;

    Ok((input, Node(offset..input.offset, verification_type)))
}
