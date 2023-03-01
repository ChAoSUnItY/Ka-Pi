use std::cmp::max;
use std::fs::read;
use std::mem;
use std::rc::Rc;

use crate::asm::annotation::AnnotationVisitor;
use crate::asm::attribute::Attribute;
use crate::asm::constants::{ConstantDynamic, ConstantObject};
use crate::asm::field::FieldVisitor;
use crate::asm::method::MethodVisitor;
use crate::asm::module::ModuleVisitor;
use crate::asm::record::RecordVisitor;
use crate::asm::types::{Type, TypePath};
use crate::asm::{constants, opcodes, symbol, Handle};
use crate::error::{KapiError, KapiResult};

pub const SKIP_CODE: u8 = 1;
pub const SKIP_DEBUG: u8 = 2;
pub const SKIP_FRAMES: u8 = 4;
pub const EXPAND_FRAMES: u8 = 8;

pub(crate) const EXPAND_ASM_INSNS: u8 = 256u16 as u8;

pub trait ConstantValue {}

#[allow(unused_variables)]
pub trait ClassVisitor {
    fn visit(
        &mut self,
        version: u32,
        access: u32,
        class_name: String,
        signature: String,
        super_name: String,
        interfaces: &[String],
    ) {
    }
    fn visit_source(&mut self, source: String, debug: String) {}
    fn visit_module(
        &mut self,
        name: String,
        access: u32,
        version: String,
    ) -> Option<Box<dyn ModuleVisitor>> {
        None
    }
    fn visit_nest_host(&mut self, nest_host: String) {}
    fn visit_outer_class(&mut self, owner: String, name: String, descriptor: String) {}
    fn visit_annotation(
        &mut self,
        descriptor: String,
        visible: bool,
    ) -> Option<Box<dyn AnnotationVisitor>> {
        None
    }
    fn visit_type_annotation(
        &mut self,
        type_ref: i32,
        type_path: &TypePath,
        descriptor: String,
        visible: bool,
    ) -> Option<Box<dyn AnnotationVisitor>> {
        None
    }
    fn visit_attribute(&mut self, attribute: Box<dyn Attribute>) {}
    fn visit_nest_member(&mut self, nest_member: String) {}
    fn visit_permitted_sub_class(&mut self, permitted_sub_class: String) {}
    fn visit_inner_class(
        &mut self,
        name: String,
        outer_name: String,
        inner_name: String,
        access: u32,
    ) {
    }
    fn visit_component_record(
        &mut self,
        name: String,
        descriptor: String,
        signature: String,
    ) -> Option<Box<dyn RecordVisitor>> {
        None
    }
    fn visit_method(
        &mut self,
        name: String,
        descriptor: String,
        signature: String,
        exceptions: &[String],
    ) -> Option<Box<dyn MethodVisitor>> {
        None
    }
    fn visit_field(
        &mut self,
        name: String,
        descriptor: String,
        signature: String,
        value: Box<dyn ConstantValue>,
    ) -> Option<Box<dyn FieldVisitor>> {
        None
    }
    fn visit_end(self)
    where
        Self: Sized,
    {
    }
}

pub trait ClassReader {
    // Internal field accessors

    fn bytes(&self) -> &Vec<u8>;
    fn header(&self) -> usize;
    fn cp_info_offsets(&self) -> &Vec<usize>;
    fn constant_utf8_values(&self) -> &Vec<Rc<String>>;
    fn constant_dynamic_values(&self) -> &Vec<Rc<ConstantDynamic>>;
    fn bootstrap_method_offsets(&self) -> &Vec<usize>;

    // JVM bytecode class file accessors

    fn access(&self) -> usize;
    fn class_name(&mut self) -> KapiResult<Rc<String>>;
    fn super_name(&mut self) -> KapiResult<Rc<String>>;
    fn interfaces(&mut self) -> KapiResult<Vec<Rc<String>>>;

    // Internal field accessors (based on default impl)
    // These functions might remove after full implementation of ClassReader is done.

    fn cp_info_count(&self) -> usize;
    fn max_string_len(&self) -> usize;

    // Internal field mutators

    fn put_utf8(&mut self, utf8_string: Rc<String>, entry_index: usize);
    fn put_constant_dynamic(&mut self, constant_dynamic: Rc<ConstantDynamic>, entry_index: usize);

    // Internal utility functions
    // These functions are not overrideable due to the specialized nature of JVM bytecode spec.

    fn first_attr_offset(&self) -> usize {
        let mut current_offset = self.header() + 8 + self.read_u16(self.header() + 6) as usize * 2;

        // This loop skips both fields and methods on 1st iteration and 2nd iteration
        for _ in 0..2 {
            let member_count = self.read_u16(current_offset) as usize;
            current_offset += 2;

            for _ in 0..member_count {
                let attr_count = self.read_u16(current_offset + 6) as usize;
                current_offset += 8;

                for _ in 0..attr_count {
                    current_offset += 6 + self.read_u32(current_offset + 2) as usize;
                }
            }
        }

        current_offset + 2
    }

