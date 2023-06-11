use nom::combinator::map;
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::IResult;

use crate::asm::node::attribute::{Attribute, MethodParameter, MethodParameters};
use crate::asm::parse::{access_flag, collect};

pub(crate) fn method_parameters(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u8, method_parameter),
        |(parameters_count, parameters)| {
            Some(Attribute::MethodParameters(MethodParameters {
                parameters_count,
                parameters,
            }))
        },
    )(input)
}

fn method_parameter(input: &[u8]) -> IResult<&[u8], MethodParameter> {
    map(
        tuple((be_u16, access_flag)),
        |(name_index, access_flags)| MethodParameter {
            name_index,
            access_flags,
        },
    )(input)
}
