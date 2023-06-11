use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::asm::node::constant::ConstantPool;
use crate::asm::node::method::Method;
use crate::asm::parse::attribute::attribute_info;
use crate::asm::parse::{access_flag, collect_with_constant_pool};

pub(crate) fn methods<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], (u16, Vec<Method>)> {
    collect_with_constant_pool(be_u16, method, constant_pool)(input)
}

fn method<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Method> {
    map(
        tuple((
            access_flag,
            be_u16,
            be_u16,
            collect_with_constant_pool(be_u16, attribute_info, constant_pool),
        )),
        |(access_flags, name_index, descriptor_index, (attribute_infos_len, attribute_infos))| {
            Method {
                access_flags,
                name_index,
                descriptor_index,
                attribute_infos_len,
                attribute_infos,
            }
        },
    )(input)
}
