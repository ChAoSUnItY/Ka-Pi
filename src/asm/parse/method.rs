use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::IResult;

use crate::asm::node::access_flag::{AccessFlag, MethodAccessFlag};
use crate::asm::node::constant::ConstantPool;
use crate::asm::node::method::Method;
use crate::asm::parse::attribute::attribute_infos;

pub(crate) fn methods<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], (u16, Vec<Method>)> {
    let (mut input, len) = be_u16(input)?;
    let mut methods = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (remain, method) = method(input, constant_pool)?;

        methods.push(method);
        input = remain;
    }

    Ok((input, (len, methods)))
}

fn method<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Method> {
    let (input, access_flags) = map(be_u16, MethodAccessFlag::mask_access_flags)(input)?;
    let (input, name_index) = be_u16(input)?;
    let (input, descriptor_index) = be_u16(input)?;
    let (input, (attribute_infos_len, attribute_infos)) = attribute_infos(input, constant_pool)?;

    Ok((
        input,
        Method {
            access_flags,
            name_index,
            descriptor_index,
            attribute_infos_len,
            attribute_infos,
        },
    ))
}
