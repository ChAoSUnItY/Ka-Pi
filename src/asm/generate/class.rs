use crate::asm::generate::bytes::ByteVecGen;
use crate::asm::generate::bytes::{ByteVec, ByteVecImpl};
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
    version: JavaVersion,
    access_flags: Vec<ClassAccessFlag>,
    this_class: String,
    super_class: String,
    interfaces: Vec<String>,
    field_writers: Vec<FieldWriter>,
    method_writers: Vec<MethodWriter>,
}

impl ClassWriter {
    pub fn new<F, I>(
        version: JavaVersion,
        access_flags: F,
        this_class: &str,
        super_class: &str,
        interfaces: I,
    ) -> Self
    where
        F: IntoIterator<Item = ClassAccessFlag>,
        I: IntoIterator<Item = String>,
    {
        Self {
            version,
            access_flags: access_flags.into_iter().collect(),
            this_class: this_class.to_string(),
            super_class: super_class.to_string(),
            interfaces: interfaces.into_iter().collect(),
            field_writers: vec![],
            method_writers: vec![],
        }
    }

    pub fn write_method<F>(
        &mut self,
        access_flags: F,
        name: &str,
        descriptor: &str,
        generation: impl FnOnce(MethodWriter) -> KapiResult<MethodWriter>,
    ) -> KapiResult<()>
    where
        F: IntoIterator<Item = MethodAccessFlag>,
    {
        let method_writer = generation(MethodWriter::new(
            &self.version,
            access_flags,
            name,
            descriptor,
        )?)?;

        self.method_writers.push(method_writer);

        Ok(())
    }

    pub fn append_method(&mut self, method_writer: MethodWriter) {
        self.method_writers.push(method_writer);
    }

    pub fn write_field<F>(
        &mut self,
        access_flags: F,
        name: &str,
        descriptor: &str,
        generation: impl FnOnce(FieldWriter) -> KapiResult<FieldWriter>,
    ) -> KapiResult<()>
    where
        F: IntoIterator<Item = FieldAccessFlag>,
    {
        let field_writer = generation(FieldWriter::new(access_flags, name, descriptor)?)?;

        self.field_writers.push(field_writer);

        Ok(())
    }

    pub fn append_field(&mut self, field_writer: FieldWriter) {
        self.field_writers.push(field_writer);
    }

    pub fn write_output(self) -> KapiResult<ByteVecImpl> {
        let mut byte_vec = ByteVecImpl::new();
        let mut symbol_table = SymbolTable::default();
        let mut field_byte_vec = ByteVecImpl::new();
        let mut method_byte_vec = ByteVecImpl::new();

        for field_writer in &self.field_writers {
            field_writer.put(&mut field_byte_vec, &mut symbol_table)?;
        }

        for method_writer in &self.method_writers {
            method_writer.put(&mut method_byte_vec, &mut symbol_table)?;
        }

        let this_class_index = symbol_table.add_class(&self.this_class);
        let super_class_index = symbol_table.add_class(&self.super_class);
        let interface_indices = self
            .interfaces
            .iter()
            .map(|interface| symbol_table.add_class(interface))
            .collect::<Vec<_>>();

        byte_vec.put_u8s(&[0xCA, 0xFE, 0xBA, 0xBE]); // magic number
        byte_vec.put_u8s(&(self.version as u32).to_be_bytes()); // major version, minor version

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
                Constant::Utf8(Utf8 { length, bytes }) => {
                    byte_vec.put_be(*length);
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

        byte_vec.put_be(self.access_flags.fold_flags()); // access flags
        byte_vec.put_be(this_class_index); // this class
        byte_vec.put_be(super_class_index); // super class
        byte_vec.put_be(interface_indices.len() as u16); // interfaces length

        for interface_index in interface_indices {
            byte_vec.put_be(interface_index);
        }

        byte_vec.put_be(self.field_writers.len() as u16); // fields length
        byte_vec.put_u8s(&field_byte_vec);

        byte_vec.put_be(self.method_writers.len() as u16); // methods length
        byte_vec.put_u8s(&method_byte_vec);

        byte_vec.put_be(symbol_table.attributes.len() as u16); // attributes length
                                                               // TODO: implement attributes

        Ok(byte_vec)
    }
}

#[cfg(test)]
mod test {
    use crate::asm::generate::class::ClassWriter;
    use crate::asm::generate::field::FieldWriter;
    use crate::asm::generate::method::MethodWriter;
    use crate::asm::node::access_flag::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag};
    use crate::asm::node::class::JavaVersion;
    use crate::error::KapiResult;

    #[test]
    fn test_class_writer_write_1_method_1_field() -> KapiResult<()> {
        let mut class_writer = ClassWriter::new(
            JavaVersion::V17,
            vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
            "Main",
            "java/lang/Object",
            vec![],
        );

        class_writer.write_field(
            vec![FieldAccessFlag::Public, FieldAccessFlag::Static],
            "field",
            "Z",
            |field| Ok(field),
        )?;
        class_writer.write_method(
            vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
            "method",
            "()Z",
            |method| Ok(method),
        )?;

        let bytes = class_writer.write_output()?;

        assert!(!bytes.is_empty());

        Ok(())
    }

    fn test_class_writer_append_1_method_1_field() -> KapiResult<()> {
        let mut class_writer = ClassWriter::new(
            JavaVersion::V17,
            vec![ClassAccessFlag::Super, ClassAccessFlag::Public],
            "Main",
            "java/lang/Object",
            vec![],
        );

        let field_writer = FieldWriter::new(
            vec![FieldAccessFlag::Public, FieldAccessFlag::Static],
            "field",
            "Z",
        )?;
        class_writer.append_field(field_writer);

        let method_writer = MethodWriter::new(
            &JavaVersion::V17,
            vec![MethodAccessFlag::Public, MethodAccessFlag::Static],
            "method",
            "()Z",
        )?;
        class_writer.append_method(method_writer);

        let bytes = class_writer.write_output()?;

        assert!(!bytes.is_empty());

        Ok(())
    }
}
