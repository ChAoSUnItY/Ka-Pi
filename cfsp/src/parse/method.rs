use crate::node::access_flag::MethodAccessFlag;
#[allow(unused_imports)]
use bitflags::Flags;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::constant::ConstantPool;
use crate::node::method::Method;
use crate::parse::attribute::attribute_info;
use crate::parse::error::ParseResult;
use crate::parse::ParsingOption;

pub(crate) fn methods<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    option: &ParsingOption,
) -> ParseResult<(u16, Vec<Method>)> {
    let methods_length = input.read_u16::<BigEndian>()?;
    let mut methods = Vec::with_capacity(methods_length as usize);

    for _ in 0..methods_length {
        methods.push(method(input, constant_pool, option)?);
    }

    Ok((methods_length, methods))
}

fn method<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    option: &ParsingOption,
) -> ParseResult<Method> {
    let access_flags = MethodAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let name_index = input.read_u16::<BigEndian>()?;
    let descriptor_index = input.read_u16::<BigEndian>()?;
    let attribute_infos_len = input.read_u16::<BigEndian>()?;
    let mut attribute_infos = Vec::with_capacity(attribute_infos_len as usize);

    for _ in 0..attribute_infos_len {
        attribute_infos.push(attribute_info(input, constant_pool, option)?);
    }

    Ok(Method {
        access_flag: access_flags,
        name_index,
        descriptor_index,
        attribute_infos_len,
        attribute_infos,
    })
}
