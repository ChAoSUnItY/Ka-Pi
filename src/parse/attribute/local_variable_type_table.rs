use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::attribute::{Attribute, LocalVariableType, LocalVariableTypeTable};
use crate::node::{Node, Nodes};
use crate::parse::{collect, map_node, node};

pub(crate) fn local_variable_type_table(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), local_variable_type),
        |(local_variable_type_table_length, local_variable_type_table): (
            Node<u16>,
            Nodes<LocalVariableType>,
        )| {
            Attribute::LocalVariableTypeTable(LocalVariableTypeTable {
                local_variable_type_table_length,
                local_variable_type_table,
            })
        },
    )(input)
}

fn local_variable_type(input: BytesSpan) -> IResult<BytesSpan, Node<LocalVariableType>> {
    map_node(
        tuple((
            node(be_u16),
            node(be_u16),
            node(be_u16),
            node(be_u16),
            node(be_u16),
        )),
        |(start_pc, length, name_index, signature_index, index): (
            Node<u16>,
            Node<u16>,
            Node<u16>,
            Node<u16>,
            Node<u16>,
        )| LocalVariableType {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        },
    )(input)
}
