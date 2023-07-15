use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::attribute::{Attribute, LocalVariable, LocalVariableTable};
use crate::node::{Node, Nodes};
use crate::parse::{collect, map_node, node};

pub(crate) fn local_variable_table(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), local_variable),
        |(local_variable_table_length, local_variable_table): (Node<u16>, Nodes<LocalVariable>)| {
            Attribute::LocalVariableTable(LocalVariableTable {
                local_variable_table_length,
                local_variable_table,
            })
        },
    )(input)
}

fn local_variable(input: BytesSpan) -> IResult<BytesSpan, Node<LocalVariable>> {
    map_node(
        tuple((
            node(be_u16),
            node(be_u16),
            node(be_u16),
            node(be_u16),
            node(be_u16),
        )),
        |(start_pc, length, name_index, descriptor_index, index): (
            Node<u16>,
            Node<u16>,
            Node<u16>,
            Node<u16>,
            Node<u16>,
        )| LocalVariable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        },
    )(input)
}
