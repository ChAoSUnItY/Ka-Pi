use byte_span::BytesSpan;
use nom::error::{ErrorKind, ParseError};
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::error::KapiError::*;

pub(crate) trait IntoKapiResult<T> {
    fn into_kapi(self) -> KapiResult<T>;
}

pub type KapiResult<T> = Result<T, KapiError>;

impl<T, E> IntoKapiResult<T> for Result<T, E>
where
    E: Into<KapiError>,
{
    fn into_kapi(self) -> KapiResult<T> {
        self.map_err(Into::into)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KapiError {
    StateError(String),
    Utf8Error(&'static str),
    TypeError(&'static str),
    ArgError(String),
    ClassResolveError(&'static str),
    ClassParseError(String),
}

impl Display for KapiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError(cause) => write!(f, "{}", cause),
            Utf8Error(cause) => write!(f, "{}", cause),
            TypeError(cause) => write!(f, "{}", cause),
            ArgError(cause) => write!(f, "{}", cause),
            ClassResolveError(cause) => write!(f, "{}", cause),
            ClassParseError(cause) => write!(f, "{}", cause),
        }
    }
}

impl Error for KapiError {}
