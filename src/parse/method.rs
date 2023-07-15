use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::access_flag::MethodAccessFlag;
use crate::node::attribute::AttributeInfo;
use crate::node::constant::ConstantPool;
use crate::node::method::Method;
use crate::node::{Node, Nodes};
use crate::parse::attribute::attribute_info;
use crate::parse::{access_flag, collect_with_constant_pool, node};

pub(crate) fn methods<'input: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'input>,
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<BytesSpan<'input>, (Node<u16>, Nodes<Method>)> {
    collect_with_constant_pool(node(be_u16), method, constant_pool)(input)
}

fn method<'input: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'input>,
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<BytesSpan<'input>, Node<Method>> {
    map(
        node(tuple((
            access_flag,
            node(be_u16),
            node(be_u16),
            collect_with_constant_pool(node(be_u16), attribute_info, constant_pool),
        ))),
        |Node(
            span,
            (access_flags, name_index, descriptor_index, (attribute_infos_len, attribute_infos)),
        ): Node<(
            Node<Vec<MethodAccessFlag>>,
            Node<u16>,
            Node<u16>,
            (Node<u16>, Nodes<AttributeInfo>),
        )>| {
            Node(
                span,
                Method {
                    access_flags,
                    name_index,
                    descriptor_index,
                    attribute_infos_len,
                    attribute_infos,
                },
            )
        },
    )(input)
}
