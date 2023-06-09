use std::io::Read;

use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::number::complete::{be_u16, be_u32};
use nom::{IResult, Parser};

use crate::asm::node::access_flag::{AccessFlag, ClassAccessFlag};
use crate::asm::node::class::{Class, JavaVersion};
use crate::asm::parse::attribute::attribute_infos;
use crate::asm::parse::constant::constant_pool;
use crate::asm::parse::field::fields;
use crate::asm::parse::method::methods;

pub(crate) fn class(input: &[u8]) -> IResult<&[u8], Class> {
    let (input, _) = tag(&[0xCA, 0xFE, 0xBA, 0xBE])(input)?;
    let (input, java_version) = map_res(be_u32, JavaVersion::try_from)(input)?;
    let (input, (constant_pool_count, constant_pool)) = constant_pool(input)?;
    let (input, access_flags) = map(be_u16, ClassAccessFlag::mask_access_flags)(input)?;
    let (input, this_class) = be_u16(input)?;
    let (input, super_class) = be_u16(input)?;
    let (input, (interfaces_count, interfaces)) = interfaces(input)?;
    let (input, (fields_count, fields)) = fields(input, &constant_pool)?;
    let (input, (methods_count, methods)) = methods(input, &constant_pool)?;
    let (input, (attributes_count, attributes)) = attribute_infos(input, &constant_pool)?;

    Ok((
        input,
        Class {
            java_version,
            constant_pool_count,
            constant_pool,
            access_flags,
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
        },
    ))
}

fn interfaces(input: &[u8]) -> IResult<&[u8], (u16, Vec<u16>)> {
    let (mut input, len) = be_u16(input)?;
    let mut interfaces = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (remain, interface_index) = be_u16(input)?;

        interfaces.push(interface_index);
        input = remain;
    }

    Ok((input, (len, interfaces)))
}
