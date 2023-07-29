use serde::{Deserialize, Serialize};

use crate::node::access_flag::ClassAccessFlag;
use crate::node::attribute::AttributeInfo;
use crate::node::constant::ConstantPool;
use crate::node::field::Field;
use crate::node::method::Method;

/// Represents a class file.
///
/// See [4.1 The ClassFile Structure](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=82).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Class {
    pub java_version: JavaVersion,
    pub constant_pool_count: u16,
    pub constant_pool: ConstantPool,
    pub access_flag: ClassAccessFlag,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<Field>,
    pub methods_count: u16,
    pub methods: Vec<Method>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl Class {
    /// Get current class from constant pool.
    pub fn this_class(&self) -> Option<&crate::node::constant::Class> {
        self.constant_pool.get_class(self.this_class)
    }

    /// Get super class from constant pool.
    pub fn super_class(&self) -> Option<&crate::node::constant::Class> {
        self.constant_pool.get_class(self.super_class)
    }

    /// Get interface from constant pool at given index.
    pub fn interface(&self, index: u16) -> Option<&crate::node::constant::Class> {
        self.interfaces
            .get(index as usize)
            .and_then(|interface_index| self.constant_pool.get_class(*interface_index))
    }
}

/// Represents java version documented in specification (combines `major_version` and `minor_version`).
///
/// See [Table 4.1-A](https://docs.oracle.com/javase/specs/jvms/se20/jvms20.pdf#page=83).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum JavaVersion {
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
    V1_7,
    V1_8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    Custom { minor: u16, major: u16 },
}

impl JavaVersion {
    #[rustfmt::skip]
    pub const KNOWN_VERSIONS: [JavaVersion; 21] = [
        Self::V1_1, Self::V1_2, Self::V1_3, Self::V1_4, Self::V1_5, Self::V1_6, Self::V1_7, Self::V1_8,
        Self::V9, Self::V10, Self::V11, Self::V12, Self::V13, Self::V14, Self::V15, Self::V16, Self::V17,
        Self::V18, Self::V19, Self::V20, Self::V21,
    ];

    pub fn version(&self) -> u32 {
        match self {
            Self::V1_1 => 3 << 16 | 45,
            Self::V1_2 => 0 << 16 | 46,
            Self::V1_3 => 0 << 16 | 47,
            Self::V1_4 => 0 << 16 | 48,
            Self::V1_5 => 0 << 16 | 49,
            Self::V1_6 => 0 << 16 | 50,
            Self::V1_7 => 0 << 16 | 51,
            Self::V1_8 => 0 << 16 | 52,
            Self::V9 => 0 << 16 | 53,
            Self::V10 => 0 << 16 | 54,
            Self::V11 => 0 << 16 | 55,
            Self::V12 => 0 << 16 | 56,
            Self::V13 => 0 << 16 | 57,
            Self::V14 => 0 << 16 | 58,
            Self::V15 => 0 << 16 | 59,
            Self::V16 => 0 << 16 | 60,
            Self::V17 => 0 << 16 | 61,
            Self::V18 => 0 << 16 | 62,
            Self::V19 => 0 << 16 | 63,
            Self::V20 => 0 << 16 | 64,
            Self::V21 => 0 << 16 | 65,
            JavaVersion::Custom { minor, major } => {
                let minor = minor.to_be_bytes();
                let major = major.to_be_bytes();

                u32::from_be_bytes([minor[0], minor[1], major[0], major[1]])
            }
        }
    }
}

impl From<u32> for JavaVersion {
    fn from(value: u32) -> Self {
        for version in Self::KNOWN_VERSIONS {
            if value == version.version() {
                return version;
            }
        }

        let bits = value.to_be_bytes();

        JavaVersion::Custom {
            minor: u16::from_be_bytes([bits[0], bits[1]]),
            major: u16::from_be_bytes([bits[2], bits[3]]),
        }
    }
}
