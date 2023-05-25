use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{error, error_position, IResult};

use crate::asm::node::attribute;
use crate::asm::node::attribute::{
    Attribute, AttributeInfo, BootstrapMethod, Exception, LineNumber, StackMapFrameEntry,
    VerificationType,
};
use crate::asm::node::constant::{Constant, ConstantPool};
use crate::asm::parse::collect;

pub(crate) fn attribute_infos<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], (u16, Vec<AttributeInfo>)> {
    let (mut input, len) = be_u16(input)?;
    let mut attributes = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (remain, attribute) = attribute_info(input, constant_pool)?;

        attributes.push(attribute);
        input = remain;
    }

    Ok((input, (len, attributes)))
}

fn attribute_info<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], AttributeInfo> {
    let (input, attribute_name_index) = be_u16(input)?;
    let (input, attribute_len) = be_u32(input)?;
    let (input, info) = take(attribute_len as usize)(input)?;
    let name_constant = constant_pool.get(attribute_name_index);

    let attribute = if let Some(Constant::Utf8(constant)) = name_constant {
        if let Ok(attribute_name) = constant.string() {
            let (remain, attribute) = attribute(info, constant_pool, &attribute_name)?;

            if !remain.is_empty() {
                return Err(Error(error::Error::new(remain, ErrorKind::NonEmpty)));
            }

            attribute
        } else {
            None
        }
    } else {
        None
    };

    Ok((
        input,
        AttributeInfo {
            attribute_name_index,
            attribute_len,
            info: info.to_vec(),
            attribute,
        },
    ))
}

fn attribute<'input: 'constant_pool, 'constant_pool: 'data, 'data>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
    data: &'data str,
) -> IResult<&'input [u8], Option<Attribute>> {
    match data {
        attribute::CONSTANT_VALUE => map(be_u16, |constant_value_index| {
            Some(Attribute::ConstantValue {
                constant_value_index,
            })
        })(input),
        attribute::CODE => code(input, constant_pool),
        attribute::STACK_MAP_TABLE => stack_map_table(input),
        attribute::SOURCE_FILE => source_file(input),
        attribute::LINE_NUMBER_TABLE => line_number_table(input),
        attribute::BOOTSTRAP_METHODS => bootstrap_methods_attribute(input),
        _ => Ok((&[], None)), // Discard input data to ignore unrecognized attribute
    }
}

fn code<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Option<Attribute>> {
    let (input, max_stack) = be_u16(input)?;
    let (input, max_locals) = be_u16(input)?;
    let (input, code_length) = be_u32(input)?;
    let (input, code) = take(code_length as usize)(input)?;
    let (input, (exception_table_length, exception_table)) = collect(be_u16, exception)(input)?;
    let (input, (attributes_length, attributes)) = attribute_infos(input, constant_pool)?;

    Ok((
        input,
        Some(Attribute::Code {
            max_stack,
            max_locals,
            code_length,
            code: code.to_vec(),
            exception_table_length,
            exception_table,
            attributes_length,
            attributes,
        }),
    ))
}

fn exception(input: &[u8]) -> IResult<&[u8], Exception> {
    map(
        tuple((be_u16, be_u16, be_u16, be_u16)),
        |(start_pc, end_pc, handler_pc, catch_type)| Exception {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        },
    )(input)
}

fn stack_map_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, stack_map_frame_entry),
        |(number_of_entries, entries)| {
            Some(Attribute::StackMapTable {
                number_of_entries,
                entries,
            })
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
        7 => map(be_u16, |cpool_index| VerificationType::Object {
            cpool_index,
        })(input),
        8 => map(be_u16, |offset| VerificationType::Uninitialized { offset })(input),
        _ => Err(Error(error_position!(input, ErrorKind::OneOf))),
    }
}

fn exceptions_attribute(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, be_u16),
        |(number_of_exceptions, exception_index_table)| {
            Some(Attribute::Exceptions {
                number_of_exceptions,
                exception_index_table,
            })
        },
    )(input)
}

fn source_file(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |source_file_index| {
        Some(Attribute::SourceFile { source_file_index })
    })(input)
}

fn line_number_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, line_number),
        |(line_number_table_length, line_number_table)| {
            Some(Attribute::LineNumberTable {
                line_number_table_length,
                line_number_table,
            })
        },
    )(input)
}

fn line_number(input: &[u8]) -> IResult<&[u8], LineNumber> {
    map(tuple((be_u16, be_u16)), |(start_pc, line_number)| {
        LineNumber {
            start_pc,
            line_number,
        }
    })(input)
}

fn bootstrap_methods_attribute(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, bootstrap_method),
        |(num_bootstrap_methods, bootstrap_methods)| {
            Some(Attribute::BootstrapMethods {
                num_bootstrap_methods,
                bootstrap_methods,
            })
        },
    )(input)
}

fn bootstrap_method(input: &[u8]) -> IResult<&[u8], BootstrapMethod> {
    let (input, bootstrap_method_ref) = be_u16(input)?;
    let (input, (num_bootstrap_arguments, bootstrap_arguments)) = collect(be_u16, be_u16)(input)?;

    Ok((
        input,
        BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        },
    ))
}
