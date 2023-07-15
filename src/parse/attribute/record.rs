use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::attribute::{Attribute, AttributeInfo, Record, RecordComponent};
use crate::node::constant::ConstantPool;
use crate::node::{Node, Nodes};
use crate::parse::attribute::attribute_info;
use crate::parse::{collect_with_constant_pool, map_node, node};

pub(crate) fn record<'input: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'input>,
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<BytesSpan<'input>, Node<Attribute>> {
    map_node(
        collect_with_constant_pool(node(be_u16), record_component, constant_pool),
        |(components_count, components): (Node<u16>, Nodes<RecordComponent>)| {
            Attribute::Record(Record {
                components_count,
                components,
            })
        },
    )(input)
}

fn record_component<'input: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'input>,
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<BytesSpan<'input>, Node<RecordComponent>> {
    map_node(
        tuple((
            node(be_u16),
            node(be_u16),
            collect_with_constant_pool(node(be_u16), attribute_info, constant_pool),
        )),
        |(name_index, descriptor_index, (attributes_count, attributes)): (
            Node<u16>,
            Node<u16>,
            (Node<u16>, Nodes<AttributeInfo>),
        )| RecordComponent {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        },
    )(input)
}
