use std::ops::BitOr;

use itertools::Itertools;
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

pub trait AccessFlag
where
    Self: Into<u16> + Copy + IntoEnumIterator,
{
    fn mask_access_flags(bytes: u16) -> Vec<Self> {
        Self::iter()
            .filter(|&access_flag| access_flag.into() & bytes >= 1)
            .collect_vec()
    }
}

pub trait AccessFlags<'a, T>
where
    T: AccessFlag + 'a,
    Self: IntoIterator<Item = &'a T> + Sized,
{
    fn fold_flags(self) -> u16 {
        self.into_iter()
            .map(|flag| (*flag).into())
            .fold(0, u16::bitor)
    }
}

impl<'a, T> AccessFlags<'a, T> for &'a [T] where T: AccessFlag {}
impl<'a, T> AccessFlags<'a, T> for &'a Vec<T> where T: AccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum ClassAccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}

impl AccessFlag for ClassAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum FieldAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

impl AccessFlag for FieldAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

impl AccessFlag for MethodAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum NestedClassAccessFlag {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

impl AccessFlag for NestedClassAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum ParameterAccessFlag {
    Final = 0x0010,
    Synthetic = 0x1000,
}

impl AccessFlag for ParameterAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum ModuleAccessFlag {
    Open = 0x0020,
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

impl AccessFlag for ModuleAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum RequiresAccessFlag {
    Transitive = 0x0020,
    StaticPhase = 0x0040,
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

impl AccessFlag for RequiresAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum ExportsAccessFlag {
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

impl AccessFlag for ExportsAccessFlag {}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum OpensAccessFlag {
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

impl AccessFlag for OpensAccessFlag {}
