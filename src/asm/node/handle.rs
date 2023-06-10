use crate::asm::node::opcode::RefKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
