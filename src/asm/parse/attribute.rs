mod code;
mod annotation;
mod stack_map_table;
mod inner_classes;

use nom::bytes::complete::take;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{error, IResult};

use crate::asm::node::attribute;
use crate::asm::node::attribute::{Attribute, AttributeInfo, BootstrapMethod, BootstrapMethods, ConstantValue, EnclosingMethod, Exceptions, LineNumber, LineNumberTable, NestHost, NestMembers, PermittedSubclasses, SourceFile};
use crate::asm::node::constant::{Constant, ConstantPool};
use crate::asm::parse::attribute::code::code;
use crate::asm::parse::attribute::inner_classes::inner_classes;
use crate::asm::parse::attribute::stack_map_table::stack_map_table;
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
            Some(Attribute::ConstantValue(ConstantValue {
                constant_value_index,
            }))
        })(input),
        attribute::CODE => code(input, constant_pool),
        attribute::STACK_MAP_TABLE => stack_map_table(input),
        attribute::EXCEPTIONS => exceptions(input),
        attribute::INNER_CLASSES => inner_classes(input),
        attribute::ENCLOSING_METHOD => enclosing_method(input),
        attribute::SYNTHETIC => Ok((&[], Some(Attribute::Synthetic))),
        attribute::SOURCE_FILE => source_file(input),
        attribute::LINE_NUMBER_TABLE => line_number_table(input),
        attribute::BOOTSTRAP_METHODS => bootstrap_methods_attribute(input),
        attribute::NEST_HOST => nest_host(input),
        attribute::NEST_MEMBERS => nest_members(input),
        attribute::PERMITTED_SUBCLASSES => permitted_subclasses(input),
        _ => Ok((&[], None)), // Discard input data to ignore unrecognized attribute
    }
}

fn exceptions(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, be_u16),
        |(number_of_exceptions, exception_index_table)| {
            Some(Attribute::Exceptions(Exceptions {
                number_of_exceptions,
                exception_index_table,
            }))
        },
    )(input)
}

fn enclosing_method(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(tuple((be_u16, be_u16)), |(class_index, method_index)| Some(Attribute::EnclosingMethod(EnclosingMethod {
        class_index,
        method_index
    })))(input)
}

fn source_file(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |source_file_index| {
        Some(Attribute::SourceFile(SourceFile { source_file_index }))
    })(input)
}

fn line_number_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, line_number),
        |(line_number_table_length, line_number_table)| {
            Some(Attribute::LineNumberTable(LineNumberTable {
                line_number_table_length,
                line_number_table,
            }))
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
            Some(Attribute::BootstrapMethods(BootstrapMethods {
                num_bootstrap_methods,
                bootstrap_methods,
            }))
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

fn nest_host(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(be_u16, |host_class_index| {
        Some(Attribute::NestHost(NestHost { host_class_index }))
    })(input)
}

fn nest_members(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(collect(be_u16, be_u16), |(number_of_classes, classes)| {
        Some(Attribute::NestMembers(NestMembers {
            number_of_classes,
            classes,
        }))
    })(input)
}

fn permitted_subclasses(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(collect(be_u16, be_u16), |(number_of_classes, classes)| {
        Some(Attribute::PermittedSubclasses(PermittedSubclasses {
            number_of_classes,
            classes,
        }))
    })(input)
}
