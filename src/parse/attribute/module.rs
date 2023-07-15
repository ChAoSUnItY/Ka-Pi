use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use byte_span::BytesSpan;

use crate::node::access_flag::{
    ExportsAccessFlag, ModuleAccessFlag, OpensAccessFlag, RequiresAccessFlag,
};
use crate::node::attribute::module::{Exports, Opens, Provides, Requires};
use crate::node::attribute::{Attribute, Module};
use crate::node::{Node, Nodes};
use crate::parse::{access_flag, collect, map_node, node};

pub(crate) fn module(input: BytesSpan) -> IResult<BytesSpan, Node<Attribute>> {
    map_node(
        tuple((
            node(be_u16),
            access_flag,
            node(be_u16),
            collect(node(be_u16), requires),
            collect(node(be_u16), exports),
            collect(node(be_u16), opens),
            collect(node(be_u16), node(be_u16)),
            collect(node(be_u16), provides),
        )),
        |(
            module_name_index,
            module_flags,
            module_version_index,
            (requires_count, requires),
            (exports_count, exports),
            (opens_count, opens),
            (uses_count, uses_index),
            (provides_count, provides),
        ): (
            Node<u16>,
            Node<Vec<ModuleAccessFlag>>,
            Node<u16>,
            (Node<u16>, Nodes<Requires>),
            (Node<u16>, Nodes<Exports>),
            (Node<u16>, Nodes<Opens>),
            (Node<u16>, Nodes<u16>),
            (Node<u16>, Nodes<Provides>),
        )| {
            Attribute::Module(Module {
                module_name_index,
                module_flags,
                module_version_index,
                requires_count,
                requires,
                exports_count,
                exports,
                opens_count,
                opens,
                uses_count,
                uses_index,
                provides_count,
                provides,
            })
        },
    )(input)
}

fn requires(input: BytesSpan) -> IResult<BytesSpan, Node<Requires>> {
    map_node(
        tuple((node(be_u16), access_flag, node(be_u16))),
        |(requires_index, requires_flags, requires_version_index): (
            Node<u16>,
            Node<Vec<RequiresAccessFlag>>,
            Node<u16>,
        )| Requires {
            requires_index,
            requires_flags,
            requires_version_index,
        },
    )(input)
}

fn exports(input: BytesSpan) -> IResult<BytesSpan, Node<Exports>> {
    map_node(
        tuple((
            node(be_u16),
            access_flag,
            collect(node(be_u16), node(be_u16)),
        )),
        |(exports_index, exports_flags, (exports_to_count, exports_to_index)): (
            Node<u16>,
            Node<Vec<ExportsAccessFlag>>,
            (Node<u16>, Nodes<u16>),
        )| Exports {
            exports_index,
            exports_flags,
            exports_to_count,
            exports_to_index,
        },
    )(input)
}

fn opens(input: BytesSpan) -> IResult<BytesSpan, Node<Opens>> {
    map_node(
        tuple((
            node(be_u16),
            access_flag,
            collect(node(be_u16), node(be_u16)),
        )),
        |(opens_index, opens_flags, (opens_to_count, opens_to_index)): (
            Node<u16>,
            Node<Vec<OpensAccessFlag>>,
            (Node<u16>, Nodes<u16>),
        )| Opens {
            opens_index,
            opens_flags,
            opens_to_count,
            opens_to_index,
        },
    )(input)
}

fn provides(input: BytesSpan) -> IResult<BytesSpan, Node<Provides>> {
    map_node(
        tuple((node(be_u16), collect(node(be_u16), node(be_u16)))),
        |(provides_index, (provides_with_count, provides_with_index)): (
            Node<u16>,
            (Node<u16>, Nodes<u16>),
        )| Provides {
            provides_index,
            provides_with_count,
            provides_with_index,
        },
    )(input)
}
