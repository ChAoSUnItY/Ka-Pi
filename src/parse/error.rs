use std::io;

use thiserror::Error;

use crate::node::opcode::Opcode;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("internal io error")]
    Io(#[from] io::Error),
    #[error("expected [0xCA, 0xFE, 0xBA, 0xBE] at the header of class file but got {0:?}")]
    MismatchedMagicNumber([u8; 4]),
    #[error("attempt to match {0} in {1:?} but got {2}")]
    MatchOutOfBoundUsize(&'static str, Vec<&'static str>, usize),
    #[error("attempt to match {0} in {1:?} but got '{2}'")]
    MatchOutOfBoundChar(&'static str, Vec<char>, char),
    #[error("attempt to match {0} in {1:?} but got opcode {2}{}", if let Some(opcode) = .3 { format!(" (Which is {:?})", opcode)} else { String::new() })]
    MatchOutOfBoundOpcode(&'static str, Vec<Opcode>, u8, Option<Opcode>),
    #[error("the parse for segment is finished but still remains {0} bytes")]
    Remains(usize),
    #[error("expected {1:?} but got '{0}'")]
    MismatchedCharacter(char, Vec<char>),
    #[error("attempt to parse {0} but is incomplete")]
    OutOfBound(&'static str),
}
