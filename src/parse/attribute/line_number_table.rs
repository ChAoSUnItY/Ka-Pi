use crate::node::attribute::{Attribute, LineNumber, LineNumberTable};
use crate::parse::collect;
use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn line_number_table(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, line_number),
        |(line_number_table_length, line_number_table)| {
            Some(Attribute::LineNumberTable(LineNumberTable {
                line_number_table_length,
                line_number_table,
            }))
        },
    )(input)
}

fn line_number(input: &[u8]) -> IResult<&[u8], LineNumber> {
    map(tuple((be_u16, be_u16)), |(start_pc, line_number)| {
        LineNumber {
            start_pc,
            line_number,
        }
    })(input)
}
