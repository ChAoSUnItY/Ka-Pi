use std::ops::BitOr;

use itertools::Itertools;
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

/// This trait marks implemented supertype (specifically an enum) as an access flag type.
pub trait AccessFlag
where
    Self: Into<u16> + Copy + IntoEnumIterator,
{
    /// Extract the given bytes into vector of access flags.
    fn extract_flags(bytes: u16) -> Vec<Self> {
        Self::iter()
            .filter(|&access_flag| {
                let access_flag = access_flag.into();

                bytes & access_flag == access_flag
            })
            .collect_vec()
    }
}

/// This trait requires implemented supertype to be an iterable access flag (commonly [std::slice] and [Vec]),
/// which makes vector of access flags able to convert into u16.
pub trait AccessFlags<'a, T>
where
    T: AccessFlag + 'a,
    Self: IntoIterator<Item = &'a T> + Sized,
{
    /// Folds access flags into u16.
    fn fold_flags(self) -> u16 {
        self.into_iter()
            .map(|flag| (*flag).into())
            .fold(0, u16::bitor)
    }
}

impl<'a, T> AccessFlags<'a, T> for &'a [T] where T: AccessFlag {}
impl<'a, T> AccessFlags<'a, T> for &'a Vec<T> where T: AccessFlag {}

/// Access flag for [node::class::Class].
///
/// See [Table 4.1-B](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=85).
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

/// Access flag for [node::field::Field].
///
/// See [Table 4.5-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=110).
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

/// Access flag for [node::method::Method].
///
/// See [Table 4.6-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=112).
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

/// Access flag for [node::attribute::InnerClass].
///
/// See [Table 4.7.6-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=138).
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

/// Access flag for [node::attribute::MethodParameter].
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=183).
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
    Mandated = 0x8000,
}

impl AccessFlag for ParameterAccessFlag {}

/// Access flag for [node::attribute::Module].
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=186).
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

/// Access flag for [node::attribute::module::Requires].
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=187).
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

/// Access flag for [node::attribute::module::Exports].
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=188).
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

/// Access flag for [node::attribute::module::Opens].
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=189).
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
