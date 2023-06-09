use crate::asm::node::access_flag::{ExportsAccessFlag, OpensAccessFlag, RequiresAccessFlag};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Requires {
    pub requires_index: u16,
    pub requires_flags: Vec<RequiresAccessFlag>,
    pub requires_version_index: u16,
}

impl ConstantRearrangeable for Requires {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.requires_index, rearrangements);
        Self::rearrange_index(&mut self.requires_version_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Exports {
    pub exports_index: u16,
    pub exports_flags: Vec<ExportsAccessFlag>,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>,
}

impl ConstantRearrangeable for Exports {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.exports_index, rearrangements);

        for exports_to in &mut self.exports_to_index {
            Self::rearrange_index(exports_to, rearrangements);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Opens {
    pub opens_index: u16,
    pub opens_flags: Vec<OpensAccessFlag>,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

impl ConstantRearrangeable for Opens {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.opens_index, rearrangements);

        for opens_to in &mut self.opens_to_index {
            Self::rearrange_index(opens_to, rearrangements);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Provides {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}

impl ConstantRearrangeable for Provides {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.provides_index, rearrangements);

        for provides_to in &mut self.provides_with_index {
            Self::rearrange_index(provides_to, rearrangements);
        }

        Ok(())
    }
}
