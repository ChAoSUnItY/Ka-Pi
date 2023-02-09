use std::{error::Error, fmt::{Display, Debug}};

#[derive(Debug, Clone)]
pub enum KapiError {
    StateError(String),
    Utf8Error(String),
    TypeError(String),
    ArgError(String),
    ClassResolveError(String),
}

impl Display for KapiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KapiError::StateError(cause) => write!(f, "{}", cause),
            KapiError::Utf8Error(cause) => write!(f, "{}", cause),
            KapiError::TypeError(cause) => write!(f, "{}", cause),
            KapiError::ArgError(cause) => write!(f, "{}", cause),
            KapiError::ClassResolveError(cause) => write!(f, "{}", cause),
        }
    }
}

impl Error for KapiError {}
