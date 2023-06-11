use crate::asm::node::attribute::{Attribute, BootstrapMethod, BootstrapMethods};
use crate::asm::parse::collect;
use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::IResult;

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
    let (input, bootstrap_method_ref) = be_u16(input)?;
    let (input, (num_bootstrap_arguments, bootstrap_arguments)) = collect(be_u16, be_u16)(input)?;

    Ok((
        input,
        BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        },
    ))
}
