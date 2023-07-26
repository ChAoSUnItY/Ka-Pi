use std::io::Read;

use crate::node::class::Class;
use crate::parse::error::{ParseError, ParseResult};

pub(crate) mod attribute;
pub(crate) mod class;
pub(crate) mod constant;
pub mod error;
pub(crate) mod field;
pub(crate) mod method;
pub(crate) mod signature;

pub fn to_class<R: Read>(class_bytes: &mut R) -> ParseResult<Class> {
    let class = class::class(class_bytes)?;

    let mut remain = vec![];

    class_bytes.read_to_end(&mut remain)?;

    if !remain.is_empty() {
        Err(ParseError::Remains(remain.len()))
    } else {
        Ok(class)
    }
}

pub use signature::{class_signature, field_signature, method_signature};
