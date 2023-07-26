use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::attribute::{
    Attribute, Object, StackMapFrameEntry, StackMapTable, VerificationType,
};
use crate::parse::error::{ParseError, ParseResult};

#[inline]
pub(super) fn stack_map_table<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let number_of_entries = input.read_u16::<BigEndian>()?;
    let mut entries = Vec::with_capacity(number_of_entries as usize);

    for _ in 0..number_of_entries {
        entries.push(stack_map_frame_entry(input)?);
    }

    Ok(Some(Attribute::StackMapTable(StackMapTable {
        number_of_entries,
        entries,
    })))
}

fn stack_map_frame_entry<R: Read>(input: &mut R) -> ParseResult<StackMapFrameEntry> {
    let frame_type = input.read_u8()?;

    match frame_type {
        0..=63 => Ok(StackMapFrameEntry::Same { frame_type }),
        64..=127 => {
            let stack = verification_type(input)?;

            Ok(StackMapFrameEntry::SameLocal1StackItem { frame_type, stack })
        }
        247 => {
            let offset_delta = input.read_u16::<BigEndian>()?;
            let stack = verification_type(input)?;

            Ok(StackMapFrameEntry::SameLocal1StackItemExtended {
                frame_type,
                offset_delta,
                stack,
            })
        }
        248..=250 => {
            let offset_delta = input.read_u16::<BigEndian>()?;

            Ok(StackMapFrameEntry::Chop {
                frame_type,
                offset_delta,
            })
        }
        251 => {
            let offset_delta = input.read_u16::<BigEndian>()?;

            Ok(StackMapFrameEntry::SameExtended {
                frame_type,
                offset_delta,
            })
        }
        252..=254 => {
            let offset_delta = input.read_u16::<BigEndian>()?;
            let mut locals = Vec::with_capacity(frame_type as usize - 251);

            for _ in 0..frame_type - 251 {
                locals.push(verification_type(input)?);
            }

            Ok(StackMapFrameEntry::Append {
                frame_type,
                offset_delta,
                locals,
            })
        }
        255 => {
            let offset_delta = input.read_u16::<BigEndian>()?;
            let number_of_locals = input.read_u16::<BigEndian>()?;
            let mut locals = Vec::with_capacity(number_of_locals as usize);

            for _ in 0..number_of_locals {
                locals.push(verification_type(input)?);
            }

            let number_of_stack_items = input.read_u16::<BigEndian>()?;
            let mut stack = Vec::with_capacity(number_of_locals as usize);

            for _ in 0..number_of_stack_items {
                stack.push(verification_type(input)?);
            }

            Ok(StackMapFrameEntry::Full {
                frame_type,
                offset_delta,
                number_of_locals,
                locals,
                number_of_stack_items,
                stack,
            })
        }
        _ => Err(ParseError::MatchOutOfBoundUsize(
            "stack map frame entry",
            vec![
                "0..=63",
                "64..=127",
                "247",
                "248..=250",
                "251",
                "252..=254",
                "255",
            ],
            frame_type as usize,
        )),
    }
}

fn verification_type<R: Read>(input: &mut R) -> ParseResult<VerificationType> {
    let tag = input.read_u8()?;

    match tag {
        0 => Ok(VerificationType::Top),
        1 => Ok(VerificationType::Integer),
        2 => Ok(VerificationType::Float),
        3 => Ok(VerificationType::Double),
        4 => Ok(VerificationType::Long),
        5 => Ok(VerificationType::Null),
        6 => Ok(VerificationType::UninitializedThis),
        7 => {
            let cpool_index = input.read_u16::<BigEndian>()?;

            Ok(VerificationType::Object(Object { cpool_index }))
        }
        8 => {
            let offset = input.read_u16::<BigEndian>()?;

            Ok(VerificationType::Uninitialized { offset })
        }
        _ => Err(ParseError::MatchOutOfBoundUsize(
            "verification type",
            vec!["0..=8"],
            tag as usize,
        )),
    }
}
