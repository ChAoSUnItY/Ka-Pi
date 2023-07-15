use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::attribute::{Attribute, LineNumber, LineNumberTable};
use crate::node::{Node, Nodes};
use crate::parse::{collect, map_node, node};

pub(crate) fn line_number_table(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), line_number),
        |(line_number_table_length, line_number_table): (Node<u16>, Nodes<LineNumber>)| {
            Attribute::LineNumberTable(LineNumberTable {
                line_number_table_length,
                line_number_table,
            })
        },
    )(input)
}

fn line_number(input: BytesSpan) -> IResult<BytesSpan, Node<LineNumber>> {
    map_node(
        tuple((node(be_u16), node(be_u16))),
        |(start_pc, line_number): (Node<u16>, Node<u16>)| LineNumber {
            start_pc,
            line_number,
        },
    )(input)
}
