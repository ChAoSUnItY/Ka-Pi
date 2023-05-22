use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::asm::opcodes::{AccessFlag, ClassAccessFlag};

pub(crate) fn mask_access_flags<AC: AccessFlag<AC> + Into<u16> + Copy + IntoEnumIterator>(
    bytes: u16,
) -> Vec<AC> {
    AC::iter()
        .filter(|&access_flag| access_flag.into() & bytes >= 1)
        .collect_vec()
}
