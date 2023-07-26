use std::io::Read;

pub use error::{ParseError, ParseResult};
pub use signature::{class_signature, field_signature, method_signature};

use crate::node::class::Class;

pub(crate) mod attribute;
pub(crate) mod class;
pub(crate) mod constant;
pub(crate) mod error;
pub(crate) mod field;
pub(crate) mod method;
pub(crate) mod signature;

/// Parses class file based on given options.
pub fn to_class<R: Read>(class_bytes: &mut R, option: ParsingOption) -> ParseResult<Class> {
    let class = class::class(class_bytes, option)?;

    let mut remain = vec![];

    class_bytes.read_to_end(&mut remain)?;

    if !remain.is_empty() {
        Err(ParseError::Remains(remain.len()))
    } else {
        Ok(class)
    }
}

/// Parsing options allows marking some parsing phase optional.
#[derive(Debug, Default)]
pub struct ParsingOption {
    parse_attribute: bool,
}

impl ParsingOption {
    /// Skips on attribute struct parsing.
    pub fn parse_attribute(mut self) -> Self {
        self.parse_attribute = true;

        self
    }
}
