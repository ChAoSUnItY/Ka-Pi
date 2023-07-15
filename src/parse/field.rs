use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::access_flag::FieldAccessFlag;
use crate::node::attribute::AttributeInfo;
use crate::node::constant::ConstantPool;
use crate::node::field::Field;
use crate::node::Node;
use crate::parse::attribute::attribute_info;
use crate::parse::{access_flag, collect_with_constant_pool, node};

pub(crate) fn fields<'input: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'input>,
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<BytesSpan<'input>, (Node<u16>, Node<Vec<Node<Field>>>)> {
    collect_with_constant_pool(node(be_u16), field, constant_pool)(input)
}

fn field<'input: 'constant_pool, 'constant_pool>(
    input: BytesSpan<'input>,
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<BytesSpan<'input>, Node<Field>> {
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
            Node<Vec<FieldAccessFlag>>,
            Node<u16>,
            Node<u16>,
            (Node<u16>, Node<Vec<Node<AttributeInfo>>>),
        )>| {
            Node(
                span,
                Field {
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
