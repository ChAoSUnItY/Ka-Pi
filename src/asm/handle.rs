use super::opcodes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle {
    tag: u8,
    owner: String,
    name: String,
    descriptor: String,
    is_interface: bool,
}

impl Handle {
    pub const fn new(tag: u8, owner: String, name: String, descriptor: String) -> Self {
        Self {
            tag,
            owner,
            name,
            descriptor,
            is_interface: tag == opcodes::H_INVOKEINTERFACE,
        }
    }
}
