use std::ops::RangeFrom;

use nom::error::ParseError;
use nom::{IResult, InputIter, InputLength, Slice};

mod attribute;
pub mod class;
mod constant;
mod field;
mod method;

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
