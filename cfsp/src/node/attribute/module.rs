use serde::{Deserialize, Serialize};

use crate::node::access_flag::{ExportsAccessFlag, OpensAccessFlag, RequiresAccessFlag};
use crate::node::constant::{ConstantPool, Module, Package, Utf8};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Requires {
    pub requires_index: u16,
    pub requires_flags: RequiresAccessFlag,
    pub requires_version_index: u16,
}

impl Requires {
    pub fn requires<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        constant_pool.get_module(self.requires_index)
    }

    pub fn requires_version<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Utf8> {
        constant_pool.get_utf8(self.requires_version_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Exports {
    pub exports_index: u16,
    pub exports_flags: ExportsAccessFlag,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>,
}

impl Exports {
    pub fn exports<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Package> {
        constant_pool.get_package(self.exports_index)
    }

    pub fn exports_to<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        self.exports_to_index
            .get(index)
            .and_then(|exports_to_index| constant_pool.get_module(*exports_to_index))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Opens {
    pub opens_index: u16,
    pub opens_flags: OpensAccessFlag,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

impl Opens {
    pub fn opens<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Package> {
        constant_pool.get_package(self.opens_index)
    }

    pub fn opens_to<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        self.opens_to_index
            .get(index)
            .and_then(|opens_to_index| constant_pool.get_module(*opens_to_index))
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
        constant_pool.get_package(self.provides_index)
    }

    pub fn provides_to<'attribute, 'constant_pool: 'attribute>(
        &'attribute self,
        index: usize,
        constant_pool: &'constant_pool ConstantPool,
    ) -> Option<&'constant_pool Module> {
        self.provides_with_index
            .get(index)
            .and_then(|provides_with_index| constant_pool.get_module(*provides_with_index))
    }
}
