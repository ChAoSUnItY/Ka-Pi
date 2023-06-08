use std::collections::HashMap;

pub mod access_flag;
pub mod attribute;
pub mod class;
pub mod constant;
pub mod field;
pub mod handle;
pub mod method;
pub mod opcode;
pub mod signature;

pub(crate) trait ConstantRearrangeable {
    /// Rearranges indices into new indices according to parameter `rearrangements`.
    fn rearrange_index(original: &mut u16, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(original) {
            *original = *target_index;
        }
    }

    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>);
}
