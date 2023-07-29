use crate::parse::ParseError;
use thiserror::Error;

pub type NodeResResult<T> = Result<T, NodeResError>;

#[derive(Debug, Error)]
pub enum NodeResError {
    #[error(transparent)]
    ParseFail(#[from] ParseError),
    #[error("failed to parse bytes to string in constant Utf8, given bytes: {0:?}")]
    StringParseFail(Box<[u8]>),
    #[error("unknown constant reference #{0}")]
    UnknownConstantReference(u16),
    #[error("attempt to match {0} in {1:?} but got {2}")]
    MatchOutOfBound(&'static str, Vec<&'static str>, usize),
    #[error("expected referenced constant {0} at #{1} but got {2}")]
    MismatchReferenceConstant(&'static str, u16, &'static str),
    #[error("character {0} cannot be converted into wildcard")]
    InvalidWildcard(char),
    #[error("character {0} cannot be converted into base type")]
    InvalidBaseType(char),
}
