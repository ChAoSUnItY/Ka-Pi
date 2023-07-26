use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::attribute::{Attribute, BootstrapMethod, BootstrapMethods};
use crate::parse::error::ParseResult;

#[inline]
pub(super) fn bootstrap_methods<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let num_bootstrap_methods = input.read_u16::<BigEndian>()?;
    let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);

    for _ in 0..num_bootstrap_methods {
        bootstrap_methods.push(bootstrap_method(input)?);
    }

    Ok(Some(Attribute::BootstrapMethods(BootstrapMethods {
        num_bootstrap_methods,
        bootstrap_methods,
    })))
}

#[inline(always)]
fn bootstrap_method<R: Read>(input: &mut R) -> ParseResult<BootstrapMethod> {
    let bootstrap_method_ref = input.read_u16::<BigEndian>()?;
    let num_bootstrap_arguments = input.read_u16::<BigEndian>()?;
    let mut bootstrap_arguments = vec![0; num_bootstrap_arguments as usize];

    for i in 0..num_bootstrap_arguments {
        bootstrap_arguments[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(BootstrapMethod {
        bootstrap_method_ref,
        num_bootstrap_arguments,
        bootstrap_arguments,
    })
}
