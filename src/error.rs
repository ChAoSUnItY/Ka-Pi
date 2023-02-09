use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RasmError {
    StateError(&'static str),
    Utf8Error(&'static str),
    TypeError(&'static str),
    ArgError(&'static str),
}

impl Display for RasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RasmError::StateError(cause) => write!(f, "{}", cause),
            RasmError::Utf8Error(cause) => write!(f, "{}", cause),
            RasmError::TypeError(cause) => write!(f, "{}", cause),
            RasmError::ArgError(cause) => write!(f, "{}", cause),
        }
    }
}

impl Error for RasmError {}
