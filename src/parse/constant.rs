use crate::node::constant;
use crate::node::constant::{
    Class, Constant, ConstantPool, ConstantTag, Double, Dynamic, FieldRef, Float, Integer,
    InterfaceMethodRef, InvokeDynamic, Long, MethodHandle, MethodRef, MethodType, Module,
    NameAndType, Package, Utf8,
};
use crate::parse::error::{ParseError, ParseResult};
use byteorder::{BigEndian, ReadBytesExt};
use std::cell::RefCell;
use std::io::Read;

pub(super) fn constant_pool<R: Read>(input: &mut R) -> ParseResult<(u16, ConstantPool)> {
    let len = input.read_u16::<BigEndian>()?;
    let mut constant_pool = ConstantPool::default();
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
    let raw_tag = input.read_u8()?;
    let tag = match ConstantTag::try_from(raw_tag) {
        Ok(tag) => tag,
        Err(_) => {
            return Err(ParseError::MatchOutOfBoundUsize(
                "constant tag",
                vec!["1..=20"],
                raw_tag as usize,
            ))
        }
    };

    let constant = match tag {
        ConstantTag::Utf8 => {
            let length = input.read_u16::<BigEndian>()?;
            let mut bytes = vec![0; length as usize];

            input.read_exact(&mut bytes)?;

            Constant::Utf8(Utf8 {
                length,
                bytes,
                string: RefCell::new(None),
            })
        }
        ConstantTag::Integer => {
            let mut bytes = [0; 4];

            input.read_exact(&mut bytes)?;

            Constant::Integer(Integer { bytes })
        }
        ConstantTag::Float => {
            let mut bytes = [0; 4];

            input.read_exact(&mut bytes)?;

            Constant::Float(Float { bytes })
        }
        ConstantTag::Long => {
            let mut high_bytes = [0; 4];
            let mut low_bytes = [0; 4];

            input.read_exact(&mut high_bytes)?;
            input.read_exact(&mut low_bytes)?;

            Constant::Long(Long {
                high_bytes,
                low_bytes,
            })
        }
        ConstantTag::Double => {
            let mut high_bytes = [0; 4];
            let mut low_bytes = [0; 4];

            input.read_exact(&mut high_bytes)?;
            input.read_exact(&mut low_bytes)?;

            Constant::Double(Double {
                high_bytes,
                low_bytes,
            })
        }
        ConstantTag::Class => {
            let name_index = input.read_u16::<BigEndian>()?;

            Constant::Class(Class { name_index })
        }
        ConstantTag::String => {
            let string_index = input.read_u16::<BigEndian>()?;

            Constant::String(constant::String { string_index })
        }
        ConstantTag::FieldRef => {
            let class_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::FieldRef(FieldRef {
                class_index,
                name_and_type_index,
            })
        }
        ConstantTag::MethodRef => {
            let class_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::MethodRef(MethodRef {
                class_index,
                name_and_type_index,
            })
        }
        ConstantTag::InterfaceMethodRef => {
            let class_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::InterfaceMethodRef(InterfaceMethodRef {
                class_index,
                name_and_type_index,
            })
        }
        ConstantTag::NameAndType => {
            let name_index = input.read_u16::<BigEndian>()?;
            let type_index = input.read_u16::<BigEndian>()?;

            Constant::NameAndType(NameAndType {
                name_index,
                type_index,
            })
        }
        ConstantTag::MethodHandle => {
            let reference_kind = input.read_u8()?;
            let reference_index = input.read_u16::<BigEndian>()?;

            Constant::MethodHandle(MethodHandle {
                reference_kind,
                reference_index,
            })
        }
        ConstantTag::MethodType => {
            let descriptor_index = input.read_u16::<BigEndian>()?;

            Constant::MethodType(MethodType { descriptor_index })
        }
        ConstantTag::Dynamic => {
            let bootstrap_method_attr_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::Dynamic(Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            })
        }
        ConstantTag::InvokeDynamic => {
            let bootstrap_method_attr_index = input.read_u16::<BigEndian>()?;
            let name_and_type_index = input.read_u16::<BigEndian>()?;

            Constant::InvokeDynamic(InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            })
        }
        ConstantTag::Module => {
            let name_index = input.read_u16::<BigEndian>()?;

            Constant::Module(Module { name_index })
        }
        ConstantTag::Package => {
            let name_index = input.read_u16::<BigEndian>()?;

            Constant::Package(Package { name_index })
        }
    };

    Ok(constant)
}
