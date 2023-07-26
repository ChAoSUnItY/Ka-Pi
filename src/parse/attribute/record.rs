use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

use crate::node::attribute::{Attribute, Record, RecordComponent};
use crate::node::constant::ConstantPool;
use crate::parse::attribute::attribute_info;
use crate::parse::error::ParseResult;

#[inline]
pub(super) fn record<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
) -> ParseResult<Option<Attribute>> {
    let components_count = input.read_u16::<BigEndian>()?;
    let mut components = Vec::with_capacity(components_count as usize);

    for _ in 0..components_count {
        components.push(record_component(input, constant_pool)?);
    }

    Ok(Some(Attribute::Record(Record {
        components_count,
        components,
    })))
}

#[inline(always)]
fn record_component<'input: 'constant_pool, 'constant_pool, R: Read>(
    input: &'input mut R,
    constant_pool: &'constant_pool ConstantPool,
) -> ParseResult<RecordComponent> {
    let name_index = input.read_u16::<BigEndian>()?;
    let descriptor_index = input.read_u16::<BigEndian>()?;
    let attributes_count = input.read_u16::<BigEndian>()?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);

    for _ in 0..attributes_count {
        attributes.push(attribute_info(input, constant_pool)?);
    }

    Ok(RecordComponent {
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    })
}
