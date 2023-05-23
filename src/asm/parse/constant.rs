use nom::bytes::complete::take;
use nom::combinator::{map, map_res};
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::IResult;
use crate::asm::node::constant::{Constant, ConstantPool, ConstantTag};

pub(crate) fn constant_pool(input: &[u8]) -> IResult<&[u8], (u16, ConstantPool)> {
    let (mut input, len) = be_u16(input)?;
    let mut constants = ConstantPool::default();

    for _ in 0..len - 1 {
        let (remain, constant) = constant(input)?;

        constants.add(constant);
        input = remain;
    }

    Ok((input, (len, constants)))
}

fn constant(input: &[u8]) -> IResult<&[u8], Constant> {
    let (input, tag) = map_res(be_u8, ConstantTag::try_from)(input)?;
    let (input, constant) = match tag {
        ConstantTag::Class => map(be_u16, |name_index| Constant::Class { name_index })(input)?,
        ConstantTag::FieldRef => map(
            tuple((be_u16, be_u16)),
            |(class_index, name_and_type_index)| Constant::FieldRef {
                class_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::MethodRef => map(
            tuple((be_u16, be_u16)),
            |(class_index, name_and_type_index)| Constant::MethodRef {
                class_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::InterfaceMethodRef => map(
            tuple((be_u16, be_u16)),
            |(class_index, name_and_type_index)| Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::String => {
            map(be_u16, |string_index| Constant::String { string_index })(input)?
        }
        ConstantTag::Integer => map(take(4usize), |bytes: &[u8]| Constant::Integer {
            bytes: bytes
                .try_into()
                .expect("Expected 4 bytes for bytes of Constant Integer"),
        })(input)?,
        ConstantTag::Float => map(take(4usize), |bytes: &[u8]| Constant::Float {
            bytes: bytes
                .try_into()
                .expect("Expected 4 bytes for bytes of Constant Float"),
        })(input)?,
        ConstantTag::Long => map(
            tuple((take(4usize), take(4usize))),
            |(high_bytes, low_bytes): (&[u8], &[u8])| Constant::Long {
                high_bytes: high_bytes
                    .try_into()
                    .expect("Expected 4 bytes for high bytes of Constant Long"),
                low_bytes: low_bytes
                    .try_into()
                    .expect("Expected 4 bytes for low bytes of Constant Long"),
            },
        )(input)?,
        ConstantTag::Double => map(
            tuple((take(4usize), take(4usize))),
            |(high_bytes, low_bytes): (&[u8], &[u8])| Constant::Double {
                high_bytes: high_bytes
                    .try_into()
                    .expect("Expected 4 bytes for high bytes of Constant Double"),
                low_bytes: low_bytes
                    .try_into()
                    .expect("Expected 4 bytes for low bytes of Constant Double"),
            },
        )(input)?,
        ConstantTag::NameAndType => map(tuple((be_u16, be_u16)), |(name_index, type_index)| {
            Constant::NameAndType {
                name_index,
                type_index,
            }
        })(input)?,
        ConstantTag::Utf8 => {
            let (input, length) = be_u16(input)?;
            let (input, string) = map_res(take(length), cesu8::from_java_cesu8)(input)?;

            (
                input,
                Constant::Utf8 {
                    data: string.to_string(),
                },
            )
        }
        ConstantTag::MethodHandle => map(
            tuple((be_u8, be_u16)),
            |(reference_kind, reference_index)| Constant::MethodHandle {
                reference_kind,
                reference_index,
            },
        )(input)?,
        ConstantTag::MethodType => map(be_u16, |descriptor_index| Constant::MethodType {
            descriptor_index,
        })(input)?,
        ConstantTag::Dynamic => map(
            tuple((be_u16, be_u16)),
            |(bootstrap_method_attr_index, name_and_type_index)| Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::InvokeDynamic => map(
            tuple((be_u16, be_u16)),
            |(bootstrap_method_attr_index, name_and_type_index)| Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::Module => map(be_u16, |name_index| Constant::Module { name_index })(input)?,
        ConstantTag::Package => map(be_u16, |name_index| Constant::Package { name_index })(input)?,
    };

    Ok((input, constant))
}
