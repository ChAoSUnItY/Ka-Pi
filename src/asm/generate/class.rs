use std::cell::RefCell;
use std::rc::Rc;

use crate::asm::generate::byte_vec::{ByteVec, ByteVecImpl};
use crate::asm::generate::field::FieldWriter;
use crate::asm::generate::method::MethodWriter;
use crate::asm::generate::symbol::SymbolTable;
use crate::asm::node::access_flag::{
    AccessFlags, ClassAccessFlag, FieldAccessFlag, MethodAccessFlag,
};
use crate::asm::node::class::JavaVersion;
use crate::asm::node::constant;
use crate::asm::node::constant::{
    Class, Constant, Double, Dynamic, FieldRef, Float, Integer, InterfaceMethodRef, InvokeDynamic,
    Long, MethodHandle, MethodRef, MethodType, Module, NameAndType, Package, Utf8,
};
use crate::error::KapiResult;

pub struct ClassWriter {
    byte_vec: Rc<RefCell<ByteVecImpl>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    version: JavaVersion,
    access_flags: Vec<ClassAccessFlag>,
    this_class_index: u16,
    super_class_index: u16,
    interface_indices: Vec<u16>,
    fields: Vec<Rc<RefCell<ByteVecImpl>>>,
    methods: Vec<Rc<RefCell<ByteVecImpl>>>,
}

impl ClassWriter {
    pub fn new<F, I>(
        version: JavaVersion,
        access_flags: F,
        class_name: &str,
        super_class: &str,
        interfaces: I,
    ) -> Self
    where
        F: IntoIterator<Item = ClassAccessFlag>,
        I: IntoIterator<Item = String>,
    {
        let mut symbol_table = SymbolTable::default();

        let this_class_index = symbol_table.add_class(class_name);
        let super_class_index = symbol_table.add_class(super_class);
        let interface_indices = interfaces
            .into_iter()
            .map(|interface| symbol_table.add_class(&interface))
            .collect::<Vec<_>>();

        Self {
            byte_vec: Rc::new(RefCell::new(ByteVecImpl::new())),
            symbol_table: Rc::new(RefCell::new(symbol_table)),
            version,
            access_flags: access_flags.into_iter().collect(),
            this_class_index,
            super_class_index,
            interface_indices,
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }

    fn write_method<F>(
        &mut self,
        access_flags: F,
        name: &str,
        descriptor: &str,
    ) -> KapiResult<MethodWriter>
        where
            F: IntoIterator<Item = MethodAccessFlag>,
    {
        let method_byte_vec = Rc::new(RefCell::new(ByteVecImpl::with_capacity(8)));

        self.methods.push(method_byte_vec.clone());

        MethodWriter::new(
            &self.version,
            &method_byte_vec,
            &self.symbol_table,
            access_flags,
            name,
            descriptor,
        )
    }

    fn write_field<F>(
        &mut self,
        access_flags: F,
        name: &str,
        descriptor: &str,
    ) -> KapiResult<FieldWriter>
        where
            F: IntoIterator<Item = FieldAccessFlag>,
    {
        let field_byte_vec = Rc::new(RefCell::new(ByteVecImpl::with_capacity(8)));

        self.fields.push(field_byte_vec.clone());

        FieldWriter::new(
            &field_byte_vec,
            &self.symbol_table,
            access_flags,
            name,
            descriptor,
        )
    }

    fn visit_end(self) -> ByteVecImpl {
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
        } = self;

        let mut byte_vec = byte_vec.borrow_mut();
        let mut symbol_table = symbol_table.borrow_mut();

        byte_vec.put_u8s(&[0xCA, 0xFE, 0xBA, 0xBE]); // magic number
        byte_vec.put_u8s(&(version as u32).to_be_bytes()); // major version, minor version

        byte_vec.put_be(symbol_table.constants.len() as u16 + 1); // constant pool length
        for constant in &symbol_table.constants {
            byte_vec.put_be(constant.tag() as u8);

            match constant {
                Constant::Class(Class { name_index }) => {
                    byte_vec.put_be(*name_index);
                }
                Constant::FieldRef(FieldRef {
                                       class_index,
                                       name_and_type_index,
                                   }) => {
                    byte_vec.put_be(*class_index);
                    byte_vec.put_be(*name_and_type_index);
                }
                Constant::MethodRef(MethodRef {
                                        class_index,
                                        name_and_type_index,
                                    }) => {
                    byte_vec.put_be(*class_index);
                    byte_vec.put_be(*name_and_type_index);
                }
                Constant::InterfaceMethodRef(InterfaceMethodRef {
                                                 class_index,
                                                 name_and_type_index,
                                             }) => {
                    byte_vec.put_be(*class_index);
                    byte_vec.put_be(*name_and_type_index);
                }
                Constant::String(constant::String { string_index }) => {
                    byte_vec.put_be(*string_index)
                }
                Constant::Integer(Integer { bytes }) => {
                    byte_vec.extend_from_slice(bytes);
                }
                Constant::Float(Float { bytes }) => {
                    byte_vec.extend_from_slice(bytes);
                }
                Constant::Long(Long {
                                   high_bytes,
                                   low_bytes,
                               }) => {
                    byte_vec.extend_from_slice(high_bytes);
                    byte_vec.extend_from_slice(low_bytes);
                }
                Constant::Double(Double {
                                     high_bytes,
                                     low_bytes,
                                 }) => {
                    byte_vec.extend_from_slice(high_bytes);
                    byte_vec.extend_from_slice(low_bytes);
                }
                Constant::NameAndType(NameAndType {
                                          name_index,
                                          type_index,
                                      }) => {
                    byte_vec.put_be(*name_index);
                    byte_vec.put_be(*type_index);
                }
                Constant::Utf8(Utf8 { length: _, bytes }) => {
                    byte_vec.put_u8s(bytes);
                }
                Constant::MethodHandle(MethodHandle {
                                           reference_kind,
                                           reference_index,
                                       }) => {
                    byte_vec.put_be(*reference_kind);
                    byte_vec.put_be(*reference_index);
                }
                Constant::MethodType(MethodType {
                                         descriptor_index: descriptor,
                                     }) => {
                    byte_vec.put_be(*descriptor);
                }
                Constant::Dynamic(Dynamic {
                                      bootstrap_method_attr_index,
                                      name_and_type_index,
                                  }) => {
                    byte_vec.put_be(*bootstrap_method_attr_index);
                    byte_vec.put_be(*name_and_type_index);
                }
                Constant::InvokeDynamic(InvokeDynamic {
                                            bootstrap_method_attr_index,
                                            name_and_type_index,
                                        }) => {
                    byte_vec.put_be(*bootstrap_method_attr_index);
                    byte_vec.put_be(*name_and_type_index);
                }
                Constant::Module(Module { name_index }) => {
                    byte_vec.put_be(*name_index);
                }
                Constant::Package(Package { name_index }) => {
                    byte_vec.put_be(*name_index);
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
        for field_segment in fields {
            byte_vec.put_u8s(&field_segment.borrow()[..]);
        }

        byte_vec.put_be(methods.len() as u16); // methods length
        for method_segment in methods {
            byte_vec.put_u8s(&method_segment.borrow()[..]);
        }

        byte_vec.put_be(symbol_table.attributes.len() as u16); // attributes length
        // TODO: implement attributes

        byte_vec.clone()
    }
}
