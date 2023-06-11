use nom::combinator::map;
use nom::IResult;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use crate::asm::node::access_flag::{AccessFlag, NestedClassAccessFlag};
use crate::asm::node::attribute::{Attribute, InnerClass, InnerClasses};
use crate::asm::parse::collect;

pub(crate) fn inner_classes(input: &[u8]) -> IResult<&[u8], Option<Attribute>> {
    let (input, (number_of_classes, class)) = collect(be_u16, inner_class)(input)?;
    
    Ok((input, Some(Attribute::InnerClasses(InnerClasses {
        number_of_classes,
        class
    }))))
}

fn inner_class(input: &[u8]) -> IResult<&[u8], InnerClass> {
    map(tuple((be_u16, be_u16, be_u16, be_u16)), |(inner_class_info_index, outer_class_info_index, inner_name_index, inner_class_access_flags)| InnerClass {
        inner_class_info_index,
        outer_class_info_index,
        inner_name_index,
        inner_class_access_flags: NestedClassAccessFlag::mask_access_flags(inner_class_access_flags)
    })(input)
}
