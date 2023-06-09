use crate::asm::node::ConstantRearrangeable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ldc {
    pub index: u8,
}

impl ConstantRearrangeable for Ldc {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut (self.index as u16), rearrangements);
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc_w {
    pub index: u16,
}

impl ConstantRearrangeable for Ldc_w {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.index, rearrangements);
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc2_w {
    pub index: u16,
}

impl ConstantRearrangeable for Ldc2_w {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) {
        Self::rearrange_index(&mut self.index, rearrangements);
    }
}
