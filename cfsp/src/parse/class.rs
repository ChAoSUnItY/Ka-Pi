#[allow(unused_imports)]
use bitflags::Flags;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::access_flag::ClassAccessFlag;
use crate::node::class::{Class, JavaVersion};
use crate::parse::attribute::attribute_info;
use crate::parse::constant::constant_pool;
use crate::parse::error::{ParseError, ParseResult};
use crate::parse::field::fields;
use crate::parse::method::methods;
use crate::parse::ParsingOption;

pub(crate) fn class<R: Read>(input: &mut R, option: ParsingOption) -> ParseResult<Class> {
    let mut magic_number = [0; 4];

    input.read_exact(&mut magic_number)?;

    match magic_number {
        [0xCA, 0xFE, 0xBA, 0xBE] => {}
        _ => return Err(ParseError::MismatchedMagicNumber(magic_number)),
    }

    let java_version = JavaVersion::from(input.read_u32::<BigEndian>()?);
    let (constant_pool_count, constant_pool) = constant_pool(input)?;
    let access_flags = ClassAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let this_class = input.read_u16::<BigEndian>()?;
    let super_class = input.read_u16::<BigEndian>()?;
    let (interfaces_count, interfaces) = interfaces(input)?;
    let (fields_count, fields) = fields(input, &constant_pool, &option)?;
    let (methods_count, methods) = methods(input, &constant_pool, &option)?;
    let attributes_count = input.read_u16::<BigEndian>()?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);

    for _ in 0..attributes_count {
        attributes.push(attribute_info(input, &constant_pool, &option)?);
    }

    Ok(Class {
        java_version,
        constant_pool_count,
        constant_pool,
        access_flag: access_flags,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
        attributes_count,
        attributes,
    })
}

#[inline(always)]
fn interfaces<R: Read>(input: &mut R) -> ParseResult<(u16, Vec<u16>)> {
    let interfaces_length = input.read_u16::<BigEndian>()?;
    let mut interfaces = Vec::with_capacity(interfaces_length as usize);

    for _ in 0..interfaces_length {
        interfaces.push(input.read_u16::<BigEndian>()?);
    }

    Ok((interfaces_length, interfaces))
}
