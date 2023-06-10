use crate::error::{KapiError, KapiResult};
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
    ///
    /// This will rearrange an u16 index.
    fn rearrange_index(original: &mut u16, rearrangements: &HashMap<u16, u16>) {
        if let Some(target_index) = rearrangements.get(original) {
            *original = *target_index;
        }
    }

    /// Rearranges indices into new indices according to parameter `rearrangements`.
    ///
    /// This will rearrange an original u8 index, see wider rearrangement function [Self::rearrange_index]
    fn rearrange_narrow_index(
        original: &mut u8,
        rearrangements: &HashMap<u16, u16>,
    ) -> KapiResult<()> {
        if let Some(target_index) = rearrangements.get(&(*original as u16)) {
            if *target_index > u8::MAX as u16 {
                // Invalid rearrangement
                return Err(KapiError::StateError(format!(
                    "Unable to rearrange from narrow index {original} to wide index {target_index}"
                )));
            }

            *original = *target_index as u8;
        }

        Ok(())
    }

    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()>;
}