    fn read_bootstrap_methods_attr(&mut self) -> KapiResult<Vec<usize>> {
        let mut current_attr_offset = self.first_attr_offset();

        for _ in 0..self.read_u16(current_attr_offset - 2) {
            let attr_name = self.read_utf8(current_attr_offset)?;
            let attr_len = self.read_u32(current_attr_offset + 2) as usize;
            current_attr_offset += 6;
            if constants::BOOTSTRAP_METHODS == &*attr_name {
                let bootstrap_methods_len = self.read_u16(current_attr_offset) as usize;
                let mut result = Vec::with_capacity(bootstrap_methods_len);
                for _ in 0..bootstrap_methods_len {
                    result.push(current_attr_offset);
                    current_attr_offset += 4 + self.read_u16(current_attr_offset + 2) as usize * 2;
                }
                return Ok(result);
            }
            current_attr_offset += attr_len;
        }

        Err(KapiError::ClassParseError(format!(
            "Expected at least 1 bootstrap method attribute but got nothing"
        )))
    }

    fn slice_rev<const SIZE: usize>(&self, offset: usize) -> [u8; SIZE] {
        let mut bytes = [0u8; SIZE];
        bytes.clone_from((&self.bytes()[offset..offset + SIZE]).try_into().unwrap());
        bytes.reverse();
        bytes
    }

    #[inline]
    fn read_u8(&self, offset: usize) -> u8 {
        self.bytes()[offset]
    }

