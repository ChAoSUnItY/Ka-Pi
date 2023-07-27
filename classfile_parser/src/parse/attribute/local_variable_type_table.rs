use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::attribute::{Attribute, LocalVariableType, LocalVariableTypeTable};
use crate::parse::error::ParseResult;

#[inline]
pub(super) fn local_variable_type_table<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let local_variable_type_table_length = input.read_u16::<BigEndian>()?;
    let mut local_variable_type_table =
        Vec::with_capacity(local_variable_type_table_length as usize);

    for _ in 0..local_variable_type_table_length {
        local_variable_type_table.push(local_variable_type(input)?);
    }

    Ok(Some(Attribute::LocalVariableTypeTable(
        LocalVariableTypeTable {
            local_variable_type_table_length,
            local_variable_type_table,
        },
    )))
}

#[inline(always)]
fn local_variable_type<R: Read>(input: &mut R) -> ParseResult<LocalVariableType> {
    let start_pc = input.read_u16::<BigEndian>()?;
    let length = input.read_u16::<BigEndian>()?;
    let name_index = input.read_u16::<BigEndian>()?;
    let signature_index = input.read_u16::<BigEndian>()?;
    let index = input.read_u16::<BigEndian>()?;

    Ok(LocalVariableType {
        start_pc,
        length,
        name_index,
        signature_index,
        index,
    })
}
