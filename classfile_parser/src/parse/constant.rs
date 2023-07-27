use std::cell::RefCell;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::node::constant;
use crate::node::constant::{
    Class, Constant, ConstantPool, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef,
    InvokeDynamic, Long, MethodHandle, MethodRef, MethodType, Module, NameAndType, Package, Utf8,
};
use crate::parse::error::{ParseError, ParseResult};

pub(super) fn constant_pool<R: Read>(input: &mut R) -> ParseResult<(u16, ConstantPool)> {
    let len = input.read_u16::<BigEndian>()?;
    let mut constant_pool = ConstantPool::with_capacity(len);
    let mut constant_counter = 0;

    while constant_counter < len - 1 {
        let constant = constant(input)?;

        if constant.occupies_2_slots() {
            constant_counter += 2;
        } else {
            constant_counter += 1;
        }

        constant_pool.add(constant);
    }

    Ok((len, constant_pool))
}

fn constant<R: Read>(input: &mut R) -> ParseResult<Constant> {
    let tag = input.read_u8()?;

    let constant = match tag {
        1 => {
            let length = input.read_u16::<BigEndian>()?;
            let mut bytes = vec![0; length as usize];

            input.read_exact(&mut bytes)?;

            Constant::Utf8(Utf8 {
                length,
                bytes,
                string: RefCell::new(None),
            })
        }
        3 => {
            let mut bytes = [0; 4];

            input.read_exact(&mut bytes)?;

            Constant::Integer(Integer { bytes })
        }
        4 => {
            let mut bytes = [0; 4];

            input.read_exact(&mut bytes)?;

            Constant::Float(Float { bytes })
        }
        5 => {
            let mut high_bytes = [0; 4];
            let mut low_bytes = [0; 4];

            input.read_exact(&mut high_bytes)?;
            input.read_exact(&mut low_bytes)?;

            Constant::Long(Long {
                high_bytes,
                low_bytes,
            })
        }
        6 => {
            let mut high_bytes = [0; 4];
            let mut low_bytes = [0; 4];

            input.read_exact(&mut high_bytes)?;
            input.read_exact(&mut low_bytes)?;

            Constant::Double(Double {
                high_bytes,
                low_bytes,
            })
        }
        7 => {
            let name_index = input.read_u16::<BigEndian>()?;

            Constant::Class(Class { name_index })
        }
        8 => {
            let string_index = input.read_u16::<BigEndian>()?;

            Constant::String(constant::String { string_index })
        }
        9 => {
            let class_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::FieldRef(FieldRef {
                class_index,
                name_and_type_index,
            })
        }
        10 => {
            let class_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::MethodRef(MethodRef {
                class_index,
                name_and_type_index,
            })
        }
        11 => {
            let class_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::InterfaceMethodRef(InterfaceMethodRef {
                class_index,
                name_and_type_index,
            })
        }
        12 => {
            let name_index = input.read_u16::<BigEndian>()?;
            let type_index = input.read_u16::<BigEndian>()?;

            Constant::NameAndType(NameAndType {
                name_index,
                type_index,
            })
        }
        15 => {
            let reference_kind = input.read_u8()?;
            let reference_index = input.read_u16::<BigEndian>()?;

            Constant::MethodHandle(MethodHandle {
                reference_kind,
                reference_index,
            })
        }
        16 => {
            let descriptor_index = input.read_u16::<BigEndian>()?;

            Constant::MethodType(MethodType { descriptor_index })
        }
        17 => {
            let bootstrap_method_attr_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::Dynamic(Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            })
        }
        18 => {
            let bootstrap_method_attr_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::InvokeDynamic(InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            })
        }
        19 => {
            let name_index = input.read_u16::<BigEndian>()?;

            Constant::Module(Module { name_index })
        }
        20 => {
            let name_index = input.read_u16::<BigEndian>()?;

            Constant::Package(Package { name_index })
        }
        _ => {
            return Err(ParseError::MatchOutOfBoundUsize(
                "constant tag",
                vec!["1..=20"],
                tag as usize,
            ))
        }
    };

    Ok(constant)
}
