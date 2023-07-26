use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::attribute::{Attribute, LocalVariable, LocalVariableTable};
use crate::parse::error::ParseResult;

#[inline]
pub(super) fn local_variable_table<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let local_variable_table_length = input.read_u16::<BigEndian>()?;
    let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);

    for _ in 0..local_variable_table_length {
        local_variable_table.push(local_variable(input)?);
    }

    Ok(Some(Attribute::LocalVariableTable(LocalVariableTable {
        local_variable_table_length,
        local_variable_table,
    })))
}

#[inline(always)]
fn local_variable<R: Read>(input: &mut R) -> ParseResult<LocalVariable> {
    let start_pc = input.read_u16::<BigEndian>()?;
    let length = input.read_u16::<BigEndian>()?;
    let name_index = input.read_u16::<BigEndian>()?;
    let descriptor_index = input.read_u16::<BigEndian>()?;
    let index = input.read_u16::<BigEndian>()?;

    Ok(LocalVariable {
        start_pc,
        length,
        name_index,
        descriptor_index,
        index,
    })
}
