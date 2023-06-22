use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::asm::node::access_flag::{ExportsAccessFlag, OpensAccessFlag, RequiresAccessFlag};
use crate::asm::node::constant::{Constant, ConstantPool, Module, Package, Utf8};
use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Requires {
    pub requires_index: u16,
    pub requires_flags: Vec<RequiresAccessFlag>,
    pub requires_version_index: u16,
}

impl Requires {
    pub fn requires<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        if let Some(Constant::Module(module)) = constant_pool.get(self.requires_index) {
            Some(module)
        } else {
            None
        }
    }

    pub fn requires_version<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        if let Some(Constant::Utf8(utf8)) = constant_pool.get(self.requires_version_index) {
            Some(utf8)
        } else {
            None
        }
    }
}

impl ConstantRearrangeable for Requires {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.requires_index, rearrangements);
        Self::rearrange_index(&mut self.requires_version_index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Exports {
    pub exports_index: u16,
    pub exports_flags: Vec<ExportsAccessFlag>,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>,
}

impl Exports {
    pub fn exports<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Package> {
        if let Some(Constant::Package(package)) = constant_pool.get(self.exports_index) {
            Some(package)
        } else {
            None
        }
    }

    pub fn exports_to<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        if let Some(Constant::Module(module)) = self
            .exports_to_index
            .get(index)
            .map(|exports_to_index| constant_pool.get(*exports_to_index))
            .flatten()
        {
            Some(module)
        } else {
            None
        }
    }
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Opens {
    pub opens_index: u16,
    pub opens_flags: Vec<OpensAccessFlag>,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

impl Opens {
    pub fn opens<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Package> {
        if let Some(Constant::Package(package)) = constant_pool.get(self.opens_index) {
            Some(package)
        } else {
            None
        }
    }

    pub fn opens_to<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        if let Some(Constant::Module(module)) = self
            .opens_to_index
            .get(index)
            .map(|opens_to_index| constant_pool.get(*opens_to_index))
            .flatten()
        {
            Some(module)
        } else {
            None
        }
    }
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Provides {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}

impl Provides {
    pub fn provides<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Package> {
        if let Some(Constant::Package(package)) = constant_pool.get(self.provides_index) {
            Some(package)
        } else {
            None
        }
    }

    pub fn provides_to<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        if let Some(Constant::Module(module)) = self
            .provides_with_index
            .get(index)
            .map(|provides_with_index| constant_pool.get(*provides_with_index))
            .flatten()
        {
            Some(module)
        } else {
            None
        }
    }
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
