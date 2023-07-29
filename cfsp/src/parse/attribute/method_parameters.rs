use crate::node::access_flag::ParameterAccessFlag;
#[allow(unused_imports)]
use bitflags::Flags;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::attribute::{Attribute, MethodParameter, MethodParameters};
use crate::parse::error::ParseResult;

#[inline]
pub(super) fn method_parameters<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let parameters_count = input.read_u8()?;
    let mut parameters = Vec::with_capacity(parameters_count as usize);

    for _ in 0..parameters_count {
        parameters.push(method_parameter(input)?);
    }

    Ok(Some(Attribute::MethodParameters(MethodParameters {
        parameters_count,
        parameters,
    })))
}

#[inline(always)]
fn method_parameter<R: Read>(input: &mut R) -> ParseResult<MethodParameter> {
    let name_index = input.read_u16::<BigEndian>()?;
    let access_flags = ParameterAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);

    Ok(MethodParameter {
        name_index,
        access_flags,
    })
}
