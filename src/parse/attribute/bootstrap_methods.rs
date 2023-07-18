use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::node::attribute::{Attribute, BootstrapMethod, BootstrapMethods};
use crate::parse::collect;

pub fn bootstrap_methods(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    map(
        collect(be_u16, bootstrap_method),
        |(num_bootstrap_methods, bootstrap_methods)| {
            Some(Attribute::BootstrapMethods(BootstrapMethods {
                num_bootstrap_methods,
                bootstrap_methods,
            }))
        },
    )(input)
}

fn bootstrap_method(input: &[u8]) -> IResult<&[u8], BootstrapMethod> {
    map(
        tuple((be_u16, collect(be_u16, be_u16))),
        |(bootstrap_method_ref, (num_bootstrap_arguments, bootstrap_arguments))| BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        },
    )(input)
}
