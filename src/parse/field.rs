use crate::node::access_flag::FieldAccessFlag;
#[allow(unused_imports)]
use bitflags::Flags;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::constant::ConstantPool;
use crate::node::field::Field;
use crate::parse::attribute::attribute_info;
use crate::parse::error::ParseResult;
use crate::parse::ParsingOption;

#[inline]
pub(crate) fn fields<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    option: &ParsingOption,
) -> ParseResult<(u16, Vec<Field>)> {
    let fields_length = input.read_u16::<BigEndian>()?;
    let mut fields = Vec::with_capacity(fields_length as usize);

    for _ in 0..fields_length {
        fields.push(field(input, constant_pool, option)?);
    }

    Ok((fields_length, fields))
}

#[inline(always)]
fn field<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
    option: &ParsingOption,
) -> ParseResult<Field> {
    let access_flags = FieldAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let name_index = input.read_u16::<BigEndian>()?;
    let descriptor_index = input.read_u16::<BigEndian>()?;
    let attribute_infos_len = input.read_u16::<BigEndian>()?;
    let mut attribute_infos = Vec::with_capacity(attribute_infos_len as usize);

    for _ in 0..attribute_infos_len {
        attribute_infos.push(attribute_info(input, constant_pool, option)?);
    }

    Ok(Field {
        access_flag: access_flags,
        name_index,
        descriptor_index,
        attribute_infos_len,
        attribute_infos,
    })
}
