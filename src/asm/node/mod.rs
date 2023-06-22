//! `node` module contains all specification-described data structures from
//! [The JVM Specification - Java SE 20 Edition](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf).
//!
//! Some of the structures are made to be user-friendly, at the same time makes it much more
//! straightforward to use.

use crate::error::{KapiError, KapiResult};
use std::collections::HashMap;

pub mod access_flag;
pub mod attribute;
pub mod class;
pub mod constant;
pub mod field;
pub mod method;
pub mod opcode;
// TODO: Move visitors to this module
#[cfg(feature = "generate")]
pub mod signature;

/// This trait indicates the implemented supertype's field can be rearrange based on given map of
/// rearrangement set.
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
    /// This will rearrange an original u8 index, see wider rearrangement function [Self::rearrange_index].
    ///
    /// Returns an [Err] if the target index of the given rearrangement entry is larger than [u8::MAX],
    /// which is impossible to rearrange to.
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

    /// Rearranges the implemented supertype's field based given map of rearrangement set.
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()>;
}
