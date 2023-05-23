use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::asm::node::access_flag::ClassAccessFlag;
use crate::asm::node::constant::ConstantPool;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    pub java_version: JavaVersion,
    pub constant_pool: ConstantPool,
    pub access_flags: Vec<ClassAccessFlag>,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
}

#[repr(u32)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum JavaVersion {
    V1_1 = 3 << 16 | 45,
    V1_2 = 0 << 16 | 46,
    V1_3 = 0 << 16 | 47,
    V1_4 = 0 << 16 | 48,
    V1_5 = 0 << 16 | 49,
    V1_6 = 0 << 16 | 50,
    V1_7 = 0 << 16 | 51,
    V1_8 = 0 << 16 | 52,
    V9 = 0 << 16 | 53,
    V10 = 0 << 16 | 54,
    V11 = 0 << 16 | 55,
    V12 = 0 << 16 | 56,
    V13 = 0 << 16 | 57,
    V14 = 0 << 16 | 58,
    V15 = 0 << 16 | 59,
    V16 = 0 << 16 | 60,
    V17 = 0 << 16 | 61,
    V18 = 0 << 16 | 62,
    V19 = 0 << 16 | 63,
    V20 = 0 << 16 | 64,
    V21 = 0 << 16 | 65,
}
