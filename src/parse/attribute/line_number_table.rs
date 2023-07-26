use crate::node::attribute::{Attribute, LineNumber, LineNumberTable};
use crate::parse::error::ParseResult;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

#[inline]
pub(super) fn line_number_table<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let line_number_table_length = input.read_u16::<BigEndian>()?;
    let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);

    for _ in 0..line_number_table_length {
        line_number_table.push(line_number(input)?);
    }

    Ok(Some(Attribute::LineNumberTable(LineNumberTable {
        line_number_table_length,
        line_number_table,
    })))
}

#[inline(always)]
fn line_number<R: Read>(input: &mut R) -> ParseResult<LineNumber> {
    let start_pc = input.read_u16::<BigEndian>()?;
    let line_number = input.read_u16::<BigEndian>()?;

    Ok(LineNumber {
        start_pc,
        line_number,
    })
}
