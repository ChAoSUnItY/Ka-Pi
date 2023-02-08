use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RasmError {
    StateError{ cause: &'static str },
    Utf8Error{ cause: &'static str }
}

impl Display for RasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RasmError::StateError { cause } => write!(f, "{}", cause),
            RasmError::Utf8Error { cause } => write!(f, "{}", cause),
        }
    }
}

impl Error for RasmError {
}