    #[inline]
    fn read_i16(&self, offset: usize) -> i16 {
        i16::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_u16(&self, offset: usize) -> u16 {
        u16::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_u32(&self, offset: usize) -> u32 {
        u32::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_i32(&self, offset: usize) -> i32 {
        i32::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_i64(&self, offset: usize) -> i64 {
        i64::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_f32(&self, offset: usize) -> f32 {
        f32::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_f64(&self, offset: usize) -> f64 {
        f64::from_ne_bytes(self.slice_rev(offset))
    }

    #[inline]
    fn read_utf8(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        let entry_index = self.read_u16(offset) as usize;

        if entry_index == 0 || offset == 0 {
            Err(KapiError::StateError("Constant pool entry index for UTF8 String is invalid while index or offset should larger than 0"))
        } else {
            self.read_utf8_from_cache(entry_index)
        }
    }

    #[inline]
    fn read_utf8_from_cache(&mut self, entry_index: usize) -> KapiResult<Rc<String>> {
        if let Some(entry) = self.constant_utf8_values().get(entry_index) {
            Ok(entry.clone())
        } else {
            let info_offset = self.cp_info_offsets()[entry_index];
            let len = self.read_u16(info_offset) as usize;
            let string = Rc::new(self.read_utf8_from_bytes(info_offset + 2, len)?);

            self.put_utf8(string.clone(), entry_index);

            Ok(string)
        }
    }

    #[inline]
    fn read_utf8_from_bytes(&self, data_offset: usize, len: usize) -> KapiResult<String> {
        let utf8_bytes = &self.bytes()[data_offset..data_offset + len];

        Ok(String::from_utf8_lossy(utf8_bytes).into())
    }

    #[inline]
    fn read_stringish(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_utf8(self.cp_info_offsets()[self.read_u16(offset) as usize])
    }

    #[inline]
    fn read_class(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_stringish(offset)
    }

    #[inline]
    fn read_module(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_stringish(offset)
    }

    #[inline]
    fn read_package(&mut self, offset: usize) -> KapiResult<Rc<String>> {
        self.read_stringish(offset)
    }

    fn read_constant_dynamic(&mut self, entry_index: usize) -> KapiResult<Rc<ConstantDynamic>> {
        let constant_dynamic = self.constant_dynamic_values().get(entry_index);

        if let Some(constant_dynamic) = constant_dynamic {
            Ok(constant_dynamic.clone())
        } else {
            let info_offset = self.cp_info_offsets()[entry_index];
            let name_and_type_info_offset =
                self.cp_info_offsets()[self.read_u16(info_offset + 2) as usize];
            let name = self.read_utf8(name_and_type_info_offset)?;
            let descriptor = self.read_utf8(name_and_type_info_offset + 2)?;
            let mut bootstrap_method_offset =
                self.bootstrap_method_offsets()[self.read_u16(info_offset) as usize];
            let handler_entry_index = self.read_u16(bootstrap_method_offset) as usize;
            let handler = self.read_const(handler_entry_index)?;
            let handler = handler
                .as_any()
                .downcast_ref::<Handle>()
                .ok_or(KapiError::ArgError(format!("Expected handler constant dynamic but got incorrect constant dynamic info at constant pool index {}", handler_entry_index)))?;
            let mut bootstrap_method_arguments =
                Vec::with_capacity(self.read_u16(bootstrap_method_offset + 2) as usize);
            bootstrap_method_offset += 4;

            for _ in 0..bootstrap_method_arguments.capacity() {
                bootstrap_method_arguments.push(self.read_const(bootstrap_method_offset)?);
                bootstrap_method_offset += 2;
            }

            let constant_dynamic = Rc::new(ConstantDynamic::new(
                name.to_string(),
                descriptor.to_string(),
                handler.to_owned(),
                bootstrap_method_arguments,
            ));

            self.put_constant_dynamic(constant_dynamic.clone(), entry_index);

            Ok(constant_dynamic)
        }
    }

    fn read_const(&mut self, entry_index: usize) -> KapiResult<Box<dyn ConstantObject>> {
        let info_offset = self.cp_info_offsets()[entry_index];

        match self.bytes()[info_offset - 1] {
            symbol::CONSTANT_INTEGER_TAG => Ok(Box::new(self.read_i32(info_offset))),
            symbol::CONSTANT_LONG_TAG => Ok(Box::new(self.read_i64(info_offset))),
            symbol::CONSTANT_FLOAT_TAG => Ok(Box::new(self.read_f32(info_offset))),
            symbol::CONSTANT_DOUBLE_TAG => Ok(Box::new(self.read_f64(info_offset))),
            symbol::CONSTANT_CLASS_TAG => Ok(Box::new(Type::object_type_from_string(
                self.read_utf8(info_offset)?.to_string(),
            ))),
            symbol::CONSTANT_STRING_TAG => Ok(Box::new(self.read_utf8(info_offset)?)),
            symbol::CONSTANT_METHOD_TYPE_TAG => Ok(Box::new(Type::method_type_from_string(
                self.read_utf8(info_offset)?.to_string(),
            ))),
            symbol::CONSTANT_METHOD_HANDLE_TAG => {
                let reference_kind = self.read_u8(info_offset);
                let reference_info_offset =
                    self.cp_info_offsets()[self.read_u16(info_offset + 1) as usize];
                let name_and_type_info_offset =
                    self.cp_info_offsets()[self.read_u16(info_offset + 2) as usize];
                let owner = self.read_class(reference_info_offset)?;
                let descriptor = self.read_class(name_and_type_info_offset)?;
                let is_interface = self.bytes()[reference_info_offset - 1]
                    == symbol::CONSTANT_INTERFACE_METHODREF_TAG;

                Ok(Box::new(Handle::new(
                    reference_kind,
                    owner.to_string(),
                    descriptor.to_string(),
                    is_interface.to_string(),
                )))
            }
            symbol::CONSTANT_DYNAMIC_TAG => Ok(Box::from(self.read_constant_dynamic(info_offset)?)),
            _ => Err(KapiError::ArgError(format!(
                "Illegal constant object at constant pool index {}",
                entry_index
            ))),
        }
    }
}

pub trait Writer {}

#[derive(Debug, Default)]
pub struct ClassReaderImpl {
    header: usize,
    class_file_buffer: Vec<u8>,
    cp_info_offsets: Vec<usize>,
    constant_utf8_values: Vec<Rc<String>>,
    constant_dynamic_values: Vec<Rc<ConstantDynamic>>,
    bootstrap_method_offsets: Vec<usize>,
    max_string_len: usize,
}

impl ClassReaderImpl {
    pub fn new_reader_from_raw(bytes: &[u8]) -> KapiResult<Self> {
        Self::new_reader(bytes, 0)
    }

    pub fn new_reader(class_file_buffer: &[u8], class_file_offset: usize) -> KapiResult<Self> {
        Self::init_reader(class_file_buffer, class_file_offset, true)
    }

    pub(crate) fn init_reader(
        class_file_buffer: &[u8],
        class_file_offset: usize,
        check_version: bool,
    ) -> KapiResult<Self> {
        let mut reader = Self::default();

        reader.class_file_buffer = class_file_buffer.to_vec();
        reader.parse_from_bytes(class_file_offset, check_version)?;

        Ok(reader)
    }

    fn parse_from_bytes(
        &mut self,
        class_file_offset: usize,
        check_version: bool,
    ) -> KapiResult<()> {
        if check_version {
            let version = self.read_u16(class_file_offset + 6) as u32;

            if version > opcodes::V21 {
                return Err(KapiError::ClassParseError(format!(
                    "Unsupported class file major version {}",
                    version
                )));
            }
        }

        let cp_count = self.read_u16(class_file_offset + 8) as usize;
        self.cp_info_offsets = Vec::with_capacity(cp_count);
        self.constant_utf8_values = Vec::with_capacity(cp_count);

        let mut current_cp_info_index = 1;
        let mut current_cp_info_offset = class_file_offset + 10;
        let mut current_max_string_len = 0;
        let mut has_bootstrap_method = false;
        let mut has_constant_dynamic = false;

        while current_cp_info_index < cp_count {
            self.cp_info_offsets.push(current_cp_info_offset + 1);
            let info_tag = self.class_file_buffer[current_cp_info_offset];
            let info_size: usize;

            match info_tag {
                symbol::CONSTANT_FIELDREF_TAG
                | symbol::CONSTANT_METHODREF_TAG
                | symbol::CONSTANT_INTERFACE_METHODREF_TAG
                | symbol::CONSTANT_INTEGER_TAG
                | symbol::CONSTANT_FLOAT_TAG
                | symbol::CONSTANT_NAME_AND_TYPE_TAG => info_size = 5,
                symbol::CONSTANT_DYNAMIC_TAG => {
                    info_size = 5;
                    has_bootstrap_method = true;
                    has_constant_dynamic = true;
                }
                symbol::CONSTANT_INVOKE_DYNAMIC_TAG => {
                    info_size = 5;
                    has_bootstrap_method = true;
                }
                symbol::CONSTANT_LONG_TAG | symbol::CONSTANT_DOUBLE_TAG => {
                    info_size = 9;
                    current_cp_info_index += 1;
                }
                symbol::CONSTANT_UTF8_TAG => {
                    info_size = 3 + self.read_u16(current_cp_info_offset + 1) as usize;
                    current_max_string_len = max(current_max_string_len, info_size);
                }
                symbol::CONSTANT_METHOD_HANDLE_TAG => info_size = 4,
                symbol::CONSTANT_STRING_TAG
                | symbol::CONSTANT_CLASS_TAG
                | symbol::CONSTANT_METHOD_TYPE_TAG
                | symbol::CONSTANT_PACKAGE_TAG
                | symbol::CONSTANT_MODULE_TAG => info_size = 3,
                _ => {
                    return Err(KapiError::ClassParseError(format!(
                        "Illegal constant pool tag {}",
                        info_tag
                    )))
                }
            }

            current_cp_info_offset += info_size;
        }

        self.max_string_len = current_max_string_len;
        self.header = current_cp_info_offset;

        if has_constant_dynamic {
            self.constant_dynamic_values = Vec::with_capacity(cp_count);
        }

        if has_bootstrap_method {
            self.bootstrap_method_offsets = self.read_bootstrap_methods_attr()?;
        }

        Ok(())
    }
}

impl ClassReader for ClassReaderImpl {
    #[inline]
    fn bytes(&self) -> &Vec<u8> {
        &self.class_file_buffer
    }

    #[inline]
    fn header(&self) -> usize {
        self.header
    }

    #[inline]
    fn cp_info_offsets(&self) -> &Vec<usize> {
        &self.cp_info_offsets
    }

    #[inline]
    fn constant_utf8_values(&self) -> &Vec<Rc<String>> {
        &self.constant_utf8_values
    }

    #[inline]
    fn constant_dynamic_values(&self) -> &Vec<Rc<ConstantDynamic>> {
        &self.constant_dynamic_values
    }

    #[inline]
    fn bootstrap_method_offsets(&self) -> &Vec<usize> {
        &self.bootstrap_method_offsets
    }

    #[inline]
    fn access(&self) -> usize {
        self.read_u16(self.header) as usize
    }

    #[inline]
    fn class_name(&mut self) -> KapiResult<Rc<String>> {
        self.read_class(self.header + 2)
    }

    #[inline]
    fn super_name(&mut self) -> KapiResult<Rc<String>> {
        self.read_class(self.header + 4)
    }

    fn interfaces(&mut self) -> KapiResult<Vec<Rc<String>>> {
        let mut current_offset = self.header + 6;
        let interfaces_len = self.read_u16(current_offset) as usize;
        let mut interfaces = Vec::with_capacity(interfaces_len);
        for _ in 0..interfaces_len {
            current_offset += 2;
            interfaces.push(self.read_class(current_offset)?);
        }
        Ok(interfaces)
    }

    #[inline]
    fn cp_info_count(&self) -> usize {
        self.cp_info_offsets.len()
    }

    #[inline]
    fn max_string_len(&self) -> usize {
        self.max_string_len
    }

    #[inline]
    fn put_utf8(&mut self, utf8_string: Rc<String>, _: usize) {
        self.constant_utf8_values.push(utf8_string)
    }

    #[inline]
    fn put_constant_dynamic(&mut self, constant_dynamic: Rc<ConstantDynamic>, _: usize) {
        self.constant_dynamic_values.push(constant_dynamic)
    }
}
