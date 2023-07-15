use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::{IResult, InputLength};

use byte_span::BytesSpan;

#[test]
fn test_segments() {
    let bytes = b"foo bar foo";

    let result = split_by_spaces(BytesSpan::new(&bytes[..]));

    assert!(result.is_ok());

    let (remain, symbols) = result.unwrap();

    assert_eq!(remain.input_len(), 0);
    assert_eq!(
        symbols[0],
        BytesSpan {
            offset: 0,
            fragment: "foo".as_bytes()
        }
    );
    assert_eq!(
        symbols[1],
        BytesSpan {
            offset: 4,
            fragment: "bar".as_bytes()
        }
    );
    assert_eq!(
        symbols[2],
        BytesSpan {
            offset: 8,
            fragment: "foo".as_bytes()
        }
    );
}

fn split_by_spaces(input: BytesSpan<'_>) -> IResult<BytesSpan<'_>, Vec<BytesSpan<'_>>> {
    separated_list0(tag(" "), alt((tag("foo"), tag("bar"))))(input)
}
