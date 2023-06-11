use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::asm::node::constant::ConstantPool;
use crate::asm::node::field::Field;
use crate::asm::parse::attribute::attribute_info;
use crate::asm::parse::{access_flag, collect_with_constant_pool};

pub(crate) fn fields<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], (u16, Vec<Field>)> {
    collect_with_constant_pool(be_u16, field, constant_pool)(input)
}

fn field<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Field> {
    map(
        tuple((
            access_flag,
            be_u16,
            be_u16,
            collect_with_constant_pool(be_u16, attribute_info, constant_pool),
        )),
        |(access_flags, name_index, descriptor_index, (attribute_infos_len, attribute_infos))| {
            Field {
                access_flags,
                name_index,
                descriptor_index,
                attribute_infos_len,
                attribute_infos,
            }
        },
    )(input)
}
