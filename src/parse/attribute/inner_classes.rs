use nom::number::complete::be_u16;
use nom::sequence::tuple;

use byte_span::BytesSpan;

use crate::node::access_flag::NestedClassAccessFlag;
use crate::node::attribute::{Attribute, InnerClass, InnerClasses};
use crate::node::{Node, Nodes};
use crate::parse::{access_flag, collect, map_node, node, ParseResult};

pub(crate) fn inner_classes(input: BytesSpan) -> ParseResult<Node<Attribute>> {
    map_node(
        collect(node(be_u16), inner_class),
        |(number_of_classes, class): (Node<u16>, Nodes<InnerClass>)| {
            Attribute::InnerClasses(InnerClasses {
                number_of_classes,
                class,
            })
        },
    )(input)
}

fn inner_class(input: BytesSpan) -> ParseResult<Node<InnerClass>> {
    map_node(
        tuple((node(be_u16), node(be_u16), node(be_u16), access_flag)),
        |(
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        ): (
            Node<u16>,
            Node<u16>,
            Node<u16>,
            Node<Vec<NestedClassAccessFlag>>,
        )| InnerClass {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        },
    )(input)
}
