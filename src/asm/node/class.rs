use std::fs::File;
use std::io::Read;
use std::path::Path;

use itertools::Itertools;
use nom::bytes::complete::{tag, take};
use nom::combinator::{map, map_res};
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::tuple;
use nom::{IResult, Parser};
use strum::IntoEnumIterator;

use crate::asm::node::constant::{Constant, ConstantTag};
use crate::asm::node::utils::mask_access_flags;
use crate::asm::opcodes::{ClassAccessFlag, JavaVersion};
use crate::error::{KapiError, KapiResult};

#[derive(Debug)]
pub struct Class {
    java_version: JavaVersion,
    constant_pool: Vec<Constant>,
    access_flags: Vec<ClassAccessFlag>,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
}

pub fn read_class<P: AsRef<Path>>(class_path: P) -> KapiResult<Class> {
    let class_path = class_path.as_ref();
    let mut file = match File::open(class_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(KapiError::ClassParseError(format!(
                "Unable to open class file {}, reason: {}",
                class_path.display(),
                err
            )))
        }
    };
    let mut class_bytes = Vec::new();

    if let Err(err) = file.read_to_end(&mut class_bytes) {
        return Err(KapiError::ClassParseError(format!(
            "Unable to read class file {}, reason: {}",
            class_path.display(),
            err
        )));
    }

    match class(&class_bytes[..]) {
        Ok((remain, class)) => {
            // TODO: uncomment this when class parser is done
            // if !remain.is_empty() {
            //     Err(KapiError::ClassParseError(format!("Unable to parse class file {}, reason: class is fully parsed but there are {} bytes left", class_path.display(), remain.len())))
            // } else {
            //     Ok(class)
            // }

            Ok(class)
        }
        Err(err) => Err(KapiError::ClassParseError(format!(
            "Unable parse class file {}, reason: {}",
            class_path.display(),
            err
        ))),
    }
}

fn class(input: &[u8]) -> IResult<&[u8], Class> {
    let (input, _) = tag(&[0xCA, 0xFE, 0xBA, 0xBE])(input)?;
    let (input, java_version) = map_res(be_u32, JavaVersion::try_from)(input)?;
    let (input, constant_pool) = constant_pool(input)?;
    let (input, access_flags) = map(be_u16, mask_access_flags)(input)?;
    let (input, this_class) = be_u16(input)?;
    let (input, super_class) = be_u16(input)?;
    let (input, interfaces) = interfaces(input)?;

    Ok((
        input,
        Class {
            java_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
        },
    ))
}

fn constant_pool(input: &[u8]) -> IResult<&[u8], Vec<Constant>> {
    let (mut input, len) = be_u16(input)?;
    let mut constants = Vec::with_capacity(len as usize);

    for _ in 0..len - 1 {
        let (remain, constant) = constant(input)?;

        constants.push(constant);
        input = remain;
    }

    Ok((input, constants))
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

fn interfaces(input: &[u8]) -> IResult<&[u8], Vec<u16>> {
    let (mut input, len) = be_u16(input)?;
    let mut interfaces = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (remain, interface_index) = be_u16(input)?;

        interfaces.push(interface_index);
        input = remain;
    }

    Ok((input, interfaces))
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::asm::node::class::read_class;
    use crate::error::KapiResult;

    #[test]
    fn test_class_parse() -> KapiResult<()> {
        let mut class_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        class_path.push("compiled_source/Main.class");

        read_class(class_path)?;

        Ok(())
    }
}
