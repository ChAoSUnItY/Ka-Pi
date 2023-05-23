use nom::bytes::complete::take;
use nom::combinator::{map, map_res};
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::IResult;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum ConstantTag {
    /** The tag value of CONSTANT_Class_info JVMS structures. */
    Class = 7,
    /** The tag value of CONSTANT_Fieldref_info JVMS structures. */
    FieldRef = 9,
    /** The tag value of CONSTANT_Methodref_info JVMS structures. */
    MethodRef = 10,
    /** The tag value of CONSTANT_InterfaceMethodref_info JVMS structures. */
    InterfaceMethodRef = 11,
    /** The tag value of CONSTANT_String_info JVMS structures. */
    String = 8,
    /** The tag value of CONSTANT_Integer_info JVMS structures. */
    Integer = 3,
    /** The tag value of CONSTANT_Float_info JVMS structures. */
    Float = 4,
    /** The tag value of CONSTANT_Long_info JVMS structures. */
    Long = 5,
    /** The tag value of CONSTANT_Double_info JVMS structures. */
    Double = 6,
    /** The tag value of CONSTANT_NameAndType_info JVMS structures. */
    NameAndType = 12,
    /** The tag value of CONSTANT_Utf8_info JVMS structures. */
    Utf8 = 1,
    /** The tag value of CONSTANT_MethodHandle_info JVMS structures. */
    MethodHandle = 15,
    /** The tag value of CONSTANT_MethodType_info JVMS structures. */
    MethodType = 16,
    /** The tag value of CONSTANT_MethodType_info JVMS structures. */
    Dynamic = 17,
    /** The tag value of CONSTANT_Dynamic_info JVMS structures. */
    InvokeDynamic = 18,
    /** The tag value of CONSTANT_InvokeDynamic_info JVMS structures. */
    Module = 19,
    /** The tag value of CONSTANT_Module_info JVMS structures. */
    Package = 20,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        bytes: [u8; 4],
    },
    Float {
        bytes: [u8; 4],
    },
    Long {
        high_bytes: [u8; 4],
        low_bytes: [u8; 4],
    },
    Double {
        high_bytes: [u8; 4],
        low_bytes: [u8; 4],
    },
    NameAndType {
        name_index: u16,
        type_index: u16,
    },
    Utf8 {
        /*  Implementation note: This has been merged into a single String type for later table
         *  implementation usage.
         */
        data: String,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

impl Constant {
    pub const fn tag(&self) -> ConstantTag {
        match self {
            Constant::Class { .. } => ConstantTag::Class,
            Constant::FieldRef { .. } => ConstantTag::FieldRef,
            Constant::MethodRef { .. } => ConstantTag::MethodRef,
            Constant::InterfaceMethodRef { .. } => ConstantTag::InterfaceMethodRef,
            Constant::String { .. } => ConstantTag::String,
            Constant::Integer { .. } => ConstantTag::Integer,
            Constant::Float { .. } => ConstantTag::Float,
            Constant::Long { .. } => ConstantTag::Long,
            Constant::Double { .. } => ConstantTag::Double,
            Constant::NameAndType { .. } => ConstantTag::NameAndType,
            Constant::Utf8 { .. } => ConstantTag::Utf8,
            Constant::MethodHandle { .. } => ConstantTag::MethodHandle,
            Constant::MethodType { .. } => ConstantTag::MethodType,
            Constant::Dynamic { .. } => ConstantTag::Dynamic,
            Constant::InvokeDynamic { .. } => ConstantTag::InvokeDynamic,
            Constant::Module { .. } => ConstantTag::Module,
            Constant::Package { .. } => ConstantTag::Package,
        }
    }
}

pub(crate) fn constant_pool(input: &[u8]) -> IResult<&[u8], Vec<Constant>> {
    let (mut input, len) = be_u16(input)?;
    let mut constants = Vec::with_capacity(len as usize);

    for _ in 0..len - 1 {
        let (remain, constant) = constant(input)?;

        constants.push(constant);
        input = remain;
    }

    Ok((input, constants))
}

fn constant(input: &[u8]) -> IResult<&[u8], Constant> {
    let (input, tag) = map_res(be_u8, ConstantTag::try_from)(input)?;
    let (input, constant) = match tag {
        ConstantTag::Class => map(be_u16, |name_index| Constant::Class { name_index })(input)?,
        ConstantTag::FieldRef => map(
            tuple((be_u16, be_u16)),
            |(class_index, name_and_type_index)| Constant::FieldRef {
                class_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::MethodRef => map(
            tuple((be_u16, be_u16)),
            |(class_index, name_and_type_index)| Constant::MethodRef {
                class_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::InterfaceMethodRef => map(
            tuple((be_u16, be_u16)),
            |(class_index, name_and_type_index)| Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::String => {
            map(be_u16, |string_index| Constant::String { string_index })(input)?
        }
        ConstantTag::Integer => map(take(4usize), |bytes: &[u8]| Constant::Integer {
            bytes: bytes
                .try_into()
                .expect("Expected 4 bytes for bytes of Constant Integer"),
        })(input)?,
        ConstantTag::Float => map(take(4usize), |bytes: &[u8]| Constant::Float {
            bytes: bytes
                .try_into()
                .expect("Expected 4 bytes for bytes of Constant Float"),
        })(input)?,
        ConstantTag::Long => map(
            tuple((take(4usize), take(4usize))),
            |(high_bytes, low_bytes): (&[u8], &[u8])| Constant::Long {
                high_bytes: high_bytes
                    .try_into()
                    .expect("Expected 4 bytes for high bytes of Constant Long"),
                low_bytes: low_bytes
                    .try_into()
                    .expect("Expected 4 bytes for low bytes of Constant Long"),
            },
        )(input)?,
        ConstantTag::Double => map(
            tuple((take(4usize), take(4usize))),
            |(high_bytes, low_bytes): (&[u8], &[u8])| Constant::Double {
                high_bytes: high_bytes
                    .try_into()
                    .expect("Expected 4 bytes for high bytes of Constant Double"),
                low_bytes: low_bytes
                    .try_into()
                    .expect("Expected 4 bytes for low bytes of Constant Double"),
            },
        )(input)?,
        ConstantTag::NameAndType => map(tuple((be_u16, be_u16)), |(name_index, type_index)| {
            Constant::NameAndType {
                name_index,
                type_index,
            }
        })(input)?,
        ConstantTag::Utf8 => {
            let (input, length) = be_u16(input)?;
            let (input, string) = map_res(take(length), cesu8::from_java_cesu8)(input)?;

            (
                input,
                Constant::Utf8 {
                    data: string.to_string(),
                },
            )
        }
        ConstantTag::MethodHandle => map(
            tuple((be_u8, be_u16)),
            |(reference_kind, reference_index)| Constant::MethodHandle {
                reference_kind,
                reference_index,
            },
        )(input)?,
        ConstantTag::MethodType => map(be_u16, |descriptor_index| Constant::MethodType {
            descriptor_index,
        })(input)?,
        ConstantTag::Dynamic => map(
            tuple((be_u16, be_u16)),
            |(bootstrap_method_attr_index, name_and_type_index)| Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::InvokeDynamic => map(
            tuple((be_u16, be_u16)),
            |(bootstrap_method_attr_index, name_and_type_index)| Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            },
        )(input)?,
        ConstantTag::Module => map(be_u16, |name_index| Constant::Module { name_index })(input)?,
        ConstantTag::Package => map(be_u16, |name_index| Constant::Package { name_index })(input)?,
    };

    Ok((input, constant))
}
