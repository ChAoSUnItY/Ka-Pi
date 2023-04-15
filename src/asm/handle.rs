use serde::{Deserialize, Serialize};

use crate::asm::opcodes::RefKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handle {
    pub tag: RefKind,
    pub owner: String,
    pub name: String,
    pub descriptor: String,
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
