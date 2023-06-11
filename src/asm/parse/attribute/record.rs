use nom::combinator::map;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::asm::node::attribute::{Attribute, Record, RecordComponent};
use crate::asm::node::constant::ConstantPool;
use crate::asm::parse::attribute::attribute_info;
use crate::asm::parse::collect_with_constant_pool;

pub(crate) fn record<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], Option<Attribute>> {
    map(
        collect_with_constant_pool(be_u16, record_component, constant_pool),
        |(components_count, components)| {
            Some(Attribute::Record(Record {
                components_count,
                components,
            }))
        },
    )(input)
}

fn record_component<'input: 'constant_pool, 'constant_pool>(
    input: &'input [u8],
    constant_pool: &'constant_pool ConstantPool,
) -> IResult<&'input [u8], RecordComponent> {
    map(
        tuple((
            be_u16,
            be_u16,
            collect_with_constant_pool(be_u16, attribute_info, constant_pool),
        )),
        |(name_index, descriptor_index, (attributes_count, attributes))| RecordComponent {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        },
    )(input)
}
