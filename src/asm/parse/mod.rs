use std::fs::File;
use std::io::Read;
use std::ops::RangeFrom;
use std::path::Path;

use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::be_u16;
use nom::{IResult, InputIter, InputLength, Slice};

use crate::asm::node::access_flag::AccessFlag;
use crate::asm::node::class::Class;
use crate::asm::node::constant::ConstantPool;
use crate::error::{KapiError, KapiResult};

pub mod attribute;
pub mod class;
pub mod constant;
pub mod field;
pub mod method;

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

    to_class(&class_bytes)
}

pub fn to_class(class_bytes: &[u8]) -> KapiResult<Class> {
    match class::class(&class_bytes[..]) {
        Ok((remain, class)) => {
            if !remain.is_empty() {
                Err(KapiError::ClassParseError(format!("Unable to parse class bytes, reason: class is fully parsed but there are {} bytes left, {remain:?}", remain.len())))
            } else {
                Ok(class)
            }
        }
        Err(err) => Err(KapiError::ClassParseError(format!(
            "Unable parse class bytes, reason: {err}"
        ))),
    }
}

fn collect<I, LP, TP, L, T, E: ParseError<I>>(
    mut len_parser: LP,
    mut item_parser: TP,
) -> impl FnMut(I) -> IResult<I, (L, Vec<T>), E>
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
    LP: FnMut(I) -> IResult<I, L, E>,
    TP: FnMut(I) -> IResult<I, T, E>,
    L: Into<u64> + Copy,
{
    move |input: I| {
        let (mut input, len) = len_parser(input)?;
        let length = len.into();
        let mut items = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let (remain, item) = item_parser(input)?;

            items.push(item);
            input = remain;
        }

        Ok((input, (len, items)))
    }
}

fn collect_with_constant_pool<'parser, 'constant_pool: 'parser, I, LP, TP, L, T, E: ParseError<I>>(
    mut len_parser: LP,
    mut item_parser: TP,
    constant_pool: &'constant_pool ConstantPool,
) -> impl FnMut(I) -> IResult<I, (L, Vec<T>), E> + '_
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
    LP: FnMut(I) -> IResult<I, L, E> + 'parser + 'constant_pool,
    TP: FnMut(I, &'constant_pool ConstantPool) -> IResult<I, T, E> + 'parser + 'constant_pool,
    L: Into<u64> + Copy,
{
    move |input: I| {
        let (mut input, len) = len_parser(input)?;
        let length = len.into();
        let mut items = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let (remain, item) = item_parser(input, constant_pool)?;

            items.push(item);
            input = remain;
        }

        Ok((input, (len, items)))
    }
}

fn access_flag<F>(input: &[u8]) -> IResult<&[u8], Vec<F>>
where
    F: AccessFlag,
{
    map(be_u16, F::mask_access_flags)(input)
}
