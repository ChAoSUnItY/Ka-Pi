use nom::bytes::complete::take;
use nom::IResult;
use nom::number::complete::{be_u16, be_u32};
use crate::asm::node::attribute::{Attribute, Code};
use crate::asm::node::constant::ConstantPool;
use crate::asm::parse::attribute::{attribute_infos, exception};
use crate::asm::parse::collect;

pub(crate) fn code<'input: 'constant_pool, 'constant_pool>(
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
        Some(Attribute::Code(Code {
            max_stack,
            max_locals,
            code_length,
            code: code.to_vec(),
            instructions: vec![],
            exception_table_length,
            exception_table,
            attributes_length,
            attributes,
        })),
    ))
}

fn instruction<'input: 'constant_pool, 'constant_pool>() {
    
}
