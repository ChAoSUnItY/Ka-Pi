use serde::{Deserialize, Serialize};

use crate::asm::opcodes::RefKind;

use super::opcodes;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handle {
    tag: RefKind,
    owner: String,
    name: String,
    descriptor: String,
}

impl Handle {
    pub const fn new(tag: RefKind, owner: String, name: String, descriptor: String) -> Self {
        Self {
            tag,
            owner,
            name,
            descriptor,
        }
    }
}
