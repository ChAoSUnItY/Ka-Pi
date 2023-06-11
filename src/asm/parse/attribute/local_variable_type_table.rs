use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::asm::node::attribute::{Attribute, LocalVariableType, LocalVariableTypeTable};
use crate::asm::parse::collect;

pub(crate) fn local_variable_type_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, local_variable_type),
        |(local_variable_type_table_length, local_variable_type_table)| {
            Some(Attribute::LocalVariableTypeTable(LocalVariableTypeTable {
                local_variable_type_table_length,
                local_variable_type_table,
            }))
        },
    )(input)
}

fn local_variable_type(input: &[u8]) -> IResult<&[u8], LocalVariableType> {
    map(
        tuple((be_u16, be_u16, be_u16, be_u16, be_u16)),
        |(start_pc, length, name_index, signature_index, index)| LocalVariableType {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        },
    )(input)
}
