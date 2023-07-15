use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::access_flag::ParameterAccessFlag;
use crate::node::attribute::{Attribute, MethodParameter, MethodParameters};
use crate::node::{Node, Nodes};
use crate::parse::{access_flag, collect, map_node, node};

pub(crate) fn method_parameters(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        collect(node(be_u8), method_parameter),
        |(parameters_count, parameters): (Node<u8>, Nodes<MethodParameter>)| {
            Attribute::MethodParameters(MethodParameters {
                parameters_count,
                parameters,
            })
        },
    )(input)
}

fn method_parameter(input: BytesSpan) -> IResult<BytesSpan, Node<MethodParameter>> {
    map_node(
        tuple((node(be_u16), access_flag)),
        |(name_index, access_flags): (Node<u16>, Node<Vec<ParameterAccessFlag>>)| MethodParameter {
            name_index,
            access_flags,
        },
    )(input)
}
