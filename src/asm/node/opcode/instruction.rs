use crate::asm::node::ConstantRearrangeable;
use crate::error::KapiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ldc {
    pub index: u8,
}

impl ConstantRearrangeable for Ldc {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_narrow_index(&mut self.index, rearrangements)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc_w {
    pub index: u16,
}

impl ConstantRearrangeable for Ldc_w {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Ldc2_w {
    pub index: u16,
}

impl ConstantRearrangeable for Ldc2_w {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetStatic {
    pub index: u16,
}

impl ConstantRearrangeable for GetStatic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PutStatic {
    pub index: u16,
}

impl ConstantRearrangeable for PutStatic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetField {
    pub index: u16,
}

impl ConstantRearrangeable for GetField {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PutField {
    pub index: u16,
}

impl ConstantRearrangeable for PutField {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeVirtual {
    pub index: u16,
}

impl ConstantRearrangeable for InvokeVirtual {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeSpecial {
    pub index: u16,
}

impl ConstantRearrangeable for InvokeSpecial {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeStatic {
    pub index: u16,
}

impl ConstantRearrangeable for InvokeStatic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeInterface {
    pub index: u16,
    pub count: u8,
}

impl ConstantRearrangeable for InvokeInterface {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InvokeDynamic {
    pub index: u16,
}

impl ConstantRearrangeable for InvokeDynamic {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct New {
    pub index: u16,
}

impl ConstantRearrangeable for New {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ANewArray {
    pub index: u16,
}

impl ConstantRearrangeable for ANewArray {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CheckCast {
    pub index: u16,
}

impl ConstantRearrangeable for CheckCast {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InstanceOf {
    pub index: u16,
}

impl ConstantRearrangeable for InstanceOf {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Wide {
    Load(WideLoad),
    Inc(u16, i16),
}

//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum WideLoad {
    ILOAD(u16),
    FLOAD(u16),
    ALOAD(u16),
    LLOAD(u16),
    DLOAD(u16),
    ISTORE(u16),
    FSTORE(u16),
    ASTORE(u16),
    LSTORE(u16),
    DSTORE(u16),
    RET(u16),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MultiANewArray {
    pub index: u16,
    pub dimensions: u8,
}

impl ConstantRearrangeable for MultiANewArray {
    fn rearrange(&mut self, rearrangements: &HashMap<u16, u16>) -> KapiResult<()> {
        Self::rearrange_index(&mut self.index, rearrangements);

        Ok(())
    }
}
