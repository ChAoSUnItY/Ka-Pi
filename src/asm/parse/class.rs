use std::fs::File;
use std::io::Read;
use std::path::Path;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::number::complete::{be_u16, be_u32};
use nom::{IResult, Parser};
use strum::IntoEnumIterator;

use crate::asm::node::access_flag::{AccessFlag, ClassAccessFlag};
use crate::asm::node::class::{Class, JavaVersion};
use crate::asm::parse::constant::constant_pool;
use crate::error::{KapiError, KapiResult};

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
    let (input, access_flags) = map(be_u16, ClassAccessFlag::mask_access_flags)(input)?;
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

    use crate::asm::parse::class::read_class;
    use crate::error::KapiResult;

    #[test]
    fn test_class_parse() -> KapiResult<()> {
        let mut class_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        class_path.push("compiled_source/Main.class");

        read_class(class_path)?;

        Ok(())
    }
}
