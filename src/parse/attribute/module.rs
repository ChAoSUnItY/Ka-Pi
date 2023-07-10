use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::node::attribute::module::{Exports, Opens, Provides, Requires};
use crate::node::attribute::{Attribute, Module};
use crate::parse::{access_flag, collect};

pub(crate) fn module(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        tuple((
            be_u16,
            access_flag,
            be_u16,
            collect(be_u16, requires),
            collect(be_u16, exports),
            collect(be_u16, opens),
            collect(be_u16, be_u16),
            collect(be_u16, provides),
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
        )| {
            Some(Attribute::Module(Module {
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
            }))
        },
    )(input)
}

fn requires(input: &[u8]) -> IResult<&[u8], Requires> {
    map(
        tuple((be_u16, access_flag, be_u16)),
        |(requires_index, requires_flags, requires_version_index)| Requires {
            requires_index,
            requires_flags,
            requires_version_index,
        },
    )(input)
}

fn exports(input: &[u8]) -> IResult<&[u8], Exports> {
    map(
        tuple((be_u16, access_flag, collect(be_u16, be_u16))),
        |(exports_index, exports_flags, (exports_to_count, exports_to_index))| Exports {
            exports_index,
            exports_flags,
            exports_to_count,
            exports_to_index,
        },
    )(input)
}

fn opens(input: &[u8]) -> IResult<&[u8], Opens> {
    map(
        tuple((be_u16, access_flag, collect(be_u16, be_u16))),
        |(opens_index, opens_flags, (opens_to_count, opens_to_index))| Opens {
            opens_index,
            opens_flags,
            opens_to_count,
            opens_to_index,
        },
    )(input)
}

fn provides(input: &[u8]) -> IResult<&[u8], Provides> {
    map(
        tuple((be_u16, collect(be_u16, be_u16))),
        |(provides_index, (provides_with_count, provides_with_index))| Provides {
            provides_index,
            provides_with_count,
            provides_with_index,
        },
    )(input)
}
