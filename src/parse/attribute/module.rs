use crate::node::access_flag::{
    ExportsAccessFlag, ModuleAccessFlag, OpensAccessFlag, RequiresAccessFlag,
};
#[allow(unused_imports)]
use bitflags::Flags;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::attribute::module::{Exports, Opens, Provides, Requires};
use crate::node::attribute::{Attribute, Module};
use crate::parse::error::ParseResult;

pub(super) fn module<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let module_name_index = input.read_u16::<BigEndian>()?;
    let module_flags = ModuleAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let module_version_index = input.read_u16::<BigEndian>()?;
    let requires_count = input.read_u16::<BigEndian>()?;
    let mut requires = Vec::with_capacity(requires_count as usize);

    for _ in 0..requires_count {
        requires.push(parse_requires(input)?);
    }

    let exports_count = input.read_u16::<BigEndian>()?;
    let mut exports = Vec::with_capacity(exports_count as usize);

    for _ in 0..exports_count {
        exports.push(parse_exports(input)?);
    }

    let opens_count = input.read_u16::<BigEndian>()?;
    let mut opens = Vec::with_capacity(opens_count as usize);

    for _ in 0..opens_count {
        opens.push(parse_opens(input)?);
    }

    let uses_count = input.read_u16::<BigEndian>()?;
    let mut uses_index = vec![0; uses_count as usize];

    for i in 0..uses_count {
        uses_index[i as usize] = input.read_u16::<BigEndian>()?;
    }

    let provides_count = input.read_u16::<BigEndian>()?;
    let mut provides = Vec::with_capacity(provides_count as usize);

    for _ in 0..provides_count {
        provides.push(parse_provides(input)?);
    }

    Ok(Some(Attribute::Module(Module {
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
    })))
}

#[inline(always)]
fn parse_requires<R: Read>(input: &mut R) -> ParseResult<Requires> {
    let requires_index = input.read_u16::<BigEndian>()?;
    let requires_flags = RequiresAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let requires_version_index = input.read_u16::<BigEndian>()?;

    Ok(Requires {
        requires_index,
        requires_flags,
        requires_version_index,
    })
}

#[inline(always)]
fn parse_exports<R: Read>(input: &mut R) -> ParseResult<Exports> {
    let exports_index = input.read_u16::<BigEndian>()?;
    let exports_flags = ExportsAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let exports_to_count = input.read_u16::<BigEndian>()?;
    let mut exports_to_index = vec![0; exports_to_count as usize];

    for i in 0..exports_to_count {
        exports_to_index[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Exports {
        exports_index,
        exports_flags,
        exports_to_count,
        exports_to_index,
    })
}

#[inline(always)]
fn parse_opens<R: Read>(input: &mut R) -> ParseResult<Opens> {
    let opens_index = input.read_u16::<BigEndian>()?;
    let opens_flags = OpensAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);
    let opens_to_count = input.read_u16::<BigEndian>()?;
    let mut opens_to_index = vec![0; opens_to_count as usize];

    for i in 0..opens_to_count {
        opens_to_index[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Opens {
        opens_index,
        opens_flags,
        opens_to_count,
        opens_to_index,
    })
}

#[inline(always)]
fn parse_provides<R: Read>(input: &mut R) -> ParseResult<Provides> {
    let provides_index = input.read_u16::<BigEndian>()?;
    let provides_with_count = input.read_u16::<BigEndian>()?;
    let mut provides_with_index = vec![0; provides_with_count as usize];

    for i in 0..provides_with_count {
        provides_with_index[i as usize] = input.read_u16::<BigEndian>()?;
    }

    Ok(Provides {
        provides_index,
        provides_with_count,
        provides_with_index,
    })
}
