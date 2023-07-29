#[allow(unused_imports)]
use bitflags::Flags;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::access_flag::NestedClassAccessFlag;
use crate::node::attribute::{Attribute, InnerClass, InnerClasses};
use crate::parse::error::ParseResult;

#[inline]
pub(super) fn inner_classes<R: Read>(input: &mut R) -> ParseResult<Option<Attribute>> {
    let number_of_classes = input.read_u16::<BigEndian>()?;
    let mut class = Vec::with_capacity(number_of_classes as usize);

    for _ in 0..number_of_classes {
        class.push(inner_class(input)?);
    }

    Ok(Some(Attribute::InnerClasses(InnerClasses {
        number_of_classes,
        class,
    })))
}

#[inline(always)]
fn inner_class<R: Read>(input: &mut R) -> ParseResult<InnerClass> {
    let inner_class_info_index = input.read_u16::<BigEndian>()?;
    let outer_class_info_index = input.read_u16::<BigEndian>()?;
    let inner_name_index = input.read_u16::<BigEndian>()?;
    let inner_class_access_flags =
        NestedClassAccessFlag::from_bits_truncate(input.read_u16::<BigEndian>()?);

    Ok(InnerClass {
        inner_class_info_index,
        outer_class_info_index,
        inner_name_index,
        inner_class_access_flags,
    })
}
