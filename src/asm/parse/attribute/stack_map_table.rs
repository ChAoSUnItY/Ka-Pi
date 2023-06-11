use nom::{error_position, IResult};
use nom::combinator::map;
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::error::ErrorKind;
use crate::asm::node::attribute::{Attribute, Object, StackMapFrameEntry, StackMapTable, VerificationType};
use crate::asm::parse::collect;

pub(crate) fn stack_map_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, stack_map_frame_entry),
        |(number_of_entries, entries)| {
            Some(Attribute::StackMapTable(StackMapTable {
                number_of_entries,
                entries,
            }))
        },
    )(input)
}

fn stack_map_frame_entry(input: &[u8]) -> IResult<&[u8], StackMapFrameEntry> {
    let (input, frame_type) = be_u8(input)?;

    match frame_type {
        0..=63 => Ok((input, StackMapFrameEntry::Same { frame_type })),
        64..=127 => map(verification_type, |stack| {
            StackMapFrameEntry::SameLocal1StackItem { frame_type, stack }
        })(input),
        247 => map(
            tuple((be_u16, verification_type)),
            |(offset_delta, stack)| StackMapFrameEntry::SameLocal1StackItemExtended {
                frame_type,
                offset_delta,
                stack,
            },
        )(input),
        248..=250 => map(be_u16, |offset_delta| StackMapFrameEntry::Chop {
            frame_type,
            offset_delta,
        })(input),
        251 => map(be_u16, |offset_delta| StackMapFrameEntry::SameExtended {
            frame_type,
            offset_delta,
        })(input),
        252..=254 => {
            let (mut input, offset_delta) = be_u16(input)?;
            let mut locals = Vec::with_capacity(frame_type as usize - 251);

            for _ in 0..frame_type - 251 {
                let (remain, verification_type) = verification_type(input)?;

                locals.push(verification_type);
                input = remain;
            }

            Ok((
                input,
                StackMapFrameEntry::Append {
                    frame_type,
                    offset_delta,
                    locals,
                },
            ))
        }
        255 => map(
            tuple((
                be_u16,
                collect(be_u16, verification_type),
                collect(be_u16, verification_type),
            )),
            |(offset_delta, (number_of_locals, locals), (number_of_stack_items, stack))| {
                StackMapFrameEntry::Full {
                    frame_type,
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            },
        )(input),
        _ => Err(Error(error_position!(input, ErrorKind::OneOf))),
    }
}

fn verification_type(input: &[u8]) -> IResult<&[u8], VerificationType> {
    let (input, tag) = be_u8(input)?;

    match tag {
        0 => Ok((input, VerificationType::Top)),
        1 => Ok((input, VerificationType::Integer)),
        2 => Ok((input, VerificationType::Float)),
        3 => Ok((input, VerificationType::Double)),
        4 => Ok((input, VerificationType::Long)),
        5 => Ok((input, VerificationType::Null)),
        6 => Ok((input, VerificationType::UninitializedThis)),
        7 => map(be_u16, |cpool_index| {
            VerificationType::Object(Object { cpool_index })
        })(input),
        8 => map(be_u16, |offset| VerificationType::Uninitialized { offset })(input),
        _ => Err(Error(error_position!(input, ErrorKind::OneOf))),
    }
}
