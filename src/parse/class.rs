use nom::combinator::map;
use nom::number::complete::{be_u16, be_u32};

use byte_span::BytesSpan;

use crate::node::class::{Class, JavaVersion};
use crate::node::Node;
use crate::parse::attribute::attribute_info;
use crate::parse::constant::constant_pool;
use crate::parse::field::fields;
use crate::parse::method::methods;
use crate::parse::{
    access_flag, collect, collect_with_constant_pool, node, tag_sized_node, ParseResult,
};

pub(crate) fn class(input: BytesSpan) -> ParseResult<Class> {
    let (input, magic_number) = tag_sized_node([0xCA, 0xFE, 0xBA, 0xBE])(input)?;
    let (input, java_version) =
        map(node(be_u32), |node: Node<u32>| node.map(JavaVersion::from))(input)?;
    let (input, (constant_pool_count, constant_pool)) = constant_pool(input)?;
    let (input, access_flags) = access_flag(input)?;
    let (input, this_class) = node(be_u16)(input)?;
    let (input, super_class) = node(be_u16)(input)?;
    let (input, (interfaces_count, interfaces)) = collect(node(be_u16), node(be_u16))(input)?;
    let (input, (fields_count, fields)) = fields(input, &constant_pool)?;
    let (input, (methods_count, methods)) = methods(input, &constant_pool)?;
    let (input, (attributes_count, attributes)) =
        collect_with_constant_pool(node(be_u16), attribute_info, &constant_pool)(input)?;

    Ok((
        input,
        Class {
            magic_number,
            java_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        },
    ))
}
