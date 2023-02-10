use std::{
    error::Error,
    fmt::{Debug, Display},
};

use jni::errors::Error as JniErr;

use crate::error::KapiError::JNIError;

pub(crate) trait IntoKapiResult<T> {
    fn as_kapi(&self) -> KapiResult<T>;
}

pub type KapiResult<T> = Result<T, KapiError>;

impl<T> IntoKapiResult<T> for jni::errors::Result<T> {
    fn as_kapi(&self) -> KapiResult<T> {
        self.map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KapiError {
    StateError(String),
    Utf8Error(String),
    TypeError(String),
    ArgError(String),
    ClassResolveError(String),
    JNIError(String),
}

impl Display for KapiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KapiError::StateError(cause) => write!(f, "{}", cause),
            KapiError::Utf8Error(cause) => write!(f, "{}", cause),
            KapiError::TypeError(cause) => write!(f, "{}", cause),
            KapiError::ArgError(cause) => write!(f, "{}", cause),
            KapiError::ClassResolveError(cause) => write!(f, "{}", cause),
            JNIError(cause) => write!(f, "{}", cause),
        }
    }
}

impl Error for KapiError {}

impl From<JniErr> for KapiError {
    fn from(value: JniErr) -> Self {
        JNIError(value.to_string())
    }
}
