use std::ops::{Range, RangeFrom, RangeTo};
use std::str::FromStr;

use nom::error::{ErrorKind, ParseError};
use nom::{
    AsBytes, Compare, CompareResult, Err, ExtendInto, FindSubstring, FindToken, IResult, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Needed, Offset, ParseTo, Slice,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BytesSpan<'fragment> {
    pub offset: usize,
    pub fragment: &'fragment [u8],
}

impl<'fragment> BytesSpan<'fragment> {
    pub fn new<F>(fragment: F) -> Self
    where
        F: Into<&'fragment [u8]>,
    {
        Self {
            offset: 0,
            fragment: fragment.into(),
        }
    }
    
    pub fn len(&self) -> usize {
        self.input_len()
    }
}

impl AsBytes for BytesSpan<'_> {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        &self.fragment
    }
}

impl<T> Compare<T> for BytesSpan<'_>
where
    T: AsBytes,
{
    #[inline]
    fn compare(&self, t: T) -> CompareResult {
        self.fragment.compare(t.as_bytes())
    }

    #[inline]
    fn compare_no_case(&self, t: T) -> CompareResult {
        self.fragment.compare_no_case(t.as_bytes())
    }
}

impl<'fragment> ExtendInto for BytesSpan<'fragment> {
    type Item = <&'fragment [u8] as ExtendInto>::Item;
    type Extender = <&'fragment [u8] as ExtendInto>::Extender;

    fn new_builder(&self) -> Self::Extender {
        self.fragment.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.fragment.extend_into(acc)
    }
}

impl<'b> FindSubstring<&'b [u8]> for BytesSpan<'_> {
    #[inline]
    fn find_substring(&self, substr: &'b [u8]) -> Option<usize> {
        self.fragment.find_substring(substr)
    }
}

impl<'b> FindSubstring<&'b str> for BytesSpan<'_> {
    #[inline]
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.fragment.find_substring(substr)
    }
}

impl FindToken<u8> for BytesSpan<'_> {
    #[inline]
    fn find_token(&self, token: u8) -> bool {
        self.fragment.find_token(token)
    }
}

impl<'b> FindToken<&'b u8> for BytesSpan<'_> {
    #[inline]
    fn find_token(&self, token: &'b u8) -> bool {
        self.fragment.find_token(token)
    }
}

impl<'fragment> InputIter for BytesSpan<'fragment> {
    type Item = <&'fragment [u8] as InputIter>::Item;
    type Iter = <&'fragment [u8] as InputIter>::Iter;
    type IterElem = <&'fragment [u8] as InputIter>::IterElem;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.fragment.iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.fragment.iter_elements()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.iter_elements().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.fragment.slice_index(count)
    }
}

impl InputLength for BytesSpan<'_> {
    #[inline]
    fn input_len(&self) -> usize {
        self.fragment.input_len()
    }
}

impl InputTake for BytesSpan<'_> {
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<'fragment> InputTakeAtPosition for BytesSpan<'fragment> {
    type Item = <&'fragment [u8] as InputTakeAtPosition>::Item;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.fragment.position(predicate) {
            Some(count) => Ok(self.take_split(count)),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.fragment.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(count) => Ok(self.take_split(count)),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.fragment.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(count) => Ok(self.take_split(count)),
            None => {
                if self.fragment.input_len() == 0 {
                    Err(Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl Offset for BytesSpan<'_> {
    fn offset(&self, second: &Self) -> usize {
        second.offset - self.offset
    }
}

impl<T> ParseTo<T> for BytesSpan<'_>
where
    T: FromStr,
{
    fn parse_to(&self) -> Option<T> {
        self.fragment.parse_to()
    }
}

impl Slice<Range<usize>> for BytesSpan<'_> {
    fn slice(&self, range: Range<usize>) -> Self {
        let next_fragment = self.fragment.slice(range);
        let head_len = self.fragment.offset(&next_fragment);
        if head_len == 0 {
            return Self {
                offset: self.offset,
                fragment: next_fragment,
            };
        }

        Self {
            offset: self.offset + head_len,
            fragment: next_fragment,
        }
    }
}

impl Slice<RangeFrom<usize>> for BytesSpan<'_> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let next_fragment = self.fragment.slice(range);
        let head_len = self.fragment.offset(&next_fragment);
        if head_len == 0 {
            return Self {
                offset: self.offset,
                fragment: next_fragment,
            };
        }

        Self {
            offset: self.offset + head_len,
            fragment: next_fragment,
        }
    }
}

impl Slice<RangeTo<usize>> for BytesSpan<'_> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let next_fragment = self.fragment.slice(range);
        let head_len = self.fragment.offset(&next_fragment);
        if head_len == 0 {
            return Self {
                offset: self.offset,
                fragment: next_fragment,
            };
        }

        Self {
            offset: self.offset + head_len,
            fragment: next_fragment,
        }
    }
}
