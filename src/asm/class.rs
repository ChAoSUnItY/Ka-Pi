use std::marker::PhantomData;

use crate::asm::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::opcodes::{AccessFlag, ClassAccessFlag, JavaVersion};
use crate::asm::symbol::{Constant, SymbolTable};

pub trait ClassVisitor {
    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub struct ClassWriter<'output: 'writer, 'writer> {
    byte_vec: &'output mut ByteVecImpl,
    symbol_table: SymbolTable,
    version: JavaVersion,
    access_flags: &'writer [ClassAccessFlag],
    this_class_index: u16,
    super_class_index: u16,
    interface_indices: Vec<u16>,
    fields: Vec<()>,  // TODO
    methods: Vec<()>, // TODO
    _phantom: PhantomData<&'writer ()>,
}

impl<'output: 'writer, 'writer> ClassWriter<'output, 'writer> {
    pub fn new_class_writer(
        output_buffer: &'output mut ByteVecImpl,
        version: JavaVersion,
        access_flags: &'writer [ClassAccessFlag],
        class_name: &str,
        super_class: &str,
        interfaces: &'writer [String],
    ) -> Self {
        let mut symbol_table = SymbolTable::default();

        let this_class_index = symbol_table.add_class(class_name);
        let super_class_index = symbol_table.add_class(super_class);
        let interface_indices = interfaces
            .iter()
            .map(|interface| symbol_table.add_class(interface))
            .collect::<Vec<_>>();

        for interface in interfaces {
            symbol_table.add_class(interface);
        }

        Self {
            byte_vec: output_buffer,
            symbol_table,
            version,
            access_flags,
            this_class_index,
            super_class_index,
            interface_indices,
            fields: Vec::new(),
            methods: Vec::new(),
            _phantom: PhantomData::default(),
        }
    }
}

impl<'output: 'writer, 'writer> ClassVisitor for ClassWriter<'output, 'writer> {
    fn visit_end(self)
    where
        Self: Sized,
    {
        let Self {
            byte_vec,
            symbol_table,
            version,
            access_flags,
            this_class_index,
            super_class_index,
            interface_indices,
            fields,
            methods,
            _phantom,
        } = self;

        byte_vec.put_u8s(&[0xCA, 0xFE, 0xBA, 0xBE]); // magic number
        byte_vec.put_u8s(&(version as u32).to_be_bytes()); // major version, minor version

        byte_vec.put_be(symbol_table.constants.len() as u16 + 1); // constant pool length
        for constant in symbol_table.constants {
            byte_vec.put_be(constant.tag() as u8);

            match constant {
                Constant::Class { name_index } => {
                    byte_vec.put_be(name_index);
                }
                Constant::FieldRef {
                    class_index,
                    name_and_type_index,
                } => {
                    byte_vec.put_be(class_index);
                    byte_vec.put_be(name_and_type_index);
                }
                Constant::MethodRef {
                    class_index,
                    name_and_type_index,
                } => {
                    byte_vec.put_be(class_index);
                    byte_vec.put_be(name_and_type_index);
                }
                Constant::InterfaceMethodRef {
                    class_index,
                    name_and_type_index,
                } => {
                    byte_vec.put_be(class_index);
                    byte_vec.put_be(name_and_type_index);
                }
                Constant::String { string_index } => byte_vec.put_be(string_index),
                Constant::Integer { bytes } => {
                    byte_vec.extend_from_slice(&bytes);
                }
                Constant::Float { bytes } => {
                    byte_vec.extend_from_slice(&bytes);
                }
                Constant::Long {
                    high_bytes,
                    low_bytes,
                } => {
                    byte_vec.extend_from_slice(&high_bytes);
                    byte_vec.extend_from_slice(&low_bytes);
                }
                Constant::Double {
                    high_bytes,
                    low_bytes,
                } => {
                    byte_vec.extend_from_slice(&high_bytes);
                    byte_vec.extend_from_slice(&low_bytes);
                }
                Constant::NameAndType {
                    name_index,
                    type_index,
                } => {
                    byte_vec.put_be(name_index);
                    byte_vec.put_be(type_index);
                }
                Constant::Utf8 { data } => {
                    byte_vec.put_utf8(&data).unwrap();
                }
                Constant::MethodHandle {
                    reference_kind,
                    reference_index,
                } => {
                    byte_vec.put_be(reference_kind);
                    byte_vec.put_be(reference_index);
                }
                Constant::MethodType { descriptor } => {
                    byte_vec.put_be(descriptor);
                }
                Constant::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => {
                    byte_vec.put_be(bootstrap_method_attr_index);
                    byte_vec.put_be(name_and_type_index);
                }
                Constant::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => {
                    byte_vec.put_be(bootstrap_method_attr_index);
                    byte_vec.put_be(name_and_type_index);
                }
                Constant::Module { name_index } => {
                    byte_vec.put_be(name_index);
                }
                Constant::Package { name_index } => {
                    byte_vec.put_be(name_index);
                }
            }
        }

        byte_vec.put_be(access_flags.fold_flags()); // access flags
        byte_vec.put_be(this_class_index); // this class
        byte_vec.put_be(super_class_index); // super class
        byte_vec.put_be(interface_indices.len() as u16); // interfaces length

        for interface_index in interface_indices {
            byte_vec.put_be(interface_index);
        }

        byte_vec.put_be(fields.len() as u16); // fields length
                                              // TODO: implement fields

        byte_vec.put_be(methods.len() as u16); // methods length
                                               // TODO: implement methods

        byte_vec.put_be(symbol_table.attributes.len() as u16); // attributes length
                                                               // TODO: implement attributes
    }
}
