use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::IResult;

use crate::asm::node::access_flag::{AccessFlag, FieldAccessFlag};
use crate::asm::node::constant::ConstantPool;
use crate::asm::node::field::Field;
use crate::asm::parse::attribute::attribute_infos;

pub(crate) fn fields<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], (u16, Vec<Field>)> {
    let (mut input, len) = be_u16(input)?;
    let mut fields = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (remain, field) = field(input, constant_pool)?;

        fields.push(field);
        input = remain;
    }

    Ok((input, (len, fields)))
}

fn field<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Field> {
    let (input, access_flags) = map(be_u16, FieldAccessFlag::mask_access_flags)(input)?;
    let (input, name_index) = be_u16(input)?;
    let (input, descriptor_index) = be_u16(input)?;
    let (input, (attribute_infos_len, attribute_infos)) = attribute_infos(input, constant_pool)?;

    Ok((
        input,
        Field {
            access_flags,
            name_index,
            descriptor_index,
            attribute_infos_len,
            attribute_infos,
        },
    ))
}
