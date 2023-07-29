use std::io;

use thiserror::Error;

use crate::node::opcode::Opcode;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("expected [0xCA, 0xFE, 0xBA, 0xBE] at the header of class file but got {0:?}")]
    MismatchedMagicNumber([u8; 4]),
    #[error("attempt to match {0} in {1:?} but got {2}")]
    MatchOutOfBoundUsize(&'static str, Vec<&'static str>, usize),
    #[error("attempt to match {0} in {1:?} but got '{2}'")]
    MatchOutOfBoundChar(&'static str, Vec<char>, char),
    #[error("attempt to match opcode but got opcode value {0}")]
    MatchOutOfBoundOpcode(u8),
    #[error("attempt to match sub opcode in wide opcode but got {0}, while {1:?} is allowed as sub opcode of wide opcode")]
    MatchOutOfBoundWideOpcode(u8, Vec<Opcode>),
    #[error("the parse for segment is finished but still remains {0} bytes")]
    Remains(usize),
    #[error("expected {1:?} but got '{0}'")]
    MismatchedCharacter(char, Vec<char>),
    #[error("attempt to parse {0} but is incomplete")]
    OutOfBound(&'static str),
}
