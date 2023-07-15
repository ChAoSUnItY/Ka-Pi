use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::attribute::{Attribute, BootstrapMethod, BootstrapMethods};
use crate::node::{Node, Nodes};
use crate::parse::{collect, map_node, node};

pub fn bootstrap_methods(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u16), bootstrap_method),
        |(num_bootstrap_methods, bootstrap_methods): (Node<u16>, Nodes<BootstrapMethod>)| {
            Attribute::BootstrapMethods(BootstrapMethods {
                num_bootstrap_methods,
                bootstrap_methods,
            })
        },
    )(input)
}

fn bootstrap_method(input: BytesSpan) -> IResult<BytesSpan, Node<BootstrapMethod>> {
    map_node(
        tuple((node(be_u16), collect(node(be_u16), node(be_u16)))),
        |(bootstrap_method_ref, (num_bootstrap_arguments, bootstrap_arguments)): (
            Node<u16>,
            (Node<u16>, Nodes<u16>),
        )| BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        },
    )(input)
}
