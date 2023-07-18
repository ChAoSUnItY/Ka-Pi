use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::node::attribute::{Attribute, LocalVariable, LocalVariableTable};
use crate::parse::collect;

pub(crate) fn local_variable_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, local_variable),
        |(local_variable_table_length, local_variable_table)| {
            Some(Attribute::LocalVariableTable(LocalVariableTable {
                local_variable_table_length,
                local_variable_table,
            }))
        },
    )(input)
}

fn local_variable(input: &[u8]) -> IResult<&[u8], LocalVariable> {
    map(
        tuple((be_u16, be_u16, be_u16, be_u16, be_u16)),
        |(start_pc, length, name_index, descriptor_index, index)| LocalVariable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        },
    )(input)
}
